use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "lowercase")]
pub(crate) enum Login {
    Login {
        login: Success,
    },
    LoginError {
        #[serde(rename = "login")]
        error: Failure,
    },
    // This can't actually happen
    Error {
        errors: Vec<super::Error>,
    },
    // This can't actually happen
    Warnings {
        warnings: Vec<super::Error>,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) struct Success {
    pub(crate) result: String,
    pub(crate) lguserid: u64,
    pub(crate) lgusername: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Failure {
    #[serde(rename = "result")]
    pub(crate) reason: String,
}
