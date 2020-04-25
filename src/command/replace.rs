use anyhow::Result;

use crate::command::Command;
use crate::Shot;

pub(crate) struct ReplaceCommand {}

impl Command for ReplaceCommand {
    fn execute(subcmd: Shot) -> Result<()> {
        Ok(())
    }
}
