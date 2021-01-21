use serde::Deserialize;

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
