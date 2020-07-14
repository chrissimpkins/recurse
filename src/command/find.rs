use std::fs::read_to_string;
use std::io::{ErrorKind, Write};
use std::path::Path;

use anyhow::{anyhow, Result};
use colored::*;
use regex::Regex;

use crate::command::Command;
use crate::ops::io::walk;
use crate::ops::path::{path_has_extension, path_is_hidden};
use crate::Recurse;

pub(crate) struct FindCommand {}

impl Command for FindCommand {
    fn execute(subcmd: Recurse, mut writer: impl Write) -> Result<()> {
        if let Recurse::Find {
            extension,
            hidden,
            mindepth,
            maxdepth,
            symlinks,
            find,
            inpath,
        } = subcmd
        {
            // ------------
            // Validations
            // ------------
            // 1) inpath exists, if not bail with error
            if !inpath.exists() {
                return Err(anyhow!(format!(
                    "no such file or directory '{}'",
                    inpath.display()
                )));
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
                            FindCommand::print_filepath_regex_matches(&filepath, &re, &mut writer)?;
                        }
                    } else {
                        FindCommand::print_filepath_regex_matches(&filepath, &re, &mut writer)?;
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
    pub(crate) fn print_filepath_regex_matches(
        filepath: &Path,
        re: &Regex,
        writer: &mut impl Write,
    ) -> Result<()> {
        match read_to_string(&filepath) {
            Ok(filestr) => {
                // short circuit the individual line checks if overall match does not
                // indicate the presence of a match
                if re.is_match(&filestr) {
                    // iterate through lines and print matches
                    let mut line_number = 0;
                    for line in filestr.lines() {
                        line_number += 1;
                        for mat in re.find_iter(line) {
                            writeln!(
                                writer,
                                "{} {} {} {} {}",
                                &filepath.display(),
                                format!("{}:{}-{}", &line_number, &mat.start(), &mat.end()).green(),
                                "[".dimmed().bold(),
                                &mat.as_str().red(),
                                "]".dimmed().bold(),
                            )?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn test_find_subcmd_invalid_inpath_validation() {
        let rw = Recurse::Find {
            extension: None,
            find: "test".to_string(),
            hidden: false,
            inpath: PathBuf::from("path/to/bogus"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = FindCommand::execute(rw, &mut output);
        // invalid directory path should raise error
        assert!(res.is_err());
        assert!(res
            .unwrap_err()
            .to_string()
            .contains("no such file or directory"));
    }

    #[test]
    fn test_find_invalid_recurse_enum_arg() {
        let rw = Recurse::Walk {
            extension: None,
            dir_only: false,
            hidden: false,
            inpath: PathBuf::from("path/to/bogus"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = FindCommand::execute(rw, &mut output);
        assert!(res.is_err());
        assert!(res
            .unwrap_err()
            .to_string()
            .contains("failure to parse find subcommand"));
    }

    #[test]
    fn test_find_invalid_filetype_non_utf8_binary_is_not_logged() {
        let rw = Recurse::Find {
            extension: None,
            find: ".*".to_string(),
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/find/librecurse.rlib"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = FindCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();
        assert!(output_vec.len() == 1);
        assert!(output_vec[0] == "");
    }

    #[test]
    fn test_find_default_match() {
        // setup
        env::set_var("NO_COLOR", "1");
        let rw = Recurse::Find {
            extension: None,
            find: r"\d\d\d\d".to_string(),
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/contains/dir1"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = FindCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();
        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_contains_dir1_test1.md 2:0-4 [ 1010 ]"));
        assert!(
            output_string.contains("tests_testfiles_contains_dir1_dir2_test2.txt 2:0-4 [ 1010 ]")
        );
        assert!(output_vec.len() == 3);
        assert!(output_vec[2] == "");
    }
}
