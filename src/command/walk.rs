use std::path::PathBuf;

use anyhow::{anyhow, Result};

use crate::command::Command;
use crate::ops::io::walk;
use crate::ops::path::{path_has_extension, path_is_hidden};
use crate::Shot;

#[derive(Debug)]
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
            let mut stdout_string = String::new();
            for path in
                WalkCommand::walk_path(extension, hidden, mindepth, maxdepth, symlinks, inpath)
            {
                stdout_string.push_str(&path.to_string_lossy());
                stdout_string.push_str("\n");
            }
            Ok(stdout_string)
        } else {
            Err(anyhow!("failure to parse walk subcommand."))
        }
    }
}

impl WalkCommand {
    /// Walks an `inpath` for a filtered Vec<PathBuf> based on the parameter
    /// definitions.
    pub(crate) fn walk_path(
        extension: Option<String>,
        hidden: bool,
        mindepth: Option<usize>,
        maxdepth: Option<usize>,
        symlinks: bool,
        inpath: PathBuf,
    ) -> Vec<PathBuf> {
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

        let mut path_vec: Vec<PathBuf> = Vec::new();
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
                    path_vec.push(filepath);
                }
            } else {
                path_vec.push(filepath);
            }
        }
        path_vec
    }
}
