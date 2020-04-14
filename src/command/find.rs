use anyhow::{anyhow, Result};

use crate::command::Command;
use crate::Config;

pub(crate) struct FindCommand {}

impl Command for FindCommand {
    fn execute(config: Config) -> Result<String> {
        Ok(String::from("Test string"))
        // Err(anyhow!("test error"))
    }
}
