pub(crate) mod login;
pub(crate) mod token;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Error {
    code: String,
    text: String,
    module: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Warning {
    code: String,
    text: String,
    module: String,
}
