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
static INIT: AtomicBool = AtomicBool::new(false);
static SAVED_STATE: Lazy<Mutex<SavedState>> = Lazy::new(|| Mutex::new(SavedState::default()));

fn main() {
    pretty_env_logger::init();

    tauri::Builder::default()
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
            cmd::upload
        ])
        .run(tauri::generate_context!())
        .expect("error while running application");
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SavedState {
    wikiurl: String,
    loginname: String,
    password: String,
    is_persistent: bool,
}

impl SavedState {
    async fn load() -> Result<SavedState, ()> {
        let loginname = storage::get_secure("b9c95dde").await.unwrap_or_default();
        let password = storage::get_secure("d7f0942b").await.unwrap_or_default();
        let wikiurl = storage::get("wikiurl").await.unwrap_or_default();
        let is_persistent = storage::get("is_persistent")
            .await
            .unwrap_or_else(|_| String::from("false"))
            .parse::<bool>()
            .unwrap_or(false);

        let s = Self {
            loginname,
            password,
            wikiurl,
            is_persistent,
        };
        Ok(s)
    }

    async fn save(self) -> Result<(), anyhow::Error> {
        storage::insert_multiple(&[
            ("b9c95dde", storage::encrypt(&self.loginname)?.as_slice()),
            ("d7f0942b", storage::encrypt(&self.password)?.as_slice()),
            ("wikiurl", self.wikiurl.as_bytes()),
            ("is_persistent", self.is_persistent.to_string().as_bytes()),
        ])
        .await
    }
}
