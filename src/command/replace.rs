use std::fs::{read_to_string, write};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

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
            inplace,
            mindepth,
            maxdepth,
            symlinks,
            find,
            inpath,
            replace,
        } = subcmd
        {
            if is_root_filepath(&inpath) {
                return Err(anyhow!(
                    "shot does not support replacements originating on the file path '{}'",
                    inpath.display()
                ));
            }
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
                            ReplaceCommand::regex_replace(&filepath, &re, &replace, &inplace)?;
                        } // otherwise skip
                    } else {
                        ReplaceCommand::regex_replace(&filepath, &re, &replace, &inplace)?;
                    }
                }
            }
            Ok(())
        } else {
            Err(anyhow!("failure to parse replace subcommand."))
        }
    }
}

impl ReplaceCommand {
    pub(crate) fn regex_replace(
        filepath: &Path,
        re: &Regex,
        replace: &str,
        inplace: &bool,
    ) -> Result<()> {
        match read_to_string(&filepath) {
            Ok(filestr) => {
                // bail if no matches so that we don't
                // write files that are not changed
                if re.is_match(&filestr) {
                    let post_replace_string = re.replace_all(&filestr, replace);
                    if *inplace == true {
                        // TODO: overwrite file
                    } else {
                        // TODO: create a second file path for the new file
                    }
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

fn is_root_filepath(inpath: &PathBuf) -> bool {
    let invalid_list = ["/", r"\"];
    let inpath_needle = inpath.to_string_lossy();
    invalid_list.contains(&inpath_needle.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ======================================
    // is_root_filepath tests
    // ======================================
    #[test]
    fn test_is_root_filepath_with_unix_root() {
        let testpath = PathBuf::from("/");
        assert!(is_root_filepath(&testpath));
    }

    #[test]
    fn test_is_root_filepath_with_win_root() {
        let testpath = PathBuf::from(r"\");
        assert!(is_root_filepath(&testpath));
    }

    #[test]
    fn test_is_root_filepath_without_root_fp() {
        let testpath = PathBuf::from("test/path/bogus");
        assert_eq!(is_root_filepath(&testpath), false);
    }
}
