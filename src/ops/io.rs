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
        assert_eq!(
            dirpaths.next().unwrap().unwrap().path(),
            Path::new("./tests/testfiles/io/stablepaths")
        );
        assert_eq!(
            dirpaths.next().unwrap().unwrap().path(),
            Path::new("./tests/testfiles/io/stablepaths/test")
        );
        assert_eq!(
            dirpaths.next().unwrap().unwrap().path(),
            Path::new("./tests/testfiles/io/stablepaths/README.md")
        );
        assert_eq!(
            dirpaths.next().unwrap().unwrap().path(),
            Path::new("./tests/testfiles/io/stablepaths/test.txt")
        );

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
        assert_eq!(
            file_entries.next(),
            Some(PathBuf::from("./tests/testfiles/io/stablepaths/test"))
        );
        assert_eq!(
            file_entries.next(),
            Some(PathBuf::from("./tests/testfiles/io/stablepaths/README.md"))
        );
        assert_eq!(
            file_entries.next(),
            Some(PathBuf::from("./tests/testfiles/io/stablepaths/test.txt"))
        );
        assert!(file_entries.next().is_none()); // the above should have exhausted the file paths in the test
    }
}
