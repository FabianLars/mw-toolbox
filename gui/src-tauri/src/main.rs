#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, path::PathBuf, sync::atomic::AtomicBool};

use once_cell::sync::Lazy;
use serde::Serialize;
use tokio::sync::Mutex;

use mw_tools::WikiClient;

mod cmd;

// There is nothing we can do if init fails, so let's panic in the disco.
static CLIENT: Lazy<Mutex<WikiClient>> = Lazy::new(|| Mutex::new(WikiClient::new().unwrap()));
static CACHE: Lazy<Mutex<HashMap<String, serde_json::Value>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static FILES_HELPER: Lazy<Mutex<Vec<PathBuf>>> = Lazy::new(|| Mutex::new(Vec::new()));
static SAVED_STATE: Lazy<Mutex<SavedState>> = Lazy::new(|| Mutex::new(SavedState::load()));

fn main() {
    pretty_env_logger::init();

    tauri::Builder::default()
        .manage(std::sync::Mutex::new(cmd::TestState {
            val: 0,
            st: "test".to_string(),
        }))
        .invoke_handler(tauri::generate_handler![
            cmd::cache_get,
            cmd::cache_set,
            cmd::clear_files,
            cmd::delete,
            cmd::download,
            cmd::edit,
            cmd::get_page,
            cmd::init,
            cmd::list,
            cmd::login,
            cmd::logout,
            cmd::r#move,
            cmd::purge,
            cmd::upload_dialog,
            cmd::upload,
            cmd::test
        ])
        .run(tauri::generate_context!())
        .expect("error while running application");
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SavedState {
    wikiurl: String,
    loginname: String,
    password: String,
    #[serde(rename(serialize = "isPersistent"))]
    is_persistent: bool,
}

impl SavedState {
    async fn save_async(self) -> Result<(), anyhow::Error> {
        use storage::*;

        insert_multiple(&[
            ("b9c95dde", encrypt(&self.loginname)?.as_slice()),
            ("d7f0942b", encrypt(&self.password)?.as_slice()),
            ("wikiurl", self.wikiurl.as_bytes()),
            ("is_persistent", self.is_persistent.to_string().as_bytes()),
        ])
        .await
    }

    fn load() -> SavedState {
        use storage::blocking::*;

        let loginname = get_secure("b9c95dde").unwrap_or_default();
        let password = get_secure("d7f0942b").unwrap_or_default();
        let wikiurl = get("wikiurl").unwrap_or_default();
        let is_persistent = get("is_persistent")
            .unwrap_or_else(|_| String::from("false"))
            .parse::<bool>()
            .unwrap_or(false);

        Self {
            wikiurl,
            loginname,
            password,
            is_persistent,
        }
    }
}
