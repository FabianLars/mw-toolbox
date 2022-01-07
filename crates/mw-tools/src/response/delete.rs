use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Delete {
    #[allow(dead_code)]
    pub(crate) delete: Response,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Response {
    #[allow(dead_code)]
    pub(crate) title: String,
    #[allow(dead_code)]
    pub(crate) reason: String,
}
