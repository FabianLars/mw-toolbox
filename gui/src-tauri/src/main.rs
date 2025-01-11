#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
};

use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde_json::Value;
use tauri::Listener;
use tokio::sync::Mutex as AsyncMutex;

use mw_tools::Client;

mod cmd;

/// The Client for the wiki API, wrapped inside tokio's async Mutex to be usable in async tauri commands.
// There is nothing we can do if init fails, so let's panic in the disco.
static CLIENT: Lazy<AsyncMutex<Client>> = Lazy::new(|| AsyncMutex::new(Client::new("").unwrap()));

/// Global boolean to cancel automatic actions.
/// Currently this includes:
/// - editing in auto-save mode.
/// - file uploads (stopping inbetween).
static CANCEL_ACTION: AtomicBool = AtomicBool::new(false);

fn main() {
    pretty_env_logger::init();

    tauri::Builder::default()
        // TODO: Check for updates
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .on_page_load(|window, _| {
            // Listener to cancel actions, see above.
            window.listen("cancel-action", move |_| {
                CANCEL_ACTION.store(true, Ordering::Relaxed);
            });
            // add OS as global window wariable, because Windows is Windows i guess.
            // Used on Windows for:
            // TODO: Re-check this now that a few years have passed
            //  1. <select> elements need to be manually blurred when the window moves.
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running application");
}
