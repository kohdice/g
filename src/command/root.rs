use anyhow::Result;
use clap::Subcommand;

use crate::command::commit::CommitArgs;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Commit(CommitArgs),
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Commands::Commit(args) => args.run(),
        }
    }
}
