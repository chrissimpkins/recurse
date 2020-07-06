use crate::Recurse;

#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) subcmd: Recurse,
}

impl Config {
    pub(crate) fn new(opt: Recurse) -> Self {
        Self { subcmd: opt }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_config_instantiation_default() {
        let rec_enum = Recurse::Walk {
            extension: None,
            dir_only: false,
            hidden: false,
            inpath: PathBuf::from("."),
            mindepth: None,
            maxdepth: None,
            symlinks: false,
        };
        let config = Config::new(rec_enum);
        match &config.subcmd {
            Recurse::Walk {
                extension,
                dir_only,
                hidden,
                inpath,
                mindepth,
                maxdepth,
                symlinks,
            } => {
                assert_eq!(extension.is_none(), true);
                assert_eq!(*dir_only, false);
                assert_eq!(*hidden, false);
                assert_eq!(inpath, &PathBuf::from("."));
                assert_eq!(mindepth.is_none(), true);
                assert_eq!(maxdepth.is_none(), true);
                assert_eq!(*symlinks, false)
            }
            _ => panic!("The configuration test did not match on the Walk subcommand"),
        }
    }

    #[test]
    fn test_config_instantiation_with_options() {
        let rec_enum = Recurse::Walk {
            extension: Some(String::from("md")),
            dir_only: true,
            hidden: true,
            inpath: PathBuf::from("."),
            mindepth: Some(3),
            maxdepth: Some(3),
            symlinks: true,
        };
        let config = Config::new(rec_enum);
        match &config.subcmd {
            Recurse::Walk {
                extension,
                dir_only,
                hidden,
                inpath,
                mindepth,
                maxdepth,
                symlinks,
            } => {
                assert_eq!(extension, &Some(String::from("md")));
                assert_eq!(*dir_only, true);
                assert_eq!(*hidden, true);
                assert_eq!(inpath, &PathBuf::from("."));
                assert_eq!(mindepth, &Some(3));
                assert_eq!(maxdepth, &Some(3));
                assert_eq!(*symlinks, true)
            }
            _ => panic!("The configuration test did not match on the Walk subcommand"),
        }
    }
}
