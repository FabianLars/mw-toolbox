use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init {
        callback: String,
        error: String,
    },
    Login {
        loginname: String,
        password: String,
        wikiurl: String,
        is_persistent: bool,
        callback: String,
        error: String,
    },
    List {
        callback: String,
        error: String,
    },
}
