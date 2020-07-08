use std::io::Write;

use anyhow::{anyhow, Result};

use crate::command::Command;
use crate::ops::io::walk;
use crate::ops::path::{path_has_extension, path_is_hidden};
use crate::Recurse;

pub(crate) struct WalkCommand {}

impl Command for WalkCommand {
    fn execute(subcmd: Recurse, mut writer: impl Write) -> Result<()> {
        if let Recurse::Walk {
            extension,
            dir_only,
            hidden,
            inpath,
            mindepth,
            maxdepth,
            symlinks,
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

            // Recursive walk of inpath with user-specified filters
            for entry in walk(inpath, &mindepth, &maxdepth, &symlinks).filter_map(|f| f.ok()) {
                let md = entry.metadata().unwrap();
                if !dir_only && md.is_file() {
                    // File path listings
                    let filepath = entry.path();
                    if !hidden && path_is_hidden(filepath) {
                        // if file is in a hidden path, skip it
                        continue;
                    } else if has_extension_filter {
                        // if user requested extension filter, filter on it
                        if path_has_extension(filepath, extension.as_ref().unwrap()) {
                            writeln!(writer, "{}", filepath.display())?;
                        }
                    } else {
                        writeln!(writer, "{}", filepath.display())?;
                    }
                } else if dir_only && md.is_dir() {
                    // Directory path listings
                    let dirpath = entry.path();
                    if !hidden && path_is_hidden(dirpath) {
                        continue;
                    } else {
                        writeln!(writer, "{}", dirpath.display())?;
                    }
                }
            }
            Ok(())
        } else {
            Err(anyhow!("failure to parse walk subcommand."))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_walk_subcmd_invalid_inpath_validation() {
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
        let res = WalkCommand::execute(rw, &mut output);
        // invalid directory path should raise error
        assert!(res.is_err());
        assert!(res
            .unwrap_err()
            .to_string()
            .contains("no such file or directory"));
    }

    #[test]
    fn test_walk_subcmd_dir_with_default_depth() {
        // TODO: need to support windows file paths
        let rw = Recurse::Walk {
            extension: None,
            dir_only: false,
            hidden: false,
            inpath: PathBuf::from("./tests/testfiles/io/stablepaths"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = WalkCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();
        // contains three expected file paths, including file path without extension
        if cfg!(windows) {
            assert!(output_slice.contains("tests\\testfiles\\io\\stablepaths\\README.md"));
            assert!(output_slice.contains("tests\\testfiles\\io\\stablepaths\\test"));
            assert!(output_slice.contains("tests\\testfiles\\io\\stablepaths\\test.txt"));
        } else {
            assert!(output_slice.contains("./tests/testfiles/io/stablepaths/README.md"));
            assert!(output_slice.contains("./tests/testfiles/io/stablepaths/test"));
            assert!(output_slice.contains("./tests/testfiles/io/stablepaths/test.txt"));
        }
        // includes total of 4 lines
        assert!(output_vec.len() == 4);
        // last line is empty string after newline
        assert!(output_vec[3] == "");
    }
}
