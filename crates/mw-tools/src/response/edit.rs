use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum Edit {
    Succes { edit: Response },
    Failure { errors: Vec<super::Error> },
}

#[derive(Debug, Deserialize)]
pub(crate) struct Response {
    pub(crate) result: String,
    pub(crate) title: String,
    pub(crate) contentmodel: String,
}
