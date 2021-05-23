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
use serde::{Deserialize, Serialize};
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SavedState {
    wikiurl: String,
    loginname: String,
    password: String,
    #[serde(rename(serialize = "isPersistent"))]
    is_persistent: bool,
}

impl SavedState {
    async fn save(self) -> Result<(), anyhow::Error> {
        storage::save_secure("b9c95dde", self).await
    }

    async fn load() -> SavedState {
        storage::load_secure::<SavedState>("b9c95dde")
            .await
            .unwrap_or_default()
    }
}
