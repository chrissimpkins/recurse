use std::fs::read_to_string;
use std::io::ErrorKind;
use std::path::Path;

use anyhow::{anyhow, Result};
use colored::*;
use regex::Regex;

use crate::command::Command;
use crate::ops::io::walk;
use crate::ops::path::{path_has_extension, path_is_hidden};
use crate::Shot;

pub(crate) struct FindCommand {}

impl Command for FindCommand {
    fn execute(subcmd: Shot) -> Result<()> {
        if let Shot::Find {
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
            let regex = Regex::new(&find)?;
            for entry in walk(inpath, &mindepth, &maxdepth, &symlinks).filter_map(|f| f.ok()) {
                if entry.metadata().unwrap().is_file() {
                    let filepath = entry.path();
                    if !hidden && path_is_hidden(filepath) {
                        // if file is in a hidden path, skip it
                        continue;
                    } else if has_extension_filter {
                        // if user requested extension filter, filter on it
                        if path_has_extension(filepath, extension.as_ref().unwrap()) {
                            FindCommand::print_filepath_regex_matches(&filepath, &regex)?;
                        }
                    } else {
                        FindCommand::print_filepath_regex_matches(&filepath, &regex)?;
                    }
                }
            }
            Ok(())
        } else {
            Err(anyhow!("failure to parse find subcommand."))
        }
    }
}

impl FindCommand {
    pub(crate) fn print_filepath_regex_matches(filepath: &Path, regex: &Regex) -> Result<()> {
        match read_to_string(&filepath) {
            Ok(filestr) => {
                // short circuit the individual line checks if overall match does not
                // indicate the presence of a match
                if regex.is_match(&filestr) {
                    // iterate through lines and print matches
                    let mut line_number = 0;
                    for line in filestr.lines() {
                        line_number += 1;
                        for mat in regex.find_iter(line) {
                            println!(
                                "{} {}:{}-{} {} {} {}",
                                &filepath.display(),
                                &line_number.to_string().green(),
                                &mat.start().to_string().green(),
                                &mat.end().to_string().green(),
                                "[".dimmed(),
                                &mat.as_str().red().bold(),
                                "]".dimmed(),
                            );
                        }
                    }
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
