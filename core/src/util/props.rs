use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Props {
    pub path: PathType,
    pub parameter: Option<String>,
    pub loginname: String,
    pub loginpassword: String,
}

impl Props {
    pub fn new(path: Option<PathBuf>, parameter: Option<String>, loginname: String, loginpassword: String) -> Self {
        let p = match path {
            None => PathType::default(),
            Some(x) => PathType::new(x),
        };
        Self {
            path: p,
            parameter,
            loginname,
            loginpassword,
        }
    }

    pub fn from_pathtype(pathtype: PathType, parameter: Option<String>, loginname: String, loginpassword: String) -> Self {
        Self {
            path: pathtype,
            parameter,
            loginname,
            loginpassword,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PathType {
    File(PathBuf),
    Files(Vec<PathBuf>),
    Folder(PathBuf),
}

impl Default for PathType {
    fn default() -> Self {
        PathType::File(PathBuf::from("./wtools_output.json"))
    }
}

impl PathType {
    pub fn new(pathbuf: PathBuf) -> Self {
        if std::fs::metadata(&pathbuf).unwrap().is_dir() {
            PathType::Folder(pathbuf)
        } else if std::fs::metadata(&pathbuf).unwrap().is_file() {
            PathType::File(pathbuf)
        } else {
            panic!("weird error");
        }
    }

    pub fn file_path(self) -> PathBuf {
        if let PathType::File(p) = self { p } else { panic!("Not a file") }
    }
}