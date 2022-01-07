use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Upload {
    pub(crate) upload: Response,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Response {
    pub(crate) result: String,
    #[allow(dead_code)]
    pub(crate) filename: String,
}
