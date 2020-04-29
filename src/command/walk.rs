use anyhow::{anyhow, Result};

use crate::command::Command;
use crate::ops::io::walk;
use crate::ops::path::{path_has_extension, path_is_hidden};
use crate::Shot;

pub(crate) struct WalkCommand {}

impl Command for WalkCommand {
    fn execute(subcmd: Shot) -> Result<()> {
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
            for entry in walk(inpath, &mindepth, &maxdepth, &symlinks).filter_map(|f| f.ok()) {
                if entry.metadata().unwrap().is_file() {
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
                }
            }
            Ok(())
        } else {
            Err(anyhow!("failure to parse walk subcommand."))
        }
    }
}

// impl WalkCommand {
//     /// Walks an `inpath` for a filtered Vec<PathBuf> based on the parameter
//     /// definitions.
//     pub(crate) fn walk_path(
//         extension: &Option<String>,
//         hidden: &bool,
//         mindepth: &Option<usize>,
//         maxdepth: &Option<usize>,
//         symlinks: &bool,
//         inpath: &PathBuf,
//     ) -> Vec<PathBuf> {
//         let has_extension_filter = extension.is_some();
//         let file_entries = walk(inpath, mindepth, maxdepth, symlinks)
//             .filter_map(|f| f.ok()) // filter paths that process does not have permission to edit
//             .filter_map(|f| {
//                 // filter paths on *files only* (i.e., eliminate dir paths, we don't need them)
//                 if f.path().is_file() {
//                     Some(f.path().to_owned())
//                 } else {
//                     None
//                 }
//             });

//         let mut path_vec: Vec<PathBuf> = Vec::new();
//         for filepath in file_entries {
//             // =================
//             // File path filters
//             // =================
//             if !hidden && path_is_hidden(&filepath) {
//                 // if file is in a hidden path, skip it
//                 continue;
//             } else if has_extension_filter {
//                 // if user requested extension filter, filter on it
//                 if path_has_extension(&filepath, extension.as_ref().unwrap()) {
//                     path_vec.push(filepath);
//                 }
//             } else {
//                 path_vec.push(filepath);
//             }
//         }
//         path_vec
//     }
// }

// pub(crate) struct FileWalkIter {
//     pub walkdir_entries: IntoIter,
//     pub has_extension_filter: bool,
//     pub extension_filter: Option<String>,
//     pub show_hidden_filter: bool,
// }

// impl FileWalkIter {
//     pub(crate) fn new(
//         extension: Option<String>,
//         hidden: bool,
//         mindepth: Option<usize>,
//         maxdepth: Option<usize>,
//         symlinks: bool,
//         inpath: PathBuf,
//     ) -> Self {
//         Self {
//             walkdir_entries: walk(&inpath, &mindepth, &maxdepth, &symlinks),
//             has_extension_filter: extension.is_some(),
//             extension_filter: extension,
//             show_hidden_filter: hidden,
//         }
//     }
// }

// impl Iterator for FileWalkIter {
//     type Item = PathBuf;
//     fn next(&mut self) -> Option<Self::Item> {
//         let entry = self.walkdir_entries.next();
//         if entry.is_some() {
//             match entry.unwrap() {
//                 Ok(entry) => {
//                     // filter on only file paths
//                     // directory paths are not included
//                     if entry.path().is_file() {
//                         // hidden file filter
//                         if !self.show_hidden_filter && path_is_hidden(&entry.path()) {
//                             self.next()
//                         } else if self.has_extension_filter {
//                             // file extension filter
//                             if path_has_extension(
//                                 &entry.path(),
//                                 self.extension_filter.as_ref().unwrap(),
//                             ) {
//                                 Some(entry.path().to_owned())
//                             } else {
//                                 self.next()
//                             }
//                         } else {
//                             // default file case
//                             Some(entry.path().to_owned())
//                         }
//                     } else {
//                         self.next()
//                     }
//                 }
//                 Err(_error) => panic!("panic during directory walk execution!"),
//             }
//         } else {
//             None
//         }
//     }
// }
