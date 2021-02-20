use serde::Deserialize;

#[derive(Deserialize)]
pub struct FindReplace {
    #[serde(default)]
    pub find: String,
    #[serde(default)]
    pub replace: String,
    #[serde(rename = "isRegex", default)]
    pub is_regex: bool,
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
        #[serde(default)]
        summary: String,
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
        #[serde(default)]
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
        #[serde(default)]
        text: String,
        callback: String,
        error: String,
    },
}
