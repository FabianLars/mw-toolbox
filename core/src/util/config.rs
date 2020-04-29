use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub path: PathType,
    pub parameter: Option<String>,
    pub loginname: String,
    pub loginpassword: String,
}

impl Config {
    pub fn new(loginname: String, loginpassword: String) -> Self {
        Self {
            path: PathType::default(),
            parameter: None,
            loginname,
            loginpassword,
        }
    }

    pub fn with_pathbuf(self, path: PathBuf) -> Config {
        Self {
            path: PathType::new(path),
            parameter: self.parameter,
            loginname: self.loginname,
            loginpassword: self.loginpassword,
        }
    }

    pub fn with_pathbuf_opt(self, path: Option<PathBuf>) -> Config {
        Self {
            path: match path {
                None => PathType::default(),
                Some(x) => PathType::new(x),
            },
            parameter: self.parameter,
            loginname: self.loginname,
            loginpassword: self.loginpassword,
        }
    }

    pub fn with_pathtype(self, path: PathType) -> Config {
        Self {
            path,
            parameter: self.parameter,
            loginname: self.loginname,
            loginpassword: self.loginpassword,
        }
    }

    pub fn with_pathstring(self, path: String) -> Config {
        Self {
            path: PathType::new(PathBuf::from(path)),
            parameter: self.parameter,
            loginname: self.loginname,
            loginpassword: self.loginpassword,
        }
    }

    pub fn with_parameter(self, parameter: Option<String>) -> Config {
        Self {
            path: self.path,
            parameter,
            loginname: self.loginname,
            loginpassword: self.loginpassword,
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