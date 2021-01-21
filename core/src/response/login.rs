use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Login {
    #[serde(rename = "login")]
    Login {
        result: String,
        lguserid: u64,
        lgusername: String,
    },
    Error {
        code: String,
        info: String,
    },
    Warnings {},
}

#[derive(Debug, Deserialize)]
pub(crate) struct Token {
    pub(crate) query: Query,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Query {
    pub(crate) tokens: Tokens,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Tokens {
    pub(crate) logintoken: Option<String>,
    pub(crate) csrftoken: Option<String>,
}
