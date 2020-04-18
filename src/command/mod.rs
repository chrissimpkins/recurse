use anyhow::Result;

pub(crate) mod find;
pub(crate) mod replace;
pub(crate) mod walk;

use crate::Shot;

pub(crate) trait Command {
    fn execute(subcmd: Shot) -> Result<String>;
}
