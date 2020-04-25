use std::fs::read_to_string;
use std::io::ErrorKind;
use std::path::Path;

use anyhow::{anyhow, Result};
use rayon::prelude::*;

// use crate::command::walk::WalkCommand;
use crate::command::Command;
use crate::ops::io::walk;
use crate::ops::path::{path_has_extension, path_is_hidden};
use crate::Shot;

#[derive(Debug)]
pub(crate) struct ContainsCommand {}

impl Command for ContainsCommand {
    fn execute(subcmd: Shot) -> Result<()> {
        if let Shot::Contains {
            extension,
            hidden,
            mindepth,
            maxdepth,
            symlinks,
            find,
            inpath,
        } = subcmd
        {
            let has_extension_filter = extension.is_some();
            for entry in walk(inpath, &mindepth, &maxdepth, &symlinks).filter_map(|f| f.ok()) {
                if entry.metadata().unwrap().is_file() {
                    let filepath = entry.path();
                    if !hidden && path_is_hidden(filepath) {
                        // if file is in a hidden path, skip it
                        continue;
                    } else if has_extension_filter {
                        // if user requested extension filter, filter on it
                        if path_has_extension(filepath, extension.as_ref().unwrap()) {
                            ContainsCommand::print_filepath_contains(&filepath, &find)?;
                        }
                    } else {
                        ContainsCommand::print_filepath_contains(&filepath, &find)?;
                    }
                }
            }
            Ok(())
        } else {
            Err(anyhow!("failure to parse walk subcommand."))
        }
    }
}

impl ContainsCommand {
    pub(crate) fn print_filepath_contains(filepath: &Path, needle: &str) -> Result<()> {
        match read_to_string(&filepath) {
            Ok(filestr) => {
                if filestr.contains(&needle) {
                    println!("{}", &filepath.display());
                }
            }
            Err(error) => match error.kind() {
                // If this was due to invalid UTF-8 conversion
                // on file read, then skip the file.
                // The intent is to test files with valid
                // UTF-8 encodings only in this subcommand
                ErrorKind::InvalidData => {}
                _ => return Err(anyhow!(error)),
            },
        }
        Ok(())
    }
}
