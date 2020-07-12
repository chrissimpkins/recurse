use std::fs::read_to_string;
use std::io::{ErrorKind, Write};
use std::path::Path;

use anyhow::{anyhow, Result};
use regex::Regex;

use crate::command::Command;
use crate::ops::io::walk;
use crate::ops::path::{path_has_extension, path_is_hidden};
use crate::Recurse;

pub(crate) struct ContainsCommand {}

impl Command for ContainsCommand {
    fn execute(subcmd: Recurse, mut writer: impl Write) -> Result<()> {
        if let Recurse::Contains {
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
                            ContainsCommand::print_filepath_regex_match(
                                &filepath,
                                &regex,
                                &mut writer,
                            )?;
                        }
                    } else {
                        ContainsCommand::print_filepath_regex_match(
                            &filepath,
                            &regex,
                            &mut writer,
                        )?;
                    }
                }
            }
            Ok(())
        } else {
            Err(anyhow!("failure to parse contains subcommand."))
        }
    }
}

impl ContainsCommand {
    pub(crate) fn print_filepath_regex_match(
        filepath: &Path,
        regex: &Regex,
        writer: &mut impl Write,
    ) -> Result<()> {
        match read_to_string(&filepath) {
            Ok(filestr) => {
                if regex.is_match(&filestr) {
                    writeln!(writer, "{}", &filepath.display())?;
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
    use std::path::PathBuf;

    #[test]
    fn test_contains_subcmd_invalid_inpath_validation() {
        let rw = Recurse::Contains {
            extension: None,
            find: "test".to_string(),
            hidden: false,
            inpath: PathBuf::from("path/to/bogus"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = ContainsCommand::execute(rw, &mut output);
        // invalid directory path should raise error
        assert!(res.is_err());
        assert!(res
            .unwrap_err()
            .to_string()
            .contains("no such file or directory"));
    }

    #[test]
    fn test_contains_invalid_recurse_enum_arg() {
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
        let res = ContainsCommand::execute(rw, &mut output);
        assert!(res.is_err());
        assert!(res
            .unwrap_err()
            .to_string()
            .contains("failure to parse contains subcommand"));
    }

    #[test]
    fn test_contains_invalid_filetype_non_utf8_binary_is_not_logged() {
        let rw = Recurse::Contains {
            extension: None,
            find: ".*".to_string(),
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/contains/librecurse.rlib"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = ContainsCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();
        assert!(output_vec.len() == 1);
        assert!(output_vec[0] == "");
    }

    #[test]
    fn test_contains_default_match() {
        let rw = Recurse::Contains {
            extension: None,
            find: r"\d\d\d\d".to_string(),
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/contains/dir1"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = ContainsCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();
        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_contains_dir1_test1.md"));
        assert!(output_string.contains("tests_testfiles_contains_dir1_dir2_test2.txt"));
        assert!(output_vec.len() == 3);
        assert!(output_vec[2] == "");
    }

    #[test]
    fn test_contains_hidden_match() {
        let rw = Recurse::Contains {
            extension: None,
            find: r"\d\d\d\d".to_string(),
            hidden: true,
            inpath: PathBuf::from("tests/testfiles/contains/dir1"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = ContainsCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();
        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_contains_dir1_test1.md"));
        assert!(output_string.contains("tests_testfiles_contains_dir1_.test-hidden.txt"));
        assert!(output_string.contains("tests_testfiles_contains_dir1_dir2_test2.txt"));
        assert!(output_vec.len() == 4);
        assert!(output_vec[3] == "");
    }

    #[test]
    fn test_contains_filter_match() {
        let rw = Recurse::Contains {
            extension: Some("txt".to_string()),
            find: r"\d\d\d\d".to_string(),
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/contains/dir1"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = ContainsCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();
        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_contains_dir1_dir2_test2.txt"));
        assert!(output_vec.len() == 2);
        assert!(output_vec[1] == "");
    }

    #[test]
    fn test_contains_filter_hidden_match() {
        let rw = Recurse::Contains {
            extension: Some("txt".to_string()),
            find: r"\d\d\d\d".to_string(),
            hidden: true,
            inpath: PathBuf::from("tests/testfiles/contains/dir1"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = ContainsCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();
        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_contains_dir1_dir2_test2.txt"));
        assert!(output_string.contains("tests_testfiles_contains_dir1_.test-hidden.txt"));
        assert!(output_vec.len() == 3);
        assert!(output_vec[2] == "");
    }

    #[test]
    fn test_contains_maxdepth_match() {
        let rw = Recurse::Contains {
            extension: None,
            find: r"\d\d\d\d".to_string(),
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/contains/dir1"),
            mindepth: None,
            maxdepth: Some(1),
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = ContainsCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();
        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_contains_dir1_test1.md"));
        assert!(output_vec.len() == 2);
        assert!(output_vec[1] == "");
    }

    #[test]
    fn test_contains_mindepth_match() {
        let rw = Recurse::Contains {
            extension: None,
            find: r"\d\d\d\d".to_string(),
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/contains/dir1"),
            mindepth: Some(2),
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = ContainsCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();
        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_contains_dir1_dir2_test2.txt"));
        assert!(output_vec.len() == 2);
        assert!(output_vec[1] == "");
    }

    #[test]
    fn test_contains_unicode_extended_devanagari() {
        let rw = Recurse::Contains {
            extension: None,
            find: r"ऄ".to_string(),
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/contains/dir1"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = ContainsCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();
        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_contains_dir1_test1.txt"));
        assert!(output_vec.len() == 2);
        assert!(output_vec[1] == "");
    }
}
