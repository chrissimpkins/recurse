// use std::fs;
// use std::io::{self, Write};
use std::path::Path;

use walkdir::{IntoIter, WalkDir};

pub(crate) fn walk<P>(path: P) -> IntoIter
where
    P: AsRef<Path>,
{
    WalkDir::new(path).into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_walk_with_dir() {
        let mut dirpaths = walk("./tests/testfiles/io/stablepaths");
        let dirpaths_len_check = walk("./tests/testfiles/io/stablepaths");
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
    fn test_walk_with_file() {
        let mut filepaths = walk("./tests/testfiles/io/stablepaths/README.md");
        let filepaths_len_check = walk("./tests/testfiles/io/stablepaths/README.md");

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
    fn test_walk_with_filter_map() {
        let expected_list = [
            PathBuf::from("./tests/testfiles/io/stablepaths/test"),
            PathBuf::from("./tests/testfiles/io/stablepaths/README.md"),
            PathBuf::from("./tests/testfiles/io/stablepaths/test.txt"),
        ];
        // filter_map to filter out directories that process does not have permission
        // to access
        let mut file_entries = walk("./tests/testfiles/io/stablepaths")
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
}
