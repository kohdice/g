use anyhow::Result;
use clap::Args;

use crate::command::root::Command;

pub const ABOUT_COMMIT: &str = "Commit Command";
pub const LONG_ABOUT_COMMIT: &str = "Commit Command Long Description";

#[derive(Debug, Args)]
pub struct CommitCommand;

impl Command for CommitCommand {
    fn run(&self) -> Result<()> {
        println!("The commit command has been executed");
        Ok(())
    }
}
