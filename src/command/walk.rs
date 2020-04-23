use anyhow::{anyhow, Result};

use crate::command::Command;
use crate::ops::io::walk;
use crate::ops::path::{path_has_extension, path_is_hidden};
use crate::Shot;

pub(crate) struct WalkCommand {}

impl Command for WalkCommand {
    fn execute(subcmd: Shot) -> Result<String> {
        if let Shot::Walk {
            extension,
            hidden,
            inpath,
            mindepth,
            maxdepth,
            symlinks,
        } = subcmd
        {
            let has_extension_filter = extension.is_some();
            let file_entries = walk(inpath, mindepth, maxdepth, symlinks)
                .filter_map(|f| f.ok()) // filter paths that process does not have permission to edit
                .filter_map(|f| {
                    // filter paths on *files only* (i.e., eliminate dir paths, we don't need them)
                    if f.path().is_file() {
                        Some(f.path().to_owned())
                    } else {
                        None
                    }
                });

            let mut stdout_string = String::new();
            for filepath in file_entries {
                // =================
                // File path filters
                // =================
                if !hidden && path_is_hidden(&filepath) {
                    // if file is in a hidden path, skip it
                    continue;
                } else if has_extension_filter {
                    // if user requested extension filter, filter on it
                    if path_has_extension(&filepath, extension.as_ref().unwrap()) {
                        stdout_string.push_str(&filepath.to_string_lossy());
                        stdout_string.push_str("\n");
                    }
                } else {
                    stdout_string.push_str(&filepath.to_string_lossy());
                    stdout_string.push_str("\n");
                }
            }
            return Ok(stdout_string);
        }
        return Err(anyhow!("failure to parse walk subcommand."));
    }
}
