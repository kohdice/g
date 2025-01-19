use anyhow::Result;
use clap::Parser;

use crate::command::root::Commands;

const LONG_ABOUT: &str = r#"A tool to assist with Git operations.

This application provides commands to simplify and enhance
Git workflows, making it easier to work with repositories."#;

#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        self.command.run()
    }
}
