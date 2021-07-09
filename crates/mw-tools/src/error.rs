use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolsError {
    /// MediaWiki API Errors
    #[error(transparent)]
    MediaWikiApi(#[from] super::response::Error),
    #[error("Couldn't extract token from response json: \"{0}\"")]
    TokenNotFound(String),
    #[error("Login failed! API returned: \"{0}\"")]
    LoginFailed(String),

    /// IOError
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Reqwest Errors
    #[error("Request timed out: \"{0}\"")]
    Timeout(String),
    #[error("Request returned non-200 status: \"{0}\"")]
    StatusCode(String),
    #[error("Error executing request: \"{0}\"")]
    RequestFailed(String),
    #[error("Error parsing body as json: \"{0}\"")]
    ParsingFailed(String),
    /// Catch-all reqwest errors
    #[error("HTTP Client Error: \"{0}\"")]
    HttpClient(String),
    /* /// Tauri Errors
    #[error("Tauri error: {0}")]
    TauriError(String), */
    #[error("Invalid Input: \"{0}\"")]
    InvalidInput(String),

    #[error("{0}")]
    Other(String),
}

impl ToolsError {
    pub const fn code(&self) -> &'static str {
        match self {
            ToolsError::MediaWikiApi(_) => "MediaWikiaApi",
            ToolsError::TokenNotFound(_) => "TokenNotFound",
            ToolsError::LoginFailed(_) => "LoginFailed",
            ToolsError::IoError(_) => "IoError",
            ToolsError::Timeout(_) => "Timeout",
            ToolsError::StatusCode(_) => "StatusCode",
            ToolsError::RequestFailed(_) => "RequestFailed",
            ToolsError::ParsingFailed(_) => "ParsingFailed",
            ToolsError::HttpClient(_) => "HttpClient",
            ToolsError::InvalidInput(_) => "InvalidInput",
            ToolsError::Other(_) => "Other",
        }
    }
}

impl From<reqwest::Error> for ToolsError {
    fn from(source: reqwest::Error) -> Self {
        if source.is_timeout() {
            Self::Timeout(source.to_string())
        } else if source.is_status() {
            Self::StatusCode(source.to_string())
        } else if source.is_decode() {
            Self::ParsingFailed(source.to_string())
        } else if source.is_request() {
            Self::RequestFailed(source.to_string())
        } else {
            Self::HttpClient(source.to_string())
        }
    }
}

impl Serialize for ToolsError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Error", 2)?;
        state.serialize_field("code", &self.code())?;
        state.serialize_field("description", &self.to_string())?;
        state.end()
    }
}
