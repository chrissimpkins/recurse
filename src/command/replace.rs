use anyhow::Result;

use crate::command::Command;
use crate::Config;

pub(crate) struct ReplaceCommand {}

impl Command for ReplaceCommand {
    fn execute(config: Config) -> Result<String> {
        Ok(String::from("Test string"))
    }
}
