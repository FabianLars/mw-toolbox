use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Upload {
    pub(crate) upload: Response,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Response {
    pub(crate) result: String,
    pub(crate) filename: String,
}
