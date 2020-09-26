use std::path::PathBuf;

use crate::error::PathTypeError;

#[derive(Debug, Clone)]
pub enum PathType {
    File(PathBuf),
    Files(Vec<PathBuf>),
    Folder(PathBuf),
}

impl PathType {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(pathbuf: PathBuf) -> Result<Self, PathTypeError> {
        if std::fs::metadata(&pathbuf)?.is_dir() {
            Ok(PathType::Folder(pathbuf))
        } else if std::fs::metadata(&pathbuf)?.is_file() {
            Ok(PathType::File(pathbuf))
        } else {
            Err(PathTypeError::Unknown)
        }
    }

    pub fn file_path(self) -> Result<PathBuf, PathTypeError> {
        if let PathType::File(p) = self {
            Ok(p)
        } else {
            Err(PathTypeError::NotAFile)
        }
    }
}

impl Default for PathType {
    fn default() -> Self {
        PathType::File(PathBuf::from("./wtools_output.json"))
    }
}
