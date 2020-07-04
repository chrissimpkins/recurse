use anyhow::Result;

pub(crate) mod contains;
pub(crate) mod find;
pub(crate) mod replace;
pub(crate) mod walk;

use crate::Recurse;

pub(crate) trait Command {
    fn execute(subcmd: Recurse) -> Result<()>;
}
