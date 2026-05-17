use std::{
    collections::BTreeSet,
    ops::Range,
    path::{Path, PathBuf},
};

use serde::Deserialize;
use toml::Spanned;

use super::{
    diagnostics::ValidationErrors,
    fs_safety::{display, validate_package_source_file},
};

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
pub(super) struct ReferenceMetadata {
    pub(super) shared: Vec<String>,
    pub(super) owned: Vec<String>,
    pub(super) spans: ReferenceSpans,
}

#[derive(Debug)]
pub(super) struct ReferenceSpans {
    pub(super) shared: Vec<ReferenceSpan>,
    pub(super) owned: Vec<ReferenceSpan>,
}

#[derive(Debug)]
pub(super) struct ReferenceSpan {
    pub(super) value: String,
    pub(super) span: Range<usize>,
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
pub(super) struct SharedReference {
    pub(super) path: PathBuf,
    pub(super) content: String,
}

#[derive(Debug)]
pub(super) struct SkillPlan {
    pub(super) name: String,
    pub(super) root: PathBuf,
    pub(super) references_dir: PathBuf,
    pub(super) metadata: ReferenceMetadata,
}

#[derive(Debug)]
pub(super) struct SkillMetadata {
    pub(super) path: PathBuf,
    pub(super) source: String,
    pub(super) references: ReferenceMetadata,
}

pub(super) fn read_metadata(
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

    let content = match std::fs::read_to_string(metadata_path) {
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

pub(super) fn declared_references(metadata: &ReferenceMetadata) -> BTreeSet<String> {
    metadata
        .shared
        .iter()
        .chain(metadata.owned.iter())
        .cloned()
        .collect()
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
