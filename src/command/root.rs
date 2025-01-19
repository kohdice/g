use anyhow::Result;
use clap::Subcommand;

use crate::command::commit::{CommitCommand, ABOUT_COMMIT, LONG_ABOUT_COMMIT};

pub trait Command {
    fn run(&self) -> Result<()>;
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(
        about = ABOUT_COMMIT,
        long_about = LONG_ABOUT_COMMIT,
    )]
    Commit,
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Commands::Commit => CommitCommand.run(),
        }
    }
}
