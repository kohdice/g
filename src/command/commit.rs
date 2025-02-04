use anyhow::Result;
use clap::Args;

const ABOUT_COMMIT: &str = "Commit Command";
const LONG_ABOUT_COMMIT: &str = "Commit Command Long Description";

#[derive(Args, Debug)]
#[command(about = ABOUT_COMMIT, long_about = LONG_ABOUT_COMMIT)]
pub(crate) struct CommitArgs {
    #[arg(short, long)]
    message: Option<String>,
}

impl CommitArgs {
    pub fn run(&self) -> Result<()> {
        match &self.message {
            Some(message) => println!("The commit message is: {}", message),
            None => println!("No message"),
        }
        Ok(())
    }
}
