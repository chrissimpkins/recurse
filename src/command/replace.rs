use std::fs::{read_to_string, write};
use std::io::ErrorKind;
use std::path::Path;

use anyhow::{anyhow, Result};
use regex::Regex;

use crate::command::Command;
use crate::ops::io::walk;
use crate::ops::path::{path_has_extension, path_is_hidden};
use crate::Shot;

pub(crate) struct ReplaceCommand {}

impl Command for ReplaceCommand {
    fn execute(subcmd: Shot) -> Result<()> {
        if let Shot::Replace {
            extension,
            hidden,
            mindepth,
            maxdepth,
            symlinks,
            find,
            inpath,
            replace,
        } = subcmd
        {
            if inpath.to_string_lossy() == "/" || inpath.to_string_lossy() == r"\" {
                return Err(anyhow!(
                    "shot does not support directory walk replacements originating on the file path '{}'",
                    inpath.display()
                ));
            }
            // TODO: add guard against running this on very broad root level directory paths
            let has_extension_filter = extension.is_some();
            let re = Regex::new(&find)?;
            for entry in walk(inpath, &mindepth, &maxdepth, &symlinks).filter_map(|f| f.ok()) {
                if entry.metadata().unwrap().is_file() {
                    let filepath = entry.path();
                    if !hidden && path_is_hidden(filepath) {
                        // if file is in a hidden path, skip it
                        continue;
                    } else if has_extension_filter {
                        // if user requested extension filter, filter on it
                        if path_has_extension(filepath, extension.as_ref().unwrap()) {
                            ReplaceCommand::regex_replace(&filepath, &re, &replace)?;
                        }
                    } else {
                        ReplaceCommand::regex_replace(&filepath, &re, &replace)?;
                    }
                }
            }
            Ok(())
        } else {
            Err(anyhow!("failure to parse find subcommand."))
        }
    }
}

impl ReplaceCommand {
    pub(crate) fn regex_replace(filepath: &Path, re: &Regex, replace: &str) -> Result<()> {
        match read_to_string(&filepath) {
            Ok(filestr) => {
                // bail if no matches so that we don't
                // write files that are not changed
                if re.is_match(&filestr) {
                    let post_replace_string = re.replace_all(&filestr, replace);
                    println!("{}:\n{}", filepath.display(), post_replace_string);
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
