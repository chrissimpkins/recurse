// use std::fs;
// use std::io::{self, Write};
use std::path::Path;

use walkdir::{IntoIter, WalkDir};

pub(crate) fn walk<P>(path: P, mindepth: Option<usize>, maxdepth: Option<usize>) -> IntoIter
where
    P: AsRef<Path>,
{
    let mut wd = WalkDir::new(path);
    if maxdepth.is_some() {
        wd = wd.max_depth(maxdepth.unwrap());
    }
    if mindepth.is_some() {
        wd = wd.min_depth(mindepth.unwrap());
    }
    wd.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_walk_with_dir_default_depth() {
        let mut dirpaths = walk("./tests/testfiles/io/stablepaths", None, None);
        let dirpaths_len_check = walk("./tests/testfiles/io/stablepaths", None, None);
        let expected_list = [
            Path::new("./tests/testfiles/io/stablepaths"),
            Path::new("./tests/testfiles/io/stablepaths/test"),
            Path::new("./tests/testfiles/io/stablepaths/README.md"),
            Path::new("./tests/testfiles/io/stablepaths/test.txt"),
        ];
        // run through all expected files in iterator and check against
        // contents of expected list.
        // Cannot test order directly because order differs across
        // macOS, Win, GNU/Linux platforms based on CI testing results
        assert!(expected_list.contains(&dirpaths.next().unwrap().unwrap().path()));
        assert!(expected_list.contains(&dirpaths.next().unwrap().unwrap().path()));
        assert!(expected_list.contains(&dirpaths.next().unwrap().unwrap().path()));
        assert!(expected_list.contains(&dirpaths.next().unwrap().unwrap().path()));

        let mut index = 0;
        for _ in dirpaths_len_check {
            index += 1;
        }
        assert_eq!(index, 4);
    }

    #[test]
    fn test_walk_with_file_default_depth() {
        let mut filepaths = walk("./tests/testfiles/io/stablepaths/README.md", None, None);
        let filepaths_len_check = walk("./tests/testfiles/io/stablepaths/README.md", None, None);

        assert_eq!(
            filepaths.next().unwrap().unwrap().path(),
            Path::new("./tests/testfiles/io/stablepaths/README.md")
        );

        let mut index = 0;
        for _ in filepaths_len_check {
            index += 1;
        }
        assert_eq!(index, 1);
    }

    #[test]
    fn test_walk_with_filter_map_default_depth() {
        let expected_list = [
            PathBuf::from("./tests/testfiles/io/stablepaths/test"),
            PathBuf::from("./tests/testfiles/io/stablepaths/README.md"),
            PathBuf::from("./tests/testfiles/io/stablepaths/test.txt"),
        ];
        // filter_map to filter out directories that process does not have permission
        // to access
        let mut file_entries = walk("./tests/testfiles/io/stablepaths", None, None)
            .filter_map(|f| f.ok())
            .filter_map(|f| {
                if f.path().is_file() {
                    Some(f.path().to_owned())
                } else {
                    None
                }
            });
        assert!(expected_list.contains(&file_entries.next().unwrap()));
        assert!(expected_list.contains(&file_entries.next().unwrap()));
        assert!(expected_list.contains(&file_entries.next().unwrap()));
        assert_eq!(file_entries.next(), None); // the above should have exhausted the file paths in the test
    }

    #[test]
    fn test_walk_with_dir_set_max_depth() {
        let mut dirpaths = walk("./tests/testfiles/io/depthtests", None, Some(1));
        let dirpaths_len_check = walk("./tests/testfiles/io/depthtests", None, Some(1));
        let expected_list = [
            Path::new("./tests/testfiles/io/depthtests"),
            Path::new("./tests/testfiles/io/depthtests/test.txt"),
            Path::new("./tests/testfiles/io/depthtests/depth2"),
        ];
        assert!(expected_list.contains(&dirpaths.next().unwrap().unwrap().path()));
        assert!(expected_list.contains(&dirpaths.next().unwrap().unwrap().path()));
        assert!(expected_list.contains(&dirpaths.next().unwrap().unwrap().path()));
        let mut index = 0;
        for _ in dirpaths_len_check {
            index += 1;
        }
        assert_eq!(index, 3);
    }

    #[test]
    fn test_walk_with_dir_set_min_depth() {
        let mut dirpaths = walk("./tests/testfiles/io/depthtests", Some(3), None);
        let dirpaths_len_check = walk("./tests/testfiles/io/depthtests", Some(3), None);
        let expected_list = [Path::new(
            "./tests/testfiles/io/depthtests/depth2/depth3/test3.txt",
        )];
        assert!(expected_list.contains(&dirpaths.next().unwrap().unwrap().path()));
        let mut index = 0;
        for _ in dirpaths_len_check {
            index += 1;
        }
        assert_eq!(index, 1);
    }
}
