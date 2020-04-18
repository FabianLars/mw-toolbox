use std::path::PathBuf;

use crate::{Cli, Subcommand};

#[derive(Debug, Clone)]
pub struct Props {
    pub(crate) path: PathType,
    pub(crate) parameter: Option<String>,
    pub(crate) loginname: String,
    pub(crate) loginpassword: String,
}

// TODO: Consider Builder Pattern

impl Props {
    pub(crate) fn from_delete(args: Cli) -> Self {
        let path = match args.command.unwrap() {
            Subcommand::Delete { input } => PathType::File(input)/*std::fs::read_to_string(input).unwrap()*/,
            _ => panic!("weird error"),
        };

        return Self {
            path,
            parameter: None,
            loginname: args.loginname,
            loginpassword: args.loginpassword,
        };
    }

    pub(crate) fn from_list(args: Cli) -> Self {
        let path: PathType;
        let param;

        match args.command.unwrap() {
            Subcommand::List {
                output, parameter, ..
            } => {
                path = PathType::File(match output {
                    Some(x) => x,
                    None => PathBuf::from("./wtools_output.json"),
                });

                param = parameter;
            }
            _ => panic!("weird error"),
        }

        Self {
            path,
            parameter: param,
            loginname: args.loginname,
            loginpassword: args.loginpassword,
        }
    }

    pub(crate) fn from_move(args: Cli) -> Self {
        let path = match args.command.unwrap() {
            Subcommand::Move { input } => PathType::File(input),
            _ => panic!("weird error"),
        };

        Self {
            path,
            parameter: None,
            loginname: args.loginname,
            loginpassword: args.loginpassword,
        }
    }

    #[cfg(feature = "league")]
    pub(crate) fn from_league(args: Cli) -> Self {
        let p = match args.command.unwrap() {
            Subcommand::League { path, .. } => match path {
                Some(x) => x,
                None => PathBuf::from("./wtools_output.json"),
            },
            _ => panic!("weird error"),
        };

        Self {
            path: PathType::File(p),
            parameter: None,
            loginname: args.loginname,
            loginpassword: args.loginpassword,
        }
    }

    pub(crate) fn from_upload(args: Cli) -> Self {
        let path = match args.command.unwrap() {
            Subcommand::Upload { input, .. } => {
                if std::fs::metadata(&input).unwrap().is_dir() {
                    PathType::Folder(input)
                } else if std::fs::metadata(&input).unwrap().is_file() {
                    PathType::File(input)
                } else {
                    panic!("weird error");
                }
            }
            _ => panic!("weird error"),
        };

        Self {
            path,
            parameter: None,
            loginname: args.loginname,
            loginpassword: args.loginpassword,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum PathType {
    File(PathBuf),
    Files(Vec<PathBuf>),
    Folder(PathBuf),
}

impl Default for PathType {
    fn default() -> Self {
        PathType::File(PathBuf::new())
    }
}

impl PathType {
    pub fn file_path(self) -> PathBuf {
        if let PathType::File(p) = self { p } else { panic!("Not a file") }
    }
}