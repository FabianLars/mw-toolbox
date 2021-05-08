#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::Value;
use tauri::Manager;
use tokio::sync::Mutex;

use mw_tools::WikiClient;

mod cmd;

// There is nothing we can do if init fails, so let's panic in the disco.
static CLIENT: Lazy<Mutex<WikiClient>> = Lazy::new(|| Mutex::new(WikiClient::new().unwrap()));

fn main() {
    pretty_env_logger::init();

    tauri::Builder::default()
        .on_page_load(|window, _| {
            let cancel_upload = Arc::new(AtomicBool::new(false));
            window.manage(cancel_upload.clone());
            window.listen("cancel-upload", move |_| {
                cancel_upload.store(true, Ordering::Relaxed)
            });
        })
        .manage(parking_lot::Mutex::new(HashMap::<String, Value>::new()))
        .invoke_handler(tauri::generate_handler![
            cmd::cache_get,
            cmd::cache_set,
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
    #[serde(rename(serialize = "isPersistent"))]
    is_persistent: bool,
}

impl SavedState {
    async fn save(self) -> Result<(), anyhow::Error> {
        use storage::*;

        insert_multiple(&[
            ("b9c95dde", encrypt(&self.loginname)?.as_slice()),
            ("d7f0942b", encrypt(&self.password)?.as_slice()),
            ("wikiurl", self.wikiurl.as_bytes()),
            ("is_persistent", self.is_persistent.to_string().as_bytes()),
        ])
        .await
    }

    async fn load() -> SavedState {
        use storage::*;

        let loginname = get_secure("b9c95dde").await.unwrap_or_default();
        let password = get_secure("d7f0942b").await.unwrap_or_default();
        let wikiurl = get("wikiurl").await.unwrap_or_default();
        let is_persistent = get("is_persistent")
            .await
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
