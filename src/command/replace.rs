use std::fs::{read_to_string, OpenOptions};
use std::io::prelude::*;
use std::io::{BufWriter, ErrorKind};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use regex::Regex;

use crate::command::Command;
use crate::ops::io::walk;
use crate::ops::path::{path_has_extension, path_is_hidden};
use crate::Recurse;

const BACKUP_FILEPATH_EXTENSION: &str = "bu";

pub(crate) struct ReplaceCommand {}

impl Command for ReplaceCommand {
    fn execute(subcmd: Recurse) -> Result<()> {
        if let Recurse::Replace {
            extension,
            hidden,
            nobu,
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
                    "recurse does not support replacements originating on the file path '{}'",
                    inpath.display()
                ));
            }
            let has_extension_filter = extension.is_some();
            let re = Regex::new(&find)?;
            for entry in walk(inpath, &mindepth, &maxdepth, &symlinks).filter_map(|f| f.ok()) {
                if entry.metadata().unwrap().is_file() {
                    let filepath = entry.path();
                    if has_backup_extension(&filepath) {
                        // If file has the backup extension that is used by
                        // this application, do not perform string replacement
                        // in that file.
                        continue;
                    } else if !hidden && path_is_hidden(filepath) {
                        // if file is in a hidden path, skip it
                        continue;
                    } else if has_extension_filter {
                        // if user requested extension filter, filter on it
                        if path_has_extension(filepath, extension.as_ref().unwrap()) {
                            ReplaceCommand::regex_replace(&filepath, &re, &replace, &nobu)?;
                        } // otherwise skip
                    } else {
                        ReplaceCommand::regex_replace(&filepath, &re, &replace, &nobu)?;
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
        no_backup: &bool,
    ) -> Result<()> {
        match read_to_string(&filepath) {
            Ok(filestr) => {
                // bail if no matches so that we don't
                // write files that are not changed
                if re.is_match(&filestr) {
                    let post_replace_string = re.replace_all(&filestr, replace);

                    if *no_backup == false {
                        // Write backup of original file
                        // This is the default behavior when user
                        // does not use an explicit flag on the
                        // command line
                        let backup_file = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .open(get_backup_filepath(filepath))?;
                        let mut backup_buffer = BufWriter::new(backup_file);
                        backup_buffer.write_all(&filestr.as_bytes())?;
                        backup_buffer.flush()?;
                    }

                    // write replacement string inplace
                    let replace_file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(filepath)?;
                    let mut buffer = BufWriter::new(replace_file);

                    buffer.write_all(post_replace_string.as_bytes())?;
                    buffer.flush()?;
                    println!("{} updated", filepath.display());
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

fn get_backup_filepath(inpath: &Path) -> PathBuf {
    match inpath.extension() {
        Some(pre_ext) => {
            let post_ext = pre_ext.to_string_lossy() + "." + BACKUP_FILEPATH_EXTENSION;
            return inpath.with_extension(post_ext.to_string());
        }
        None => {
            return inpath.with_extension(BACKUP_FILEPATH_EXTENSION);
        }
    }
}

fn has_backup_extension(inpath: &Path) -> bool {
    match inpath.extension() {
        Some(ext) => return ext.to_string_lossy() == BACKUP_FILEPATH_EXTENSION,
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
            get_backup_filepath(&testpath),
            PathBuf::from("test/path/bogus.txt.bu")
        );
    }

    #[test]
    fn test_get_secondary_filepath_with_2_extension() {
        let testpath = PathBuf::from("test/path/bogus.2");
        assert_eq!(
            get_backup_filepath(&testpath),
            PathBuf::from("test/path/bogus.2.bu")
        );
    }

    #[test]
    fn test_get_secondary_filepath_without_extension() {
        let testpath = PathBuf::from("test/path/bogus");
        assert_eq!(
            get_backup_filepath(&testpath),
            PathBuf::from("test/path/bogus.bu")
        );
    }
    // ======================================
    // has_secondary_extension tests
    // ======================================
    #[test]
    fn test_has_secondary_extension_with_secondary_extension() {
        let testpath = PathBuf::from("test/path/bogus.txt.bu");
        assert!(has_backup_extension(&testpath));
    }

    #[test]
    fn test_has_secondary_extension_with_secondary_extension_alt1() {
        let testpath = PathBuf::from("test/path/bogus.bu");
        assert!(has_backup_extension(&testpath));
    }

    #[test]
    fn test_has_secondary_extension_without_secondary_extension() {
        let testpath = PathBuf::from("test/path/bogus.txt");
        assert_eq!(has_backup_extension(&testpath), false);
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
