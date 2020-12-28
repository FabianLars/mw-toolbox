use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    // your custom commands
    // multiple arguments are allowed
    // note that rename_all = "camelCase": you need to use "myCustomCommand" on JS
    Login {
        loginname: String,
        password: String,
        url: String,
        callback: String,
        error: String,
    },
    List {
        callback: String,
        error: String,
    },
}
