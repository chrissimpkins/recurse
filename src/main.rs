use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

use anyhow::{Context, Result};
use structopt::StructOpt;
use walkdir::{DirEntry, WalkDir};

#[derive(StructOpt)]
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

fn main() -> Result<()> {
    if let Err(error) = run() {
        let _ = writeln!(io::stderr(), "Error: {}", error);
        process::exit(1);
    }
    process::exit(0);
}

fn run() -> Result<()> {
    let opt = Shot::from_args();
    // TODO: implement
    Ok(())
}
