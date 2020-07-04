use crate::Recurse;

#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) subcmd: Recurse,
}

impl Config {
    pub(crate) fn new(opt: Recurse) -> Self {
        Self { subcmd: opt }
    }
}
