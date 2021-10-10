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

use mw_tools::Client;

mod cmd;

#[cfg(target_os = "macos")]
mod menu;

/// The Client for the wiki API, wrapped inside tokio's async Mutex to be usable in async tauri commands.
// There is nothing we can do if init fails, so let's panic in the disco.
static CLIENT: Lazy<AsyncMutex<Client>> = Lazy::new(|| AsyncMutex::new(Client::new("").unwrap()));

/// Global boolean to cancel editing in auto-save mode.
static CANCEL_EDIT: AtomicBool = AtomicBool::new(false);
/// Global boolean to cancel file upload.
static CANCEL_UPLOAD: AtomicBool = AtomicBool::new(false);

fn main() {
    pretty_env_logger::init();

    let builder = tauri::Builder::default()
        .on_page_load(|window, _| {
            // Listener to cancel file upload.
            window.listen("cancel-upload", move |_| {
                CANCEL_UPLOAD.store(true, Ordering::Relaxed);
            });
            // Listener to cancel editing in auto-save mode.
            window.listen("cancel-autoedit", move |_| {
                CANCEL_EDIT.store(true, Ordering::Relaxed);
            });
            // add OS as global window wariable, because Windows is Windows i guess.
            // Used on Windows for:
            //  1. App Window needs to be resized before opening <select> dropdowns to fix its position.
            //  2. <input type="text"> and <select> elements need padding-bottom: 1px; for proper alignment.
            let _ = window.eval(&format!("window.OS='{}'", std::env::consts::OS));
        })
        // Using a HashMap for the application cache.
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
            cmd::rename,
            cmd::purge,
            cmd::update_profile_store,
            cmd::upload
        ]);

    // Needed on macOS to enable basic operations, like copy&past and select-all via keyboard shortcuts.
    #[cfg(target_os = "macos")]
    let builder = builder.menu(menu::menu());

    builder
        .run(tauri::generate_context!())
        .expect("error while running application");
}
