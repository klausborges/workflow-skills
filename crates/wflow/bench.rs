use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use miette::{Diagnostic, Result};
use serde::Serialize;
use thiserror::Error;
use tiktoken_rs::{CoreBPE, cl100k_base_singleton, o200k_base_singleton};

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

#[derive(Debug, Diagnostic, Error)]
#[error("benchmark failed:\n{summary}")]
struct BenchmarkFailed {
    summary: String,
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

fn display(path: &Path) -> String {
    path.display().to_string()
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
