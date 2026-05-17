use std::path::Path;

use miette::Result;

mod diagnostics;
mod fs_safety;
mod metadata;
mod sync;
mod validate;

use sync::sync_references;
use validate::validate;

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
