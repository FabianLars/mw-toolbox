use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Invalid Operation on serde_json::Value {0}")]
    InvalidJsonOperation(String),
    #[error(transparent)]
    ClientError(#[from] ClientError),
    #[error(transparent)]
    PathTypeError(#[from] PathTypeError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    UrlEncodingError(#[from] urlencoding::FromUrlEncodingError),
    #[error("Invalid Input: {0}")]
    InvalidInput(String),
    #[error("Provied input file is empty")]
    EmptyInput,
    #[error("unknown api error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Building inner Reqwest Client failed.")]
    BuildFailed { source: reqwest::Error },
    #[error("Error executing request.")]
    RequestFailed { source: reqwest::Error },
    #[error("Error extracting body as text.")]
    TextConversionFailed { source: reqwest::Error },
    #[error("Error extracting body as text.")]
    JsonConversionFailed { source: reqwest::Error },
    // Represents all other cases of Reqwests Errors
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("Couldn't extract token from response json: {0}")]
    TokenNotFound(String),
    #[error("malformed url: {0}")]
    MalformedUrl(String),
    #[error("Login failed! Reason: {0}")]
    LoginFailed(String),
}

#[derive(Error, Debug)]
pub enum PathTypeError {
    #[error("PathType is not a File")]
    NotAFile,
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
    #[error("Unknown PathTypeError. Wtf is happening")]
    Unknown,
}
