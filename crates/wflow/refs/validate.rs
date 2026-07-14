use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use super::{
    ValidationMode,
    diagnostics::{ValidationErrors, ValidationFailed},
    fs_safety::{
        display, display_relative, reference_files, validate_mutation_target,
        validate_package_source_file,
    },
    metadata::{
        ReferenceMetadata, SharedReference, SkillMetadata, SkillPlan, declared_references,
        read_metadata,
    },
};

pub(super) fn validate(
    root: &Path,
    mode: ValidationMode,
) -> std::result::Result<(BTreeMap<String, SharedReference>, Vec<SkillPlan>), ValidationFailed> {
    let mut errors = ValidationErrors::default();
    let shared = load_shared_references(root, &mut errors);
    let skill_dirs = installable_skill_dirs(root, &mut errors);
    let mut plans = Vec::new();

    for skill_dir in skill_dirs {
        if let Some(plan) = validate_skill(root, &shared, &skill_dir, mode, &mut errors) {
            plans.push(plan);
        }
    }

    if errors.is_empty() {
        Ok((shared, plans))
    } else {
        Err(errors.into_validation_failed())
    }
}

fn load_shared_references(
    root: &Path,
    errors: &mut ValidationErrors,
) -> BTreeMap<String, SharedReference> {
    let shared_dir = root.join("skills").join("_shared");
    let mut shared = BTreeMap::new();

    match fs::symlink_metadata(&shared_dir) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            errors.push(format!(
                "shared references directory is symlinked: {}",
                display_relative(root, &shared_dir)
            ));
            return shared;
        }
        Ok(metadata) if metadata.is_dir() => {}
        Ok(_) => {
            errors.push(format!(
                "shared references path is not a directory: {}",
                display_relative(root, &shared_dir)
            ));
            return shared;
        }
        Err(error) => {
            errors.push(format!(
                "cannot inspect shared references at {}: {error}",
                display_relative(root, &shared_dir)
            ));
            return shared;
        }
    }

    let entries = match fs::read_dir(&shared_dir) {
        Ok(entries) => entries,
        Err(error) => {
            errors.push(format!(
                "cannot read shared references at {}: {error}",
                display(&shared_dir)
            ));
            return shared;
        }
    };

    for entry in entries {
        let Ok(entry) = entry else {
            errors.push(format!("cannot read an entry in {}", display(&shared_dir)));
            continue;
        };

        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("md") {
            continue;
        }

        if !validate_package_source_file(root, "shared reference file", &path, errors) {
            continue;
        }

        let Some(name) = path.file_stem().and_then(|stem| stem.to_str()) else {
            errors.push(format!(
                "shared reference has invalid filename: {}",
                display(&path)
            ));
            continue;
        };

        match fs::read_to_string(&path) {
            Ok(content) => {
                shared.insert(
                    name.to_owned(),
                    SharedReference {
                        path: path.clone(),
                        content,
                    },
                );
            }
            Err(error) => errors.push(format!(
                "cannot read shared reference {}: {error}",
                display(&path)
            )),
        }
    }

    shared
}

fn installable_skill_dirs(root: &Path, errors: &mut ValidationErrors) -> Vec<PathBuf> {
    let skills_dir = root.join("skills");
    let entries = match fs::read_dir(&skills_dir) {
        Ok(entries) => entries,
        Err(error) => {
            errors.push(format!(
                "cannot read skills directory {}: {error}",
                display(&skills_dir)
            ));
            return Vec::new();
        }
    };

    let mut dirs = Vec::new();
    for entry in entries {
        let Ok(entry) = entry else {
            errors.push(format!("cannot read an entry in {}", display(&skills_dir)));
            continue;
        };

        let path = entry.path();
        if path.file_name().and_then(|name| name.to_str()) == Some("_shared") {
            continue;
        }

        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) => {
                errors.push(format!(
                    "cannot inspect skill directory {}: {error}",
                    display_relative(root, &path)
                ));
                continue;
            }
        };

        if metadata.file_type().is_symlink() {
            errors.push(format!(
                "installable skill directory is symlinked: {}",
                display_relative(root, &path)
            ));
            continue;
        }

        if !metadata.is_dir() {
            continue;
        }

        if path.join("SKILL.md").is_file() {
            dirs.push(path);
        }
    }

    dirs.sort();
    dirs
}

fn validate_skill(
    root: &Path,
    shared: &BTreeMap<String, SharedReference>,
    dir: &Path,
    mode: ValidationMode,
    errors: &mut ValidationErrors,
) -> Option<SkillPlan> {
    let name = dir
        .file_name()
        .and_then(|file_name| file_name.to_str())
        .unwrap_or("<unknown>")
        .to_owned();
    let metadata_path = dir.join("skill.toml");
    let skill_md_path = dir.join("SKILL.md");
    let interface_metadata_path = dir.join("agents").join("openai.yaml");
    let references_dir = dir.join("references");

    validate_package_source_file(root, "skill instruction file", &skill_md_path, errors);
    validate_interface_metadata(root, &name, &interface_metadata_path, errors);
    let skill_metadata = read_metadata(root, &name, &metadata_path, errors)?;

    validate_metadata_names(&name, &skill_metadata, shared, errors);
    validate_skill_reference_links(
        root,
        &name,
        &skill_md_path,
        &skill_metadata.references,
        errors,
    );
    validate_reference_files(
        root,
        &name,
        shared,
        &skill_metadata.references,
        &references_dir,
        mode,
        errors,
    );

    Some(SkillPlan {
        name,
        root: root.to_path_buf(),
        references_dir,
        metadata: skill_metadata.references,
    })
}

fn validate_interface_metadata(
    root: &Path,
    skill_name: &str,
    path: &Path,
    errors: &mut ValidationErrors,
) {
    if !validate_package_source_file(root, "OpenAI interface metadata file", path, errors) {
        errors.push(format!(
            "skill `{skill_name}` is missing OpenAI interface metadata at {}",
            display_relative(root, path)
        ));
        return;
    }

    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            errors.push(format!(
                "skill `{skill_name}` cannot read OpenAI interface metadata {}: {error}",
                display_relative(root, path)
            ));
            return;
        }
    };

    let lines = content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>();
    let fields = ["display_name", "short_description", "default_prompt"];

    if lines.len() != fields.len() + 1 || lines.first() != Some(&"interface:") {
        errors.push(format!(
            "skill `{skill_name}` has invalid OpenAI interface metadata {}; expected `interface` with display_name, short_description, and default_prompt",
            display_relative(root, path)
        ));
        return;
    }

    for (line, field) in lines.iter().skip(1).zip(fields) {
        let prefix = format!("  {field}: ");
        let Some(value) = line.strip_prefix(&prefix) else {
            errors.push(format!(
                "skill `{skill_name}` has invalid OpenAI interface metadata {}; expected field `{field}`",
                display_relative(root, path)
            ));
            continue;
        };

        let valid =
            serde_json::from_str::<String>(value).is_ok_and(|parsed| !parsed.trim().is_empty());
        if !valid {
            errors.push(format!(
                "skill `{skill_name}` OpenAI interface field `{field}` must be a non-empty JSON-compatible quoted string in {}",
                display_relative(root, path)
            ));
        }
    }
}

fn validate_skill_reference_links(
    root: &Path,
    skill_name: &str,
    skill_md_path: &Path,
    metadata: &ReferenceMetadata,
    errors: &mut ValidationErrors,
) {
    let Ok(content) = fs::read_to_string(skill_md_path) else {
        return;
    };
    let declared = declared_references(metadata);
    let marker = "](references/";
    let mut remainder = content.as_str();

    while let Some(start) = remainder.find(marker) {
        let target_start = start + marker.len();
        let target_remainder = &remainder[target_start..];
        let Some(end) = target_remainder.find(')') else {
            errors.push(format!(
                "skill `{skill_name}` has an unterminated local reference link in {}",
                display_relative(root, skill_md_path)
            ));
            return;
        };

        let target = &target_remainder[..end];
        let Some(name) = target.strip_suffix(".md") else {
            errors.push(format!(
                "skill `{skill_name}` has invalid local reference link `references/{target}` in {}; use `references/<declared-name>.md`",
                display_relative(root, skill_md_path)
            ));
            remainder = &target_remainder[end + 1..];
            continue;
        };

        if !is_valid_reference_name(name) {
            errors.push(format!(
                "skill `{skill_name}` has invalid local reference link `references/{target}` in {}",
                display_relative(root, skill_md_path)
            ));
        } else if !declared.contains(name) {
            errors.push(format!(
                "skill `{skill_name}` links undeclared reference `references/{target}` in {}; add `{name}` to skill.toml or remove the link",
                display_relative(root, skill_md_path)
            ));
        }

        remainder = &target_remainder[end + 1..];
    }
}

fn validate_metadata_names(
    skill_name: &str,
    metadata: &SkillMetadata,
    shared: &BTreeMap<String, SharedReference>,
    errors: &mut ValidationErrors,
) {
    validate_sorted(skill_name, "shared", &metadata.references.shared, errors);
    validate_sorted(skill_name, "owned", &metadata.references.owned, errors);

    let mut seen = BTreeSet::new();
    for reference in metadata
        .references
        .spans
        .shared
        .iter()
        .chain(metadata.references.spans.owned.iter())
    {
        if !is_valid_reference_name(&reference.value) {
            let message = format!(
                "skill `{skill_name}` has invalid reference name `{}`; use bare [a-z0-9-]+ names without `.md`",
                reference.value
            );
            errors.push_spanned(
                message,
                &metadata.path,
                &metadata.source,
                reference.span.clone(),
            );
        }

        if !seen.insert(&reference.value) {
            errors.push(format!(
                "skill `{skill_name}` declares duplicate reference `{}`",
                reference.value
            ));
        }
    }

    for reference in &metadata.references.shared {
        if !shared.contains_key(reference) {
            errors.push(format!("skill `{skill_name}` declares shared reference `{reference}` but no shared source exists"));
        }
    }

    for reference in &metadata.references.owned {
        if shared.contains_key(reference) {
            errors.push(format!(
                "skill `{skill_name}` declares owned reference `{reference}`, but that name collides with a shared reference"
            ));
        }
    }
}

fn validate_sorted(
    skill_name: &str,
    field: &str,
    values: &[String],
    errors: &mut ValidationErrors,
) {
    let mut sorted = values.to_vec();
    sorted.sort();

    if values != sorted {
        errors.push(format!(
            "skill `{skill_name}` metadata field `references.{field}` is not sorted"
        ));
    }
}

fn validate_reference_files(
    root: &Path,
    skill_name: &str,
    shared: &BTreeMap<String, SharedReference>,
    metadata: &ReferenceMetadata,
    references_dir: &Path,
    mode: ValidationMode,
    errors: &mut ValidationErrors,
) {
    let declared_shared: BTreeSet<_> = metadata.shared.iter().cloned().collect();
    let declared_owned: BTreeSet<_> = metadata.owned.iter().cloned().collect();
    let existing = reference_files(root, skill_name, references_dir, errors);

    if mode == ValidationMode::Verify
        && !references_dir.is_dir()
        && !declared_references(metadata).is_empty()
    {
        errors.push(format!(
            "skill `{skill_name}` is missing references directory {}",
            display_relative(root, references_dir)
        ));
    }

    for (name, path) in &existing {
        if shared.contains_key(name) {
            if !declared_shared.contains(name) && mode == ValidationMode::Verify {
                errors.push(format!(
                    "skill `{skill_name}` has undeclared generated shared reference {}",
                    display_relative(root, path)
                ));
            }
        } else if !declared_owned.contains(name) {
            errors.push(format!(
                "skill `{skill_name}` has undeclared skill-owned reference {}",
                display_relative(root, path)
            ));
        }
    }

    for reference in &metadata.shared {
        if let Some(shared_reference) = shared.get(reference) {
            let generated_path = references_dir.join(format!("{reference}.md"));
            validate_mutation_target(root, skill_name, &generated_path, errors);
            match existing.get(reference) {
                Some(path) if mode == ValidationMode::Verify => match fs::read_to_string(path) {
                    Ok(content) if content == shared_reference.content => {}
                    Ok(_) => errors.push(format!(
                        "skill `{skill_name}` generated reference {} differs from shared source {}",
                        display_relative(root, path),
                        display_relative(root, &shared_reference.path)
                    )),
                    Err(error) => errors.push(format!(
                        "skill `{skill_name}` cannot read generated reference {}: {error}",
                        display_relative(root, path)
                    )),
                },
                None if mode == ValidationMode::Verify => errors.push(format!(
                    "skill `{skill_name}` is missing declared generated reference {}",
                    display_relative(root, &generated_path)
                )),
                _ => {}
            }
        }
    }

    for reference in &metadata.owned {
        let owned_path = references_dir.join(format!("{reference}.md"));
        validate_mutation_target(root, skill_name, &owned_path, errors);
        if !existing.contains_key(reference) {
            errors.push(format!(
                "skill `{skill_name}` is missing declared skill-owned reference {}",
                display_relative(root, &owned_path)
            ));
        }
    }
}

fn is_valid_reference_name(name: &str) -> bool {
    let has_markdown_extension = Path::new(name)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("md"));

    !name.is_empty()
        && !has_markdown_extension
        && !name.contains('/')
        && !name.contains('\\')
        && name.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
}
