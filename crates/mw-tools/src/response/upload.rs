use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum Upload {
    Succes { upload: Response },
    Failure { errors: Vec<super::Error> },
}

#[derive(Debug, Deserialize)]
pub(crate) struct Response {
    pub(crate) result: String,
    pub(crate) filename: String,
}
