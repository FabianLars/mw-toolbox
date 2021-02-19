use serde::Deserialize;

#[derive(Deserialize)]
pub struct FindReplace {
    pub find: Option<String>,
    pub replace: Option<String>,
    #[serde(rename = "isRegex")]
    pub is_regex: Option<bool>,
}

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

    /* Cache (used to be window.sessionstorage) */
    CacheGet {
        key: String,
        callback: String,
        error: String,
    },
    CacheSet {
        key: String,
        value: serde_json::Value,
        callback: String,
        error: String,
    },

    /* Page commands */
    Delete {
        pages: Vec<String>,
        callback: String,
        error: String,
    },
    Download {
        files: Vec<String>,
        callback: String,
        error: String,
    },
    Edit {
        title: String,
        content: String,
        summary: Option<String>,
        callback: String,
        error: String,
    },
    // Get Page content for Edit-tab
    GetPage {
        page: String,
        patterns: Vec<FindReplace>,
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
    // Call File dialog
    UploadDialog {
        callback: String,
        error: String,
    },
    // Upload rust-side stored files
    Upload {
        text: Option<String>,
        callback: String,
        error: String,
    },
}
