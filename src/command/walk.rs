use anyhow::{anyhow, Result};

use crate::command::Command;
use crate::ops::io::walk;
use crate::ops::path::{path_has_extension, path_is_hidden};
use crate::Recurse;

pub(crate) struct WalkCommand {}

impl Command for WalkCommand {
    fn execute(subcmd: Recurse) -> Result<()> {
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
            let has_extension_filter = extension.is_some();
            // Validations
            // 1) inpath exists, if not bail with error
            if !inpath.exists() {
                return Err(anyhow!(format!(
                    "no such file or directory '{}'",
                    inpath.display()
                )));
            }

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
                            println!("{}", filepath.display());
                        }
                    } else {
                        println!("{}", filepath.display());
                    }
                } else if dir_only && md.is_dir() {
                    // Directory path listings
                    let dirpath = entry.path();
                    if !hidden && path_is_hidden(dirpath) {
                        continue;
                    } else {
                        println!("{}", dirpath.display());
                    }
                }
            }
            Ok(())
        } else {
            Err(anyhow!("failure to parse walk subcommand."))
        }
    }
}
