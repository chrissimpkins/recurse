use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};

/// Returns a canonical absolute file path for the `filepath` argument.
/// This function will return an error in (at least) the following situations:
///
/// - The path does not exist.
/// - A non-final component in path is not a directory.
pub(crate) fn get_absolute_filepath<P>(filepath: P) -> Result<PathBuf>
where
    P: AsRef<Path>,
{
    match filepath.as_ref().canonicalize() {
        Ok(pb) => Ok(pb),
        Err(error) => Err(anyhow!(error)),
    }
}

/// Returns a boolean that indicates whether the `filepath` parameter
/// includes the `extension` file extension.  Also returns `false` in
/// cases where the filepath does not exist or there is no extension.
pub(crate) fn path_has_extension<P>(filepath: P, extension: &str) -> bool
where
    P: AsRef<Path>,
{
    match filepath.as_ref().extension() {
        Some(ext) => {
            // permissive comparison of the extension
            // allows for use of period character in
            // the parameter (e.g., `.txt`)
            if extension.starts_with(".") {
                return ext.to_str().unwrap() == &extension[1..];
            }
            // or no period character in the extension
            // parameter (e.g., `txt`)
            return ext.to_str().unwrap() == extension;
        }
        None => return false,
    }
}

/// Returns a boolean that indicates whether there is a dot directory
/// or dot file anywhere in the canonical absolute path to the file.
/// This is used as a filter during execution and requires a valid
/// file or directory path.  Panic is raised if the path is not valid.
pub(crate) fn path_is_hidden<P>(filepath: P) -> bool
where
    P: AsRef<Path>,
{
    match get_absolute_filepath(filepath) {
        Ok(pb) => {
            for path in pb.iter() {
                if path.to_str().unwrap().starts_with(".") {
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

    // ======================================
    // get_absolute_filepath function tests
    // ======================================
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

    // ======================================
    // path_has_extension function tests
    // ======================================
    #[test]
    fn test_path_has_extension_with_correct_extension() {
        let testpath = Path::new("./tests/testfiles/path/test.txt");
        assert!(path_has_extension(testpath, ".txt"));
        assert!(path_has_extension(testpath, "txt"));
    }

    #[test]
    fn test_path_has_extension_with_incorrect_extension() {
        let testpath = Path::new("./tests/testfiles/path/test.txt");
        assert_eq!(path_has_extension(testpath, ".yaml"), false);
        assert_eq!(path_has_extension(testpath, "yaml"), false);
    }

    #[test]
    fn test_path_has_extension_with_no_extension() {
        let testpath = Path::new("./tests/testfiles/path/testfile");
        assert_eq!(path_has_extension(testpath, ".txt"), false);
        assert_eq!(path_has_extension(testpath, "txt"), false);
    }

    // ======================================
    // path_is_hidden function tests
    // ======================================
    #[test]
    fn test_path_is_hidden_with_dotfile() {
        let testpath = Path::new("./tests/testfiles/path/.testfile");
        assert!(path_is_hidden(testpath));
        assert!(path_is_hidden(testpath)); // confirm that we do not transfer ownership
    }

    #[test]
    fn test_path_is_hidden_with_dotfile_pathbuf() {
        let testpath = PathBuf::from("./tests/testfiles/path/.testfile");
        // Owned types need to be borrowed or ownership is relinquished and
        // raises panic
        assert!(path_is_hidden(&testpath)); // addressed by using &testpath
        assert!(path_is_hidden(testpath)); // ownership transitions on this call
    }

    #[test]
    fn test_path_is_hidden_with_dotdir_in_path() {
        let testpath = Path::new("./tests/testfiles/.dotdir/testfile");
        assert!(path_is_hidden(testpath));
    }

    #[test]
    fn test_path_is_hidden_with_dotdir_only() {
        let testpath = Path::new("./tests/testfiles/.dotdir");
        assert!(path_is_hidden(testpath));
    }

    #[test]
    fn test_path_is_not_hidden_without_dotfile_or_dotdir() {
        let testpath = Path::new("./tests/testfiles/path/testfile");
        assert_eq!(path_is_hidden(testpath), false);
    }
}
