use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

use super::diagnostics::ValidationErrors;

pub(super) fn reference_files(
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

pub(super) fn validate_mutation_target(
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

pub(super) fn ensure_safe_existing_target(
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

pub(super) fn ensure_safe_references_dir(
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

pub(super) fn ensure_references_dir_for_sync(
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

pub(super) fn validate_package_source_file(
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

pub(super) fn display(path: &Path) -> String {
    path.display().to_string()
}

pub(super) fn display_relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .map_or_else(|_| display(path), display)
}

fn is_temp_reference_artifact(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with('.') && name.contains(".tmp."))
}

#[cfg(unix)]
fn has_multiple_links(metadata: &fs::Metadata) -> bool {
    metadata.nlink() > 1
}

#[cfg(not(unix))]
fn has_multiple_links(_metadata: &fs::Metadata) -> bool {
    false
}
