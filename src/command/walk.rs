use anyhow::Result;

use crate::command::Command;
use crate::Config;

pub(crate) struct WalkCommand {}

impl Command for WalkCommand {
    fn execute(config: Config) -> Result<String> {
        Ok(String::from("Test string"))
    }
}
