use serde::{de::IgnoredAny, Deserialize, Serialize};
use thiserror::Error;

pub(crate) mod delete;
pub(crate) mod download;
pub mod edit;
pub(crate) mod list;
pub(crate) mod login;
pub(crate) mod parse;
pub(crate) mod rename;
pub(crate) mod token;
pub(crate) mod upload;

pub(crate) type Ignore = IgnoredAny;

#[derive(Debug, Deserialize, Error, Serialize)]
#[error("API returned an error: \"{code}\". Description: \"{description}\"")]
pub struct Error {
    pub(crate) code: String,
    #[serde(rename = "text")]
    pub(crate) description: String,
}

pub(crate) fn deserialize_string_from_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(i64),
        Float(f64),
    }

    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) => Ok(s),
        StringOrNumber::Number(i) => Ok(i.to_string()),
        StringOrNumber::Float(f) => Ok(f.to_string()),
    }
}
