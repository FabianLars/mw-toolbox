use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Edit {
    pub edit: Response,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub result: String,
    pub title: String,
}
