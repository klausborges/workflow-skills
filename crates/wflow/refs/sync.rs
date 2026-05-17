use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use miette::{IntoDiagnostic, Result};

use super::{
    diagnostics::ValidationErrors,
    fs_safety::{
        display, display_relative, ensure_references_dir_for_sync, ensure_safe_existing_target,
        ensure_safe_references_dir, reference_files,
    },
    metadata::{SharedReference, SkillPlan, declared_references},
};

pub(super) fn sync_references(
    shared: &BTreeMap<String, SharedReference>,
    plans: &[SkillPlan],
) -> Result<()> {
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
