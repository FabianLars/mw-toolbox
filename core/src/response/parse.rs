use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum Parse {
    Succes { parse: Response },
    Failure { errors: Vec<super::Error> },
}

#[derive(Debug, Deserialize)]
pub(crate) struct Response {
    pub(crate) title: String,
    pub(crate) wikitext: String,
}
