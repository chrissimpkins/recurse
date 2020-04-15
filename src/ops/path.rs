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
}
