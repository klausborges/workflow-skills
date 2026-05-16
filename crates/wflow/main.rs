use std::path::PathBuf;

use clap::{Parser, Subcommand};
use miette::Result;
use wflow::{RefsAction, run_refs};

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
}

#[derive(Debug, Subcommand)]
enum RefsCommand {
    Sync,
    Verify,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Refs { command } => match command {
            RefsCommand::Sync => run_refs(&cli.root, RefsAction::Sync),
            RefsCommand::Verify => run_refs(&cli.root, RefsAction::Verify),
        },
    }
}
