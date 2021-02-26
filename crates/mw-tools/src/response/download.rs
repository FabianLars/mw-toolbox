use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Imageinfo {
    pub(crate) query: Query,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Query {
    pub(crate) pages: Vec<Page>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Page {
    pub(crate) title: String,
    pub(crate) imageinfo: Option<Vec<Info>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Info {
    pub(crate) url: String,
}
