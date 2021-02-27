use thiserror::Error;

#[derive(Error, Debug)]
pub enum ToolsError {
    #[error(transparent)]
    MediaWikiError(#[from] super::response::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /* #[error("Building inner Reqwest Client failed.")]
    BuildFailed { source: reqwest::Error }, */
    #[error("Error executing request.")]
    RequestFailed { source: reqwest::Error },
    #[error("Error extracting body as text.")]
    TextConversionFailed { source: reqwest::Error },
    #[error("Error extracting body as json.")]
    JsonConversionFailed { source: reqwest::Error },
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error("Couldn't extract token from response json: {0}")]
    TokenNotFound(String),
    #[error("Login failed! API returned: '{0}'")]
    LoginFailed(String),

    #[error("Invalid Input: {0}")]
    InvalidInput(String),
    #[error("Provided input is empty")]
    EmptyInput,
}
