use std::fs::read_to_string;
use std::io::ErrorKind;

use anyhow::{anyhow, Result};

use crate::command::walk::WalkCommand;
use crate::command::Command;
use crate::Shot;

#[derive(Debug)]
pub(crate) struct ContainsCommand {}

impl Command for ContainsCommand {
    fn execute(subcmd: Shot) -> Result<String> {
        if let Shot::Contains {
            extension,
            hidden,
            mindepth,
            maxdepth,
            symlinks,
            find,
            inpath,
        } = subcmd
        {
            let mut stdout_string = String::new();
            for path in
                WalkCommand::walk_path(extension, hidden, mindepth, maxdepth, symlinks, inpath)
            {
                match read_to_string(&path) {
                    Ok(filestr) => {
                        if filestr.contains(&find) {
                            stdout_string.push_str(&path.to_string_lossy());
                            stdout_string.push_str("\n");
                        }
                    }
                    Err(error) => match error.kind() {
                        // If this was due to invalid UTF-8 conversion
                        // on file read, then skip the file.
                        // The intent is to test files with valid
                        // UTF-8 encodings only in this subcommand
                        ErrorKind::InvalidData => continue,
                        _ => return Err(anyhow!(error)),
                    },
                }
            }
            Ok(stdout_string)
        } else {
            Err(anyhow!("failure to parse walk subcommand."))
        }
    }
}
