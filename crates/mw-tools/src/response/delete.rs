use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Delete {
    delete: Response,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Response {
    pub(crate) title: String,
    pub(crate) reason: String,
}
