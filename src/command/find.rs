use anyhow::{anyhow, Result};

use crate::command::Command;
use crate::Shot;

pub(crate) struct FindCommand {}

impl Command for FindCommand {
    fn execute(subcmd: Shot) -> Result<String> {
        Ok(String::from("Test string"))
        // Err(anyhow!("test error"))
    }
}
