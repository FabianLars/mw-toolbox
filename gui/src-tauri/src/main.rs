#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
};

use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde_json::Value;
use tokio::sync::Mutex as AsyncMutex;

use mw_tools::WikiClient;

mod cmd;

// There is nothing we can do if init fails, so let's panic in the disco.
static CLIENT: Lazy<AsyncMutex<WikiClient>> =
    Lazy::new(|| AsyncMutex::new(WikiClient::new().unwrap()));

static CANCEL_EDIT: AtomicBool = AtomicBool::new(false);
static CANCEL_UPLOAD: AtomicBool = AtomicBool::new(false);

fn main() {
    pretty_env_logger::init();

    tauri::Builder::default()
        .on_page_load(|window, _| {
            window.listen("cancel-upload", move |_| {
                CANCEL_UPLOAD.store(true, Ordering::Relaxed)
            });
            window.listen("cancel-autoedit", move |_| {
                CANCEL_EDIT.store(true, Ordering::Relaxed)
            });
        })
        .manage(Mutex::new(HashMap::<String, Value>::new()))
        .invoke_handler(tauri::generate_handler![
            cmd::cache_get,
            cmd::cache_set,
            cmd::delete,
            cmd::download,
            cmd::edit,
            cmd::auto_edit,
            cmd::get_page,
            cmd::init,
            cmd::list,
            cmd::login,
            cmd::logout,
            cmd::r#move,
            cmd::purge,
            cmd::update_profile_store,
            cmd::upload
        ])
        .run(tauri::generate_context!())
        .expect("error while running application");
}
