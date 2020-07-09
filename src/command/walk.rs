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

    // ============
    // File testing
    // ============
    #[test]
    fn test_walk_subcmd_dir_with_default_depth() {
        let rw = Recurse::Walk {
            extension: None,
            dir_only: false,
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/io/stablepaths"),
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
        // the path gymnastics are to support cross-platform file path testing
        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_io_stablepaths_README.md"));
        assert!(output_string.contains("tests_testfiles_io_stablepaths_test"));
        assert!(output_string.contains("tests_testfiles_io_stablepaths_test.txt"));
        // includes total of 4 lines
        assert!(output_vec.len() == 4);
        // last line is empty string after newline
        assert!(output_vec[3] == "");
    }

    #[test]
    fn test_walk_subcmd_with_dir_set_max_depth_1_level() {
        let rw = Recurse::Walk {
            extension: None,
            dir_only: false,
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/io/depthtests"),
            mindepth: None,
            maxdepth: Some(1),
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = WalkCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();

        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_io_depthtests_test.txt"));
        // includes total of 2 lines
        assert!(output_vec.len() == 2);
        // last line is empty string after newline
        assert!(output_vec[1] == "");
    }

    #[test]
    fn test_walk_subcmd_with_dir_set_max_depth_2_levels() {
        let rw = Recurse::Walk {
            extension: None,
            dir_only: false,
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/io/depthtests"),
            mindepth: None,
            maxdepth: Some(2),
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = WalkCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();

        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_io_depthtests_test.txt"));
        assert!(output_string.contains("tests_testfiles_io_depthtests_depth2_test2.txt"));
        // includes total of 3 lines
        assert!(output_vec.len() == 3);
        // last line is empty string after newline
        assert!(output_vec[2] == "");
    }

    #[test]
    fn test_walk_subcmd_with_dir_set_min_depth_3_levels() {
        let rw = Recurse::Walk {
            extension: None,
            dir_only: false,
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/io/depthtests"),
            mindepth: Some(3),
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = WalkCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();

        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_io_depthtests_depth2_depth3_test3.txt"));
        // includes total of 2 lines
        assert!(output_vec.len() == 2);
        // last line is empty string after newline
        assert!(output_vec[1] == "");
    }

    #[test]
    fn test_walk_subcmd_with_extension_filter() {
        let rw = Recurse::Walk {
            extension: Some("txt".to_string()),
            dir_only: false,
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/io/stablepaths"),
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
        // the path gymnastics are to support cross-platform file path testing
        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_io_stablepaths_test.txt"));
        // includes total of 2 lines
        assert!(output_vec.len() == 2);
        // last line is empty string after newline
        assert!(output_vec[1] == "");
    }

    #[test]
    fn test_walk_subcmd_with_extension_filter_alt_ext_format() {
        let rw = Recurse::Walk {
            extension: Some(".txt".to_string()),
            dir_only: false,
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/io/stablepaths"),
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
        // the path gymnastics are to support cross-platform file path testing
        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_io_stablepaths_test.txt"));
        // includes total of 2 lines
        assert!(output_vec.len() == 2);
        // last line is empty string after newline
        assert!(output_vec[1] == "");
    }

    #[test]
    fn test_walk_subcmd_with_hidden_filepaths() {
        let rw = Recurse::Walk {
            extension: None,
            dir_only: false,
            hidden: true,
            inpath: PathBuf::from("tests/testfiles/.dotdir"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = WalkCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();

        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_.dotdir_.testfile"));
        assert!(output_string.contains("tests_testfiles_.dotdir_testfile"));
        assert!(output_string.contains("tests_testfiles_.dotdir_.testfile.txt"));
        // includes total of 4 lines
        assert!(output_vec.len() == 4);
        // last line is empty string after newline
        assert!(output_vec[3] == "");
    }

    #[test]
    fn test_walk_subcmd_with_hidden_filepaths_and_extension_filter() {
        let rw = Recurse::Walk {
            extension: Some("txt".to_string()),
            dir_only: false,
            hidden: true,
            inpath: PathBuf::from("tests/testfiles/.dotdir"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = WalkCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();

        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_.dotdir_.testfile.txt"));
        // includes total of 2 lines
        assert!(output_vec.len() == 2);
        // last line is empty string after newline
        assert!(output_vec[1] == "");
    }

    #[test]
    fn test_walk_subcmd_with_hidden_filepaths_and_extension_filter_alt_ext_format() {
        let rw = Recurse::Walk {
            extension: Some(".txt".to_string()),
            dir_only: false,
            hidden: true,
            inpath: PathBuf::from("tests/testfiles/.dotdir"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = WalkCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();

        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_.dotdir_.testfile.txt"));
        // includes total of 2 lines
        assert!(output_vec.len() == 2);
        // last line is empty string after newline
        assert!(output_vec[1] == "");
    }

    #[test]
    fn test_walk_subcmd_without_hidden_filepaths() {
        let rw = Recurse::Walk {
            extension: None,
            dir_only: false,
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/.dotdir"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = WalkCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();
        // includes total of 1 lines with no paths
        assert!(output_vec.len() == 1);
        // last line is empty string after newline
        assert!(output_vec[0] == "");
    }

    // =================
    // Directory testing
    // =================
    #[test]
    fn test_walk_subcmd_filter_dirs_only_default_depth() {
        let rw = Recurse::Walk {
            extension: None,
            dir_only: true,
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/io/depthtests"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = WalkCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();

        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_io_depthtests"));
        assert!(output_string.contains("tests_testfiles_io_depthtests_depth2"));
        assert!(output_string.contains("tests_testfiles_io_depthtests_depth2_depth3"));
        // includes total of 4 lines
        assert!(output_vec.len() == 4);
        // last line is empty string after newline
        assert!(output_vec[3] == "");
    }

    #[test]
    fn test_walk_subcmd_filter_dirs_only_hidden_switch_off() {
        let rw = Recurse::Walk {
            extension: None,
            dir_only: true,
            hidden: false,
            inpath: PathBuf::from("tests/testfiles/.dotdir"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = WalkCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();
        // includes total of 1 lines
        assert!(output_vec.len() == 1);
        // last line is empty string after newline
        assert!(output_vec[0] == "");
    }

    #[test]
    fn test_walk_subcmd_filter_dirs_only_hidden_switch_on() {
        let rw = Recurse::Walk {
            extension: None,
            dir_only: true,
            hidden: true,
            inpath: PathBuf::from("tests/testfiles/.dotdir"),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let mut output = Vec::new();
        let res = WalkCommand::execute(rw, &mut output);
        assert!(res.is_ok());
        let output_slice = std::str::from_utf8(&output).unwrap();
        let output_vec: Vec<&str> = output_slice.split("\n").collect();

        let mut output_string = output_slice.replace("/", "_");
        output_string = output_string.replace(r"\", "_");
        assert!(output_string.contains("tests_testfiles_.dotdir"));
        // includes total of 2 lines
        assert!(output_vec.len() == 2);
        // last line is empty string after newline
        assert!(output_vec[1] == "");
    }
}
