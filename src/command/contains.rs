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

    // TODO: test file needs:
    // - text files to confirm functionality:
    //   -- contains text that matches regex
    //   -- does not contain text that matches regex
    //   -- hidden file with text match
    //   -- multiple extensions to test extension filters
    //   -- multi sub-directory structure for min and max depth tests
}
