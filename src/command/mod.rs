use anyhow::Result;

pub(crate) mod find;
pub(crate) mod replace;

use crate::Config;

pub(crate) trait Command {
    fn execute(config: Config) -> Result<String>;
}
