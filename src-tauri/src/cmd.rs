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
    Delete {
        pages: Vec<String>,
        callback: String,
        error: String,
    },
    List {
        listtype: String,
        param: Option<String>,
        callback: String,
        error: String,
    },
    Move {
        from: Vec<String>,
        to: Vec<String>,
        callback: String,
        error: String,
    },
    Purge {
        is_nulledit: bool,
        pages: Vec<String>,
        callback: String,
        error: String,
    },
    UploadDialog {
        callback: String,
        error: String,
    },
    Upload {
        text: Option<String>,
        callback: String,
        error: String,
    },
}
