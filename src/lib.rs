use std::path::{Path, PathBuf};

use anyhow::Result;
use structopt::StructOpt;

use walkdir::{DirEntry, WalkDir};

pub(crate) mod command;
pub(crate) mod config;
pub(crate) mod ops;

use command::find::FindCommand;
use command::replace::ReplaceCommand;
use command::walk::WalkCommand;
use command::Command;
use config::Config;
/// The command line argument implementation
#[derive(StructOpt, Debug)]
#[structopt(about = "A shotgun for text files")]
enum Shot {
    #[structopt(about = "Find text in files")]
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
    #[structopt(about = "Replace text in files")]
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
    #[structopt(about = "Walk the directory structure for files")]
    Walk {
        /// File extension filter
        #[structopt(short = "e", long = "ext", help = "File extension filter")]
        extension: Option<String>,

        /// Include hidden files under dot directory or dot file paths
        /// The default is to not include these files
        #[structopt(long = "hidden", help = "Include hidden files")]
        hidden: bool,

        /// Input file
        #[structopt(parse(from_os_str), help = "In file path")]
        inpath: PathBuf,

        /// Define the minimum depth of the directory traversal
        #[structopt(long = "mindepth", help = "Minimum directory depth")]
        mindepth: Option<usize>,

        /// Define the maximum depth of the directory traversal
        #[structopt(long = "maxdepth", help = "Maximum directory depth")]
        maxdepth: Option<usize>,
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
        } => return FindCommand::execute(config.subcmd),
        Shot::Replace {
            extension: _,
            find: _,
            replace: _,
            inpath: _,
        } => return ReplaceCommand::execute(config.subcmd),
        Shot::Walk {
            extension: _,
            hidden: _,
            inpath: _,
            mindepth: _,
            maxdepth: _,
        } => return WalkCommand::execute(config.subcmd),
    }
}
