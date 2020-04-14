use std::path::{Path, PathBuf};

use anyhow::Result;
use structopt::StructOpt;

use walkdir::{DirEntry, WalkDir};

pub(crate) mod command;
pub(crate) mod config;

use command::find::FindCommand;
use command::replace::ReplaceCommand;
use command::Command;
use config::Config;
/// The command line argument implementation
#[derive(StructOpt, Debug)]
#[structopt(about = "Text manipulation tool for files")]
enum Shot {
    Find {
        /// File extension filter
        #[structopt(short = "e", long = "ext", help = "File extension filter")]
        extension: Option<String>,
        /// Find string
        #[structopt(help = "Find string")]
        find: String,

        /// Input file
        #[structopt(parse(from_os_str), help = "In file path")]
        inpath: PathBuf,
    },
    Replace {
        /// File extension filter
        #[structopt(short = "e", long = "ext", help = "File extension filter")]
        extension: Option<String>,
        /// Find string
        #[structopt(help = "Find string")]
        find: String,

        /// Replace string
        #[structopt(help = "Replace string")]
        replace: String,

        /// Input file
        #[structopt(parse(from_os_str), help = "In file path")]
        inpath: PathBuf,
    },
}

/// `shot` executable execution entry point
pub fn run() -> Result<String> {
    let config = Config::new(Shot::from_args());
    match &config.subcmd {
        Shot::Find {
            extension: _,
            find: _,
            inpath: _,
        } => match FindCommand::execute(config) {
            Ok(stdout) => Ok(stdout),
            Err(error) => Err(error),
        },
        Shot::Replace {
            extension: _,
            find: _,
            replace: _,
            inpath: _,
        } => match ReplaceCommand::execute(config) {
            Ok(stdout) => Ok(stdout),
            Err(error) => Err(error),
        },
    }
}
