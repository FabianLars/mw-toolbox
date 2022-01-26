#[derive(Debug, thiserror::Error)]
pub enum Error {
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

impl Error {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Error::MediaWikiApi(_) => "MediaWikiaApi",
            Error::TokenNotFound(_) => "TokenNotFound",
            Error::LoginFailed(_) => "LoginFailed",
            Error::IoError(_) => "IoError",
            Error::Timeout(_) => "Timeout",
            Error::StatusCode(_) => "StatusCode",
            Error::RequestFailed(_) => "RequestFailed",
            Error::ParsingFailed(_) => "ParsingFailed",
            Error::HttpClient(_) => "HttpClient",
            Error::InvalidInput(_) => "InvalidInput",
            Error::Other(_) => "Other",
        }
    }
}

impl From<reqwest::Error> for Error {
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

impl serde::Serialize for Error {
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
