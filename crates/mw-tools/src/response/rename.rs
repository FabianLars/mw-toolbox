use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum Rename {
    Succes {
        #[serde(rename = "move")]
        moved: Moved,
    },
    Failure {
        errors: Vec<super::Error>,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) struct Moved {
    pub(crate) from: String,
    pub(crate) to: String,
}
