use std::{ops::Range, path::Path};

use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

use super::fs_safety::display;

#[derive(Debug, Diagnostic, Error)]
#[error("reference validation failed:\n{summary}")]
pub(super) struct ValidationFailed {
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
pub(super) struct ValidationErrors {
    messages: Vec<String>,
    related: Vec<ValidationIssue>,
}

impl ValidationErrors {
    pub(super) fn push(&mut self, message: String) {
        self.messages.push(message);
    }

    pub(super) fn push_spanned(
        &mut self,
        message: String,
        path: &Path,
        source: &str,
        span: Range<usize>,
    ) {
        self.messages.push(message.clone());
        self.related.push(ValidationIssue {
            message,
            source_code: NamedSource::new(display(path), source.to_owned()),
            span: (span.start, span.end.saturating_sub(span.start)).into(),
        });
    }

    pub(super) const fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub(super) fn into_validation_failed(self) -> ValidationFailed {
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
