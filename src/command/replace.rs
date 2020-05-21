use std::fs::{read_to_string, write, OpenOptions};
use std::io::prelude::*;
use std::io::{BufWriter, ErrorKind};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use regex::Regex;

use crate::command::Command;
use crate::ops::io::walk;
use crate::ops::path::{path_has_extension, path_is_hidden};
use crate::Shot;

const SECONDARY_FILEPATH_EXTENSION: &str = "2";

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
            // Protect against accidental attempts to replace every path beginning at root
            // when a path typo of `/` (Unix) or `\` (Win) is used on the command line
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
                    if has_secondary_extension(&filepath) {
                        // If file has the secondary extension that is used by
                        // this application, do not perform string replacement
                        // in that file. Use the original and overwrite the
                        // secondary file with previous replacements.
                        // Removal of this check will lead to additional secondary
                        // files with new extensions based on the last secondary
                        // file path in sequence.  This is not desired behavior.
                        continue;
                    } else if !hidden && path_is_hidden(filepath) {
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
                        // Write files in place when explicitly requested by user
                        // ========================================================
                        //
                        // DANGER ZONE
                        //
                        // ========================================================
                        // With directory walks this is a potentially very
                        // dangerous code block that has the potential to cause
                        // widespread file system mayhem.  User *must* explicitly
                        // request that they want in place file writes to use this
                        // functionality.
                        // With great power comes great responsibility...
                        let file = OpenOptions::new().write(true).create(true).open(filepath)?;
                        let mut buffer = BufWriter::new(file);

                        buffer.write_all(post_replace_string.as_bytes())?;
                        buffer.flush()?;
                        println!("{} updated", filepath.display());
                    } else {
                        // Create a secondary file path by default
                        let secondary_filepath = get_secondary_filepath(filepath);
                        let file = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .open(&secondary_filepath)?;
                        let mut buffer = BufWriter::new(file);

                        buffer.write_all(post_replace_string.as_bytes())?;
                        buffer.flush()?;
                        println!(
                            "{} updated with write to {}",
                            filepath.display(),
                            secondary_filepath.display()
                        );
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

fn get_secondary_filepath(inpath: &Path) -> PathBuf {
    match inpath.extension() {
        Some(pre_ext) => {
            let post_ext = pre_ext.to_string_lossy() + "." + SECONDARY_FILEPATH_EXTENSION;
            return inpath.with_extension(post_ext.to_string());
        }
        None => {
            return inpath.with_extension(SECONDARY_FILEPATH_EXTENSION);
        }
    }
}

fn has_secondary_extension(inpath: &Path) -> bool {
    match inpath.extension() {
        Some(ext) => return ext.to_string_lossy() == SECONDARY_FILEPATH_EXTENSION,
        None => return false,
    }
}

fn is_root_filepath(inpath: &Path) -> bool {
    let invalid_list = ["/", r"\"];
    let inpath_needle = inpath.to_string_lossy();
    invalid_list.contains(&inpath_needle.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ======================================
    // get_secondary_filepath tests
    // ======================================
    #[test]
    fn test_get_secondary_filepath_with_txt_extension() {
        let testpath = PathBuf::from("test/path/bogus.txt");
        assert_eq!(
            get_secondary_filepath(&testpath),
            PathBuf::from("test/path/bogus.txt.2")
        );
    }

    #[test]
    fn test_get_secondary_filepath_with_2_extension() {
        let testpath = PathBuf::from("test/path/bogus.2");
        assert_eq!(
            get_secondary_filepath(&testpath),
            PathBuf::from("test/path/bogus.2.2")
        );
    }

    #[test]
    fn test_get_secondary_filepath_without_extension() {
        let testpath = PathBuf::from("test/path/bogus");
        assert_eq!(
            get_secondary_filepath(&testpath),
            PathBuf::from("test/path/bogus.2")
        );
    }
    // ======================================
    // has_secondary_extension tests
    // ======================================
    #[test]
    fn test_has_secondary_extension_with_secondary_extension() {
        let testpath = PathBuf::from("test/path/bogus.txt.2");
        assert!(has_secondary_extension(&testpath));
    }

    #[test]
    fn test_has_secondary_extension_without_secondary_extension() {
        let testpath = PathBuf::from("test/path/bogus.txt");
        assert_eq!(has_secondary_extension(&testpath), false);
    }

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
