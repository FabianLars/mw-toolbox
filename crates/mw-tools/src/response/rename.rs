use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Rename {
    #[serde(rename = "move")]
    pub(crate) rename: Response,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Response {
    pub(crate) from: String,
    pub(crate) to: String,
}
