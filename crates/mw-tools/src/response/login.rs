use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub(crate) struct Login {
    pub(crate) login: Response,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub(crate) struct Response {
    pub(crate) result: String,
    pub(crate) reason: Option<super::Error>,
    pub(crate) lguserid: u64,
    pub(crate) lgusername: String,
}
