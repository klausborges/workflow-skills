use std::{
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand, ValueEnum};
use miette::{Context, IntoDiagnostic, Result};
use wflow::{BenchEncoding, RefsAction, count_bench, run_refs};

#[derive(Debug, Parser)]
#[command(version, about = "workflow skills maintenance tooling")]
struct Cli {
    #[arg(long, global = true, default_value = ".")]
    root: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Refs {
        #[command(subcommand)]
        command: RefsCommand,
    },
    #[command(about = "Measure skill footprint from explicit file paths")]
    Bench {
        #[command(subcommand)]
        command: BenchCommand,
    },
}

#[derive(Debug, Subcommand)]
enum RefsCommand {
    Sync,
    Verify,
}

#[derive(Debug, Subcommand)]
enum BenchCommand {
    #[command(
        about = "Count lines, bytes, and tokens for explicit UTF-8 files",
        long_about = "Count logical lines, bytes, and tokens for explicit UTF-8 files.\n\nPaths are never discovered recursively by wflow. Pass file paths directly, or pass newline-delimited paths through --files-from. Token counts are estimates for the selected tiktoken encoding, not provider billing guarantees."
    )]
    Count {
        #[arg(
            long,
            value_enum,
            default_value_t = CliBenchEncoding::O200kBase,
            help = "Token encoding used for the estimate"
        )]
        encoding: CliBenchEncoding,

        #[arg(long, help = "Write machine-readable JSON instead of a table")]
        json: bool,

        #[arg(
            long = "files-from",
            help = "Read newline-delimited file paths from a file, or '-' for stdin"
        )]
        files_from: Option<PathBuf>,

        #[arg(help = "Explicit UTF-8 files to count")]
        files: Vec<PathBuf>,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
#[value(rename_all = "snake_case")]
enum CliBenchEncoding {
    O200kBase,
    Cl100kBase,
}

impl From<CliBenchEncoding> for BenchEncoding {
    fn from(value: CliBenchEncoding) -> Self {
        match value {
            CliBenchEncoding::O200kBase => Self::O200kBase,
            CliBenchEncoding::Cl100kBase => Self::Cl100kBase,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Refs { command } => match command {
            RefsCommand::Sync => run_refs(&cli.root, RefsAction::Sync),
            RefsCommand::Verify => run_refs(&cli.root, RefsAction::Verify),
        },
        Command::Bench { command } => match command {
            BenchCommand::Count {
                encoding,
                json,
                files_from,
                files,
            } => {
                let files = collect_bench_files(files, files_from)?;
                let counts = count_bench(&cli.root, encoding.into(), &files)?;
                if json {
                    serde_json::to_writer_pretty(io::stdout(), &counts).into_diagnostic()?;
                    println!();
                } else {
                    print_bench_table(&counts);
                }
                Ok(())
            }
        },
    }
}

fn collect_bench_files(
    mut files: Vec<PathBuf>,
    files_from: Option<PathBuf>,
) -> Result<Vec<PathBuf>> {
    let Some(files_from) = files_from else {
        return Ok(files);
    };

    let content = if files_from == Path::new("-") {
        let mut content = String::new();
        io::stdin().read_to_string(&mut content).into_diagnostic()?;
        content
    } else {
        fs::read_to_string(&files_from)
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to read --files-from {}", files_from.display()))?
    };

    files.extend(
        content
            .lines()
            .filter(|line| !line.is_empty())
            .map(PathBuf::from),
    );

    Ok(files)
}

fn print_bench_table(counts: &wflow::BenchCount) {
    let width = counts
        .files
        .iter()
        .map(|file| file.path.len())
        .chain(["path".len()])
        .max()
        .unwrap_or("path".len());

    println!(
        "{:<width$} {:>7} {:>7} {:>7}",
        "path", "lines", "bytes", "tokens"
    );
    for file in &counts.files {
        println!(
            "{:<width$} {:>7} {:>7} {:>7}",
            file.path, file.lines, file.bytes, file.tokens
        );
    }
    println!(
        "{:<width$} {:>7} {:>7} {:>7}",
        "total", counts.total.lines, counts.total.bytes, counts.total.tokens
    );
}
