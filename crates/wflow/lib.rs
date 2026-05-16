use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    ops::Range,
    path::{Path, PathBuf},
};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

use miette::{Diagnostic, IntoDiagnostic, NamedSource, Result, SourceSpan};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tiktoken_rs::{CoreBPE, cl100k_base_singleton, o200k_base_singleton};
use toml::Spanned;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefsAction {
    Sync,
    Verify,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BenchEncoding {
    O200kBase,
    Cl100kBase,
}

impl BenchEncoding {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::O200kBase => "o200k_base",
            Self::Cl100kBase => "cl100k_base",
        }
    }

    fn tokenizer(self) -> &'static CoreBPE {
        match self {
            Self::O200kBase => o200k_base_singleton(),
            Self::Cl100kBase => cl100k_base_singleton(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BenchCount {
    pub encoding: String,
    pub files: Vec<BenchFileCount>,
    pub total: BenchTotal,
}

#[derive(Debug, Serialize)]
pub struct BenchFileCount {
    pub path: String,
    pub canonical_path: String,
    pub lines: usize,
    pub bytes: usize,
    pub tokens: usize,
}

#[derive(Debug, Default, Serialize)]
pub struct BenchTotal {
    pub lines: usize,
    pub bytes: usize,
    pub tokens: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ValidationMode {
    SyncPreflight,
    Verify,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SkillToml {
    references: RawReferenceMetadata,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawReferenceMetadata {
    shared: Vec<Spanned<String>>,
    owned: Vec<Spanned<String>>,
}

#[derive(Debug)]
struct ReferenceMetadata {
    shared: Vec<String>,
    owned: Vec<String>,
    spans: ReferenceSpans,
}

#[derive(Debug)]
struct ReferenceSpans {
    shared: Vec<ReferenceSpan>,
    owned: Vec<ReferenceSpan>,
}

#[derive(Debug)]
struct ReferenceSpan {
    value: String,
    span: Range<usize>,
}

impl From<RawReferenceMetadata> for ReferenceMetadata {
    fn from(raw: RawReferenceMetadata) -> Self {
        let shared_spans = reference_spans(&raw.shared);
        let owned_spans = reference_spans(&raw.owned);
        Self {
            shared: raw.shared.into_iter().map(Spanned::into_inner).collect(),
            owned: raw.owned.into_iter().map(Spanned::into_inner).collect(),
            spans: ReferenceSpans {
                shared: shared_spans,
                owned: owned_spans,
            },
        }
    }
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

#[derive(Debug)]
struct SkillMetadata {
    path: PathBuf,
    source: String,
    references: ReferenceMetadata,
}

#[derive(Debug, Diagnostic, Error)]
#[error("reference validation failed:\n{summary}")]
struct ValidationFailed {
    summary: String,
    #[related]
    related: Vec<ValidationIssue>,
}

#[derive(Debug, Diagnostic, Error)]
#[error("{message}")]
struct ValidationIssue {
    message: String,
    #[source_code]
    source_code: NamedSource<String>,
    #[label("relevant metadata")]
    span: SourceSpan,
}

#[derive(Debug, Default)]
struct ValidationErrors {
    messages: Vec<String>,
    related: Vec<ValidationIssue>,
}

impl ValidationErrors {
    fn push(&mut self, message: String) {
        self.messages.push(message);
    }

    fn push_spanned(&mut self, message: String, path: &Path, source: &str, span: Range<usize>) {
        self.messages.push(message.clone());
        self.related.push(ValidationIssue {
            message,
            source_code: NamedSource::new(display(path), source.to_owned()),
            span: (span.start, span.end.saturating_sub(span.start)).into(),
        });
    }

    const fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    fn into_validation_failed(self) -> ValidationFailed {
        ValidationFailed {
            summary: self
                .messages
                .into_iter()
                .map(|error| format!("- {error}"))
                .collect::<Vec<_>>()
                .join("\n"),
            related: self.related,
        }
    }
}

#[derive(Debug, Diagnostic, Error)]
#[error("benchmark failed:\n{summary}")]
struct BenchmarkFailed {
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

/// Count line, byte, and token footprint for explicit files.
///
/// # Errors
///
/// Returns a diagnostic error when inputs are missing, are directories, are not
/// UTF-8 text, or cannot be read.
pub fn count_bench(root: &Path, encoding: BenchEncoding, paths: &[PathBuf]) -> Result<BenchCount> {
    let root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let tokenizer = encoding.tokenizer();
    let mut seen = BTreeSet::new();
    let mut files = Vec::new();
    let mut errors = Vec::new();

    if paths.is_empty() {
        errors.push("no benchmark files provided".to_owned());
    }

    for path in paths {
        let input_path = if path.is_absolute() {
            path.clone()
        } else {
            root.join(path)
        };
        let display_path = display_bench_path(&root, path, &input_path);

        let canonical_path = match input_path.canonicalize() {
            Ok(canonical_path) => canonical_path,
            Err(error) => {
                errors.push(format!(
                    "cannot canonicalize benchmark path {display_path}: {error}"
                ));
                continue;
            }
        };

        if !seen.insert(canonical_path.clone()) {
            continue;
        }

        let metadata = match fs::metadata(&canonical_path) {
            Ok(metadata) => metadata,
            Err(error) => {
                errors.push(format!(
                    "cannot inspect benchmark path {display_path}: {error}"
                ));
                continue;
            }
        };

        if metadata.is_dir() {
            errors.push(format!("benchmark path is a directory: {display_path}"));
            continue;
        }

        if !metadata.is_file() {
            errors.push(format!("benchmark path is not a file: {display_path}"));
            continue;
        }

        let content = match fs::read_to_string(&canonical_path) {
            Ok(content) => content,
            Err(error) => {
                errors.push(format!(
                    "cannot read benchmark file {display_path}: {error}"
                ));
                continue;
            }
        };

        files.push(BenchFileCount {
            path: display_path,
            canonical_path: display(&canonical_path),
            lines: logical_line_count(&content),
            bytes: content.len(),
            tokens: tokenizer.encode_ordinary(&content).len(),
        });
    }

    if !errors.is_empty() {
        return Err(BenchmarkFailed {
            summary: errors
                .into_iter()
                .map(|error| format!("- {error}"))
                .collect::<Vec<_>>()
                .join("\n"),
        }
        .into());
    }

    let total = files.iter().fold(BenchTotal::default(), |mut total, file| {
        total.lines += file.lines;
        total.bytes += file.bytes;
        total.tokens += file.tokens;
        total
    });

    Ok(BenchCount {
        encoding: encoding.as_str().to_owned(),
        files,
        total,
    })
}

fn validate(
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
    let references_dir = dir.join("references");

    let skill_metadata = read_metadata(root, &name, &metadata_path, errors)?;
    validate_package_source_file(root, "skill instruction file", &skill_md_path, errors);

    validate_metadata_names(&name, &skill_metadata, shared, errors);
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

fn read_metadata(
    root: &Path,
    name: &str,
    metadata_path: &Path,
    errors: &mut ValidationErrors,
) -> Option<SkillMetadata> {
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
        Ok(parsed) => Some(SkillMetadata {
            path: metadata_path.to_path_buf(),
            source: content,
            references: parsed.references.into(),
        }),
        Err(error) => {
            let message = format!(
                "skill `{name}` has invalid metadata {}: {error}",
                display(metadata_path)
            );
            if let Some(span) = error.span() {
                errors.push_spanned(message, metadata_path, &content, span);
            } else {
                errors.push(message);
            }
            None
        }
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

        let mut errors = ValidationErrors::default();
        for (reference, path) in
            reference_files(&plan.root, &plan.name, &plan.references_dir, &mut errors)
        {
            if shared.contains_key(&reference) && !plan.metadata.shared.contains(&reference) {
                remove_generated_reference(&plan.root, &plan.name, &plan.references_dir, &path)?;
            }
        }

        if !errors.is_empty() {
            return Err(errors.into_validation_failed().into());
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
    errors: &mut ValidationErrors,
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

fn validate_mutation_target(
    root: &Path,
    skill_name: &str,
    path: &Path,
    errors: &mut ValidationErrors,
) {
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
    errors: &mut ValidationErrors,
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

fn display_bench_path(root: &Path, original: &Path, input_path: &Path) -> String {
    if original.is_absolute() {
        input_path
            .strip_prefix(root)
            .map_or_else(|_| display(original), display)
    } else {
        display(original)
    }
}

fn logical_line_count(content: &str) -> usize {
    if content.is_empty() {
        0
    } else {
        content.lines().count()
    }
}

fn reference_spans(values: &[Spanned<String>]) -> Vec<ReferenceSpan> {
    values
        .iter()
        .map(|value| ReferenceSpan {
            value: value.get_ref().clone(),
            span: value.span(),
        })
        .collect()
}
