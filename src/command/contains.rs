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
        println!("Sub-command `contains` is not implemented yet. Follow issue https://github.com/chrissimpkins/recurse/issues/5 for progress.");
        Ok(())
        // if let Recurse::Contains {
        //     extension,
        //     hidden,
        //     mindepth,
        //     maxdepth,
        //     symlinks,
        //     find,
        //     inpath,
        // } = subcmd
        // {
        //     let has_extension_filter = extension.is_some();
        //     let regex = Regex::new(&find)?;
        //     for entry in walk(inpath, &mindepth, &maxdepth, &symlinks).filter_map(|f| f.ok()) {
        //         if entry.metadata().unwrap().is_file() {
        //             let filepath = entry.path();
        //             if !hidden && path_is_hidden(filepath) {
        //                 // if file is in a hidden path, skip it
        //                 continue;
        //             } else if has_extension_filter {
        //                 // if user requested extension filter, filter on it
        //                 if path_has_extension(filepath, extension.as_ref().unwrap()) {
        //                     ContainsCommand::print_filepath_regex_match(
        //                         &filepath,
        //                         &regex,
        //                         &mut writer,
        //                     )?;
        //                 }
        //             } else {
        //                 ContainsCommand::print_filepath_regex_match(
        //                     &filepath,
        //                     &regex,
        //                     &mut writer,
        //                 )?;
        //             }
        //         }
        //     }
        //     Ok(())
        // } else {
        //     Err(anyhow!("failure to parse contains subcommand."))
        // }
    }
}

// impl ContainsCommand {
//     pub(crate) fn print_filepath_regex_match(
//         filepath: &Path,
//         regex: &Regex,
//         writer: &mut impl Write,
//     ) -> Result<()> {
//         match read_to_string(&filepath) {
//             Ok(filestr) => {
//                 if regex.is_match(&filestr) {
//                     writeln!(writer, "{}", &filepath.display())?;
//                 }
//             }
//             Err(error) => match error.kind() {
//                 // If this was due to invalid UTF-8 conversion
//                 // on file read, then skip the file.
//                 // The intent is to test files with valid
//                 // UTF-8 encodings only in this subcommand
//                 ErrorKind::InvalidData => {}
//                 _ => return Err(anyhow!(error)),
//             },
//         }
//         Ok(())
//     }
// }
