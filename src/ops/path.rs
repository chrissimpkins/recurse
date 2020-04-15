use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};

/// Returns a canonical absolute file path for the `filepath` argument.
/// This function will return an error in (at least) the following situations:
///
/// - The path does not exist.
/// - A non-final component in path is not a directory.
pub(crate) fn get_absolute_filepath<P>(filepath: P) -> Result<PathBuf>
where
    P: Into<PathBuf>,
{
    match filepath.into().canonicalize() {
        Ok(pb) => Ok(pb),
        Err(error) => Err(anyhow!(error)),
    }
}

/// Returns a boolean that indicates whether there is a dot directory
/// or dot file anywhere in the canonical absolute path to the file.
/// This is used as a filter during execution and requires a valid
/// file or directory path.  Panic is raised if the path is not valid.
pub(crate) fn path_is_hidden<P>(filepath: P) -> bool
where
    P: Into<PathBuf>,
{
    match get_absolute_filepath(filepath) {
        Ok(pb) => {
            for path in pb.iter() {
                if path.to_string_lossy().starts_with(".") {
                    return true;
                }
            }
            false
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_absolute_filepath_good_path() {
        let testpath = get_absolute_filepath("./Cargo.toml");
        assert!(testpath.is_ok());
        assert!(testpath.unwrap().ends_with("shot/Cargo.toml"));
    }

    #[test]
    fn test_get_absolute_filepath_bad_path() {
        let testpath = get_absolute_filepath("./bogus.bad");
        assert!(testpath.is_err());
    }

    #[test]
    fn test_path_is_hidden_with_dotfile() {
        let testpath = Path::new("./tests/testfiles/path/.testfile");
        assert!(path_is_hidden(testpath));
    }

    #[test]
    fn test_path_is_hidden_with_dotdir() {
        let testpath = Path::new("./tests/testfiles/.dotdir/testfile");
        assert!(path_is_hidden(testpath));
    }

    #[test]
    fn test_path_is_not_hidden_without_dotfile_or_dotdir() {
        let testpath = Path::new("./tests/testfiles/path/testfile");
        assert_eq!(path_is_hidden(testpath), false);
    }
}
