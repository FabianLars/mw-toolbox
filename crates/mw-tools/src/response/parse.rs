use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Parse {
    pub(crate) parse: Response,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Response {
    pub(crate) title: String,
    pub(crate) wikitext: String,
}
