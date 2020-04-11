use std::{error::Error, fmt};

// TODO: Actually use these
#[derive(Debug, Clone)]
pub enum WtoolsError {
    DeleteError(String),
    ListError(String),
    RenameError(String),
    UpdateError(String),
    UploadError(String),
    Unspecified,
}

impl fmt::Display for WtoolsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WtoolsError::ListError(reason) => write!(f, "{}", reason),
            _ => write!(f, "Unspecified custom error from wtools occured!"),
        }
    }
}

impl Error for WtoolsError {}
