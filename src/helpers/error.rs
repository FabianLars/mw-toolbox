use std::{error::Error, fmt};

#[derive(Debug)]
pub enum WtoolsError {
    ListError(String),
    Other,
}

impl fmt::Display for WtoolsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WtoolsError::ListError(reason) => write!(f, "{}", reason),
            _ => write!(f, "Random custom error from wtools occured!"),
        }
    }
}

impl Error for WtoolsError {}
