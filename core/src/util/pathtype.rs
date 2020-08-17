use async_std::path::PathBuf;

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

    pub fn from(pathbuf: PathBuf) -> Self {
        if std::fs::metadata(&pathbuf).unwrap().is_dir() {
            PathType::Folder(pathbuf)
        } else if std::fs::metadata(&pathbuf).unwrap().is_file() {
            PathType::File(pathbuf)
        } else {
            panic!("weird error");
        }
    }

    pub fn file_path(self) -> PathBuf {
        if let PathType::File(p) = self {
            p
        } else {
            panic!("Not a file")
        }
    }
}

impl Default for PathType {
    fn default() -> Self {
        PathType::File(PathBuf::from("./wtools_output.json"))
    }
}
