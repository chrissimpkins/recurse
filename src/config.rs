use crate::Shot;

#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) subcmd: Shot,
}

impl Config {
    pub(crate) fn new(opt: Shot) -> Self {
        Self { subcmd: opt }
    }
}
