pub(crate) mod list;
pub(crate) mod login;
pub(crate) mod token;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Error {
    pub(crate) code: String,
    pub(crate) text: String,
    pub(crate) module: String,
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
