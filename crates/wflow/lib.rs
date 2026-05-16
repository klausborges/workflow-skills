use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

use miette::{Diagnostic, IntoDiagnostic, Result};
use serde::Deserialize;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefsAction {
    Sync,
    Verify,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ValidationMode {
    SyncPreflight,
    Verify,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SkillToml {
    references: ReferenceMetadata,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReferenceMetadata {
    shared: Vec<String>,
    owned: Vec<String>,
}

#[derive(Debug)]
struct SharedReference {
    path: PathBuf,
    content: String,
}

#[derive(Debug)]
struct SkillPlan {
    name: String,
    root: PathBuf,
    references_dir: PathBuf,
    metadata: ReferenceMetadata,
}

#[derive(Debug, Diagnostic, Error)]
#[error("reference validation failed:\n{summary}")]
struct ValidationFailed {
    summary: String,
}

/// Run a reference maintenance action against a workflow-skills repository root.
///
/// # Errors
///
/// Returns a diagnostic error when metadata is invalid, generated references
/// drift, or filesystem operations fail.
pub fn run_refs(root: &Path, action: RefsAction) -> Result<()> {
    let root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());

    match action {
        RefsAction::Verify => {
            validate(&root, ValidationMode::Verify)?;
        }
        RefsAction::Sync => {
            let (shared, plans) = validate(&root, ValidationMode::SyncPreflight)?;
            sync_references(&shared, &plans)?;
            validate(&root, ValidationMode::Verify)?;
        }
    }

    Ok(())
}

fn validate(
    root: &Path,
    mode: ValidationMode,
) -> std::result::Result<(BTreeMap<String, SharedReference>, Vec<SkillPlan>), ValidationFailed> {
    let mut errors = Vec::new();
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
        Err(ValidationFailed {
            summary: errors
                .into_iter()
                .map(|error| format!("- {error}"))
                .collect::<Vec<_>>()
                .join("\n"),
        })
    }
}

fn load_shared_references(
    root: &Path,
    errors: &mut Vec<String>,
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

fn installable_skill_dirs(root: &Path, errors: &mut Vec<String>) -> Vec<PathBuf> {
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
    errors: &mut Vec<String>,
) -> Option<SkillPlan> {
    let name = dir
        .file_name()
        .and_then(|file_name| file_name.to_str())
        .unwrap_or("<unknown>")
        .to_owned();
    let metadata_path = dir.join("skill.toml");
    let skill_md_path = dir.join("SKILL.md");
    let references_dir = dir.join("references");

    let metadata = read_metadata(root, &name, &metadata_path, errors)?;
    validate_package_source_file(root, "skill instruction file", &skill_md_path, errors);

    validate_metadata_names(&name, &metadata, shared, errors);
    validate_reference_files(
        root,
        &name,
        shared,
        &metadata,
        &references_dir,
        mode,
        errors,
    );

    Some(SkillPlan {
        name,
        root: root.to_path_buf(),
        references_dir,
        metadata,
    })
}

fn read_metadata(
    root: &Path,
    name: &str,
    metadata_path: &Path,
    errors: &mut Vec<String>,
) -> Option<ReferenceMetadata> {
    if !validate_package_source_file(root, "skill metadata file", metadata_path, errors) {
        errors.push(format!(
            "skill `{name}` is missing metadata at {}",
            display(metadata_path)
        ));
        return None;
    }

    let content = match fs::read_to_string(metadata_path) {
        Ok(content) => content,
        Err(error) => {
            errors.push(format!(
                "skill `{name}` cannot read {}: {error}",
                display(metadata_path)
            ));
            return None;
        }
    };

    match toml::from_str::<SkillToml>(&content) {
        Ok(parsed) => Some(parsed.references),
        Err(error) => {
            errors.push(format!(
                "skill `{name}` has invalid metadata {}: {error}",
                display(metadata_path)
            ));
            None
        }
    }
}

fn validate_metadata_names(
    skill_name: &str,
    metadata: &ReferenceMetadata,
    shared: &BTreeMap<String, SharedReference>,
    errors: &mut Vec<String>,
) {
    validate_sorted(skill_name, "shared", &metadata.shared, errors);
    validate_sorted(skill_name, "owned", &metadata.owned, errors);

    let mut seen = BTreeSet::new();
    for reference in metadata.shared.iter().chain(metadata.owned.iter()) {
        if !is_valid_reference_name(reference) {
            errors.push(format!(
                "skill `{skill_name}` has invalid reference name `{reference}`; use bare [a-z0-9-]+ names without `.md`"
            ));
        }

        if !seen.insert(reference) {
            errors.push(format!(
                "skill `{skill_name}` declares duplicate reference `{reference}`"
            ));
        }
    }

    for reference in &metadata.shared {
        if !shared.contains_key(reference) {
            errors.push(format!("skill `{skill_name}` declares shared reference `{reference}` but no shared source exists"));
        }
    }

    for reference in &metadata.owned {
        if shared.contains_key(reference) {
            errors.push(format!(
                "skill `{skill_name}` declares owned reference `{reference}`, but that name collides with a shared reference"
            ));
        }
    }
}

fn validate_sorted(skill_name: &str, field: &str, values: &[String], errors: &mut Vec<String>) {
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
    errors: &mut Vec<String>,
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

fn sync_references(shared: &BTreeMap<String, SharedReference>, plans: &[SkillPlan]) -> Result<()> {
    for plan in plans {
        if !declared_references(&plan.metadata).is_empty() {
            ensure_references_dir_for_sync(&plan.root, &plan.name, &plan.references_dir)
                .map_err(miette::Report::msg)?;
        }

        for reference in &plan.metadata.shared {
            let shared_reference = &shared[reference];
            let generated_path = plan.references_dir.join(format!("{reference}.md"));
            write_generated_reference(
                &plan.root,
                &plan.name,
                &plan.references_dir,
                &generated_path,
                &shared_reference.content,
            )?;
        }

        let mut errors = Vec::new();
        for (reference, path) in
            reference_files(&plan.root, &plan.name, &plan.references_dir, &mut errors)
        {
            if shared.contains_key(&reference) && !plan.metadata.shared.contains(&reference) {
                remove_generated_reference(&plan.root, &plan.name, &plan.references_dir, &path)?;
            }
        }

        if !errors.is_empty() {
            return Err(miette::Report::msg(errors.join("\n")));
        }
    }

    Ok(())
}

fn write_generated_reference(
    root: &Path,
    skill_name: &str,
    references_dir: &Path,
    path: &Path,
    content: &str,
) -> Result<()> {
    ensure_safe_references_dir(root, skill_name, references_dir).map_err(miette::Report::msg)?;
    ensure_safe_existing_target(root, skill_name, path).map_err(miette::Report::msg)?;

    if fs::read_to_string(path).is_ok_and(|current| current == content) {
        return Ok(());
    }

    let Some(parent) = path.parent() else {
        return Err(miette::Report::msg(format!(
            "skill `{skill_name}` reference path has no parent: {}",
            display_relative(root, path)
        )));
    };

    let temp_path = create_temp_reference(parent, path, content)?;
    if let Err(error) = fs::rename(&temp_path, path) {
        let _ = fs::remove_file(&temp_path);
        return Err(error).into_diagnostic();
    }

    Ok(())
}

fn remove_generated_reference(
    root: &Path,
    skill_name: &str,
    references_dir: &Path,
    path: &Path,
) -> Result<()> {
    ensure_safe_references_dir(root, skill_name, references_dir).map_err(miette::Report::msg)?;
    ensure_safe_existing_target(root, skill_name, path).map_err(miette::Report::msg)?;
    fs::remove_file(path).into_diagnostic()
}

fn create_temp_reference(parent: &Path, path: &Path, content: &str) -> Result<PathBuf> {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("reference.md");

    for attempt in 0..100 {
        let temp_path = parent.join(format!(
            ".{file_name}.tmp.{}.{}",
            std::process::id(),
            attempt
        ));
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)
        {
            Ok(mut file) => {
                if let Err(error) = file.write_all(content.as_bytes()) {
                    let _ = fs::remove_file(&temp_path);
                    return Err(error).into_diagnostic();
                }
                return Ok(temp_path);
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error).into_diagnostic(),
        }
    }

    Err(miette::Report::msg(format!(
        "could not create temporary reference file for {}",
        display(path)
    )))
}

fn reference_files(
    root: &Path,
    skill_name: &str,
    references_dir: &Path,
    errors: &mut Vec<String>,
) -> BTreeMap<String, PathBuf> {
    let mut files = BTreeMap::new();

    let metadata = match fs::symlink_metadata(references_dir) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return files,
        Err(error) => {
            errors.push(format!(
                "skill `{skill_name}` cannot inspect references directory {}: {error}",
                display_relative(root, references_dir)
            ));
            return files;
        }
    };

    if metadata.file_type().is_symlink() {
        errors.push(format!(
            "skill `{skill_name}` has symlinked references directory {}",
            display_relative(root, references_dir)
        ));
        return files;
    }

    if !metadata.is_dir() {
        errors.push(format!(
            "skill `{skill_name}` references path is not a directory: {}",
            display_relative(root, references_dir)
        ));
        return files;
    }

    let entries = match fs::read_dir(references_dir) {
        Ok(entries) => entries,
        Err(error) => {
            errors.push(format!(
                "skill `{skill_name}` cannot read references directory {}: {error}",
                display_relative(root, references_dir)
            ));
            return files;
        }
    };

    for entry in entries {
        let Ok(entry) = entry else {
            errors.push(format!(
                "skill `{skill_name}` cannot read an entry in {}",
                display_relative(root, references_dir)
            ));
            continue;
        };

        let path = entry.path();
        if is_temp_reference_artifact(&path) {
            errors.push(format!(
                "skill `{skill_name}` has temporary reference artifact {}",
                display_relative(root, &path)
            ));
            continue;
        }

        if path.extension().and_then(|extension| extension.to_str()) != Some("md") {
            continue;
        }

        match fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                errors.push(format!(
                    "skill `{skill_name}` has symlinked reference file {}",
                    display_relative(root, &path)
                ));
                continue;
            }
            Ok(metadata) if has_multiple_links(&metadata) => {
                errors.push(format!(
                    "skill `{skill_name}` has hardlinked reference file {}",
                    display_relative(root, &path)
                ));
                continue;
            }
            Ok(_) => {}
            Err(error) => {
                errors.push(format!(
                    "skill `{skill_name}` cannot inspect reference file {}: {error}",
                    display_relative(root, &path)
                ));
                continue;
            }
        }

        if let Some(name) = path.file_stem().and_then(|stem| stem.to_str()) {
            files.insert(name.to_owned(), path);
        } else {
            errors.push(format!(
                "skill `{skill_name}` reference has invalid filename: {}",
                display_relative(root, &path)
            ));
        }
    }

    files
}

fn is_temp_reference_artifact(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with('.') && name.contains(".tmp."))
}

fn validate_mutation_target(root: &Path, skill_name: &str, path: &Path, errors: &mut Vec<String>) {
    if let Ok(metadata) = fs::symlink_metadata(path) {
        if metadata.file_type().is_symlink() {
            errors.push(format!(
                "skill `{skill_name}` has symlinked reference file {}",
                display_relative(root, path)
            ));
            return;
        }

        if has_multiple_links(&metadata) {
            errors.push(format!(
                "skill `{skill_name}` has hardlinked reference file {}",
                display_relative(root, path)
            ));
            return;
        }
    }

    let Some(parent) = path.parent() else {
        errors.push(format!(
            "skill `{skill_name}` reference path has no parent: {}",
            display_relative(root, path)
        ));
        return;
    };

    let parent_to_check = if parent.exists() {
        parent
    } else {
        parent.parent().unwrap_or(parent)
    };

    match parent_to_check.canonicalize() {
        Ok(canonical_parent) if canonical_parent.starts_with(root) => {}
        Ok(canonical_parent) => errors.push(format!(
            "skill `{skill_name}` reference mutation target {} resolves outside repo root through {}",
            display_relative(root, path),
            display(&canonical_parent)
        )),
        Err(error) => errors.push(format!(
            "skill `{skill_name}` cannot canonicalize reference mutation target {}: {error}",
            display_relative(root, path)
        )),
    }
}

fn ensure_safe_existing_target(
    root: &Path,
    skill_name: &str,
    path: &Path,
) -> std::result::Result<(), String> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => {
            return Err(format!(
                "skill `{skill_name}` cannot inspect reference mutation target {}: {error}",
                display_relative(root, path)
            ));
        }
    };

    if metadata.file_type().is_symlink() {
        return Err(format!(
            "skill `{skill_name}` has symlinked reference file {}",
            display_relative(root, path)
        ));
    }

    if has_multiple_links(&metadata) {
        return Err(format!(
            "skill `{skill_name}` has hardlinked reference file {}",
            display_relative(root, path)
        ));
    }

    match path.canonicalize() {
        Ok(canonical_path) if canonical_path.starts_with(root) => Ok(()),
        Ok(canonical_path) => Err(format!(
            "skill `{skill_name}` reference mutation target {} resolves outside repo root through {}",
            display_relative(root, path),
            display(&canonical_path)
        )),
        Err(error) => Err(format!(
            "skill `{skill_name}` cannot canonicalize reference mutation target {}: {error}",
            display_relative(root, path)
        )),
    }
}

fn ensure_safe_references_dir(
    root: &Path,
    skill_name: &str,
    references_dir: &Path,
) -> std::result::Result<(), String> {
    let metadata = fs::symlink_metadata(references_dir).map_err(|error| {
        format!(
            "skill `{skill_name}` cannot inspect references directory {}: {error}",
            display_relative(root, references_dir)
        )
    })?;

    if metadata.file_type().is_symlink() {
        return Err(format!(
            "skill `{skill_name}` has symlinked references directory {}",
            display_relative(root, references_dir)
        ));
    }

    if !metadata.is_dir() {
        return Err(format!(
            "skill `{skill_name}` references path is not a directory: {}",
            display_relative(root, references_dir)
        ));
    }

    match references_dir.canonicalize() {
        Ok(canonical_dir) if canonical_dir.starts_with(root) => Ok(()),
        Ok(canonical_dir) => Err(format!(
            "skill `{skill_name}` references directory {} resolves outside repo root through {}",
            display_relative(root, references_dir),
            display(&canonical_dir)
        )),
        Err(error) => Err(format!(
            "skill `{skill_name}` cannot canonicalize references directory {}: {error}",
            display_relative(root, references_dir)
        )),
    }
}

fn ensure_references_dir_for_sync(
    root: &Path,
    skill_name: &str,
    references_dir: &Path,
) -> std::result::Result<(), String> {
    match fs::symlink_metadata(references_dir) {
        Ok(_) => ensure_safe_references_dir(root, skill_name, references_dir),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let Some(parent) = references_dir.parent() else {
                return Err(format!(
                    "skill `{skill_name}` references directory has no parent: {}",
                    display_relative(root, references_dir)
                ));
            };
            ensure_safe_skill_directory(root, skill_name, parent)?;
            match fs::create_dir(references_dir) {
                Ok(()) => ensure_safe_references_dir(root, skill_name, references_dir),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    ensure_safe_references_dir(root, skill_name, references_dir)
                }
                Err(error) => Err(format!(
                    "skill `{skill_name}` cannot create references directory {}: {error}",
                    display_relative(root, references_dir)
                )),
            }
        }
        Err(error) => Err(format!(
            "skill `{skill_name}` cannot inspect references directory {}: {error}",
            display_relative(root, references_dir)
        )),
    }
}

fn ensure_safe_skill_directory(
    root: &Path,
    skill_name: &str,
    skill_dir: &Path,
) -> std::result::Result<(), String> {
    let metadata = fs::symlink_metadata(skill_dir).map_err(|error| {
        format!(
            "skill `{skill_name}` cannot inspect skill directory {}: {error}",
            display_relative(root, skill_dir)
        )
    })?;

    if metadata.file_type().is_symlink() {
        return Err(format!(
            "skill `{skill_name}` has symlinked skill directory {}",
            display_relative(root, skill_dir)
        ));
    }

    if !metadata.is_dir() {
        return Err(format!(
            "skill `{skill_name}` path is not a directory: {}",
            display_relative(root, skill_dir)
        ));
    }

    match skill_dir.canonicalize() {
        Ok(canonical_dir) if canonical_dir.starts_with(root) => Ok(()),
        Ok(canonical_dir) => Err(format!(
            "skill `{skill_name}` directory {} resolves outside repo root through {}",
            display_relative(root, skill_dir),
            display(&canonical_dir)
        )),
        Err(error) => Err(format!(
            "skill `{skill_name}` cannot canonicalize skill directory {}: {error}",
            display_relative(root, skill_dir)
        )),
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

fn validate_package_source_file(
    root: &Path,
    description: &str,
    path: &Path,
    errors: &mut Vec<String>,
) -> bool {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return false,
        Err(error) => {
            errors.push(format!(
                "cannot inspect {description} {}: {error}",
                display_relative(root, path)
            ));
            return false;
        }
    };

    if metadata.file_type().is_symlink() {
        errors.push(format!(
            "symlinked {description}: {}",
            display_relative(root, path)
        ));
        return false;
    }

    if !metadata.is_file() {
        errors.push(format!(
            "{description} is not a file: {}",
            display_relative(root, path)
        ));
        return false;
    }

    match path.canonicalize() {
        Ok(canonical_path) if canonical_path.starts_with(root) => true,
        Ok(canonical_path) => {
            errors.push(format!(
                "{description} {} resolves outside repo root through {}",
                display_relative(root, path),
                display(&canonical_path)
            ));
            false
        }
        Err(error) => {
            errors.push(format!(
                "cannot canonicalize {description} {}: {error}",
                display_relative(root, path)
            ));
            false
        }
    }
}

#[cfg(unix)]
fn has_multiple_links(metadata: &fs::Metadata) -> bool {
    metadata.nlink() > 1
}

#[cfg(not(unix))]
fn has_multiple_links(_metadata: &fs::Metadata) -> bool {
    false
}

fn declared_references(metadata: &ReferenceMetadata) -> BTreeSet<String> {
    metadata
        .shared
        .iter()
        .chain(metadata.owned.iter())
        .cloned()
        .collect()
}

fn display(path: &Path) -> String {
    path.display().to_string()
}

fn display_relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .map_or_else(|_| display(path), display)
}
