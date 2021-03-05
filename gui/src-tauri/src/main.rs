#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use mw_tools::{api, WikiClient};
use once_cell::sync::Lazy;
use serde::Serialize;
use tauri::Result;

#[derive(Serialize)]
pub struct LoginResponse {
    username: String,
    url: String,
}

// There is nothing we can do if init fails, so let's panic in the disco.
static CLIENT: Lazy<Mutex<WikiClient>> = Lazy::new(|| Mutex::new(WikiClient::new().unwrap()));
static CACHE: Lazy<Mutex<HashMap<String, serde_json::Value>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static FILES_HELPER: Lazy<Mutex<Vec<PathBuf>>> = Lazy::new(|| Mutex::new(Vec::new()));
static SAVED_STATE: Lazy<Mutex<SavedState>> = Lazy::new(|| Mutex::new(SavedState::default()));

fn main() {
    pretty_env_logger::init();

    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
    *SAVED_STATE.lock().expect("Couldn't lock State Mutex") =
        rt.block_on(SavedState::load()).unwrap();

    tauri::AppBuilder::new()
        .setup(move |_webview, _source| {
            tauri::event::listen("clear-files", move |_| {
                if let Ok(mut x) = FILES_HELPER.lock() {
                    x.clear();
                }
            })
        })
        .invoke_handler(move |_webview, arg| {
            use cmd::Cmd::*;
            match serde_json::from_str(arg) {
                Err(e) => Err(e.to_string()),
                Ok(command) => {
                    match command {
                        Init { callback, error } => tauri::execute_promise(
                            _webview,
                            move || {
                                Ok(SAVED_STATE
                                    .lock()
                                    .expect("Couldn't lock State Mutex")
                                    .clone())
                            },
                            callback,
                            error,
                        ),
                        Login {
                            loginname,
                            password,
                            wikiurl,
                            is_persistent,
                            callback,
                            error,
                        } => {
                            let mut client = CLIENT.lock().expect("Couldn't lock Client Mutex");
                            client.url(&wikiurl);
                            client.credentials(&loginname, &password);
                            let callback_val = LoginResponse {
                                username: loginname.clone(),
                                url: wikiurl.clone(),
                            };
                            // This kinda blocks the ui, but works best for now
                            let client_res = rt.block_on(client.login());

                            if client_res.is_ok() {
                                *SAVED_STATE.lock().expect("Couldn't lock State Mutex") =
                                    SavedState {
                                        wikiurl: wikiurl.clone(),
                                        loginname: loginname.clone(),
                                        password: password.clone(),
                                        is_persistent,
                                    }
                            }

                            let handle = rt.clone();
                            let loginname = match is_persistent {
                                true => loginname,
                                false => String::new(),
                            };
                            let password = match is_persistent {
                                true => password,
                                false => String::new(),
                            };
                            tauri::execute_promise(
                                _webview,
                                move || {
                                    if let Err(e) = client_res {
                                        return Err(e.into());
                                    }
                                    match handle.block_on(
                                        SavedState {
                                            wikiurl,
                                            loginname,
                                            password,
                                            is_persistent,
                                        }
                                        .save(),
                                    ) {
                                        Ok(_) => Ok(callback_val),
                                        Err(err) => Err(err),
                                    }
                                },
                                callback,
                                error,
                            )
                        }
                        Logout { callback, error } => {
                            let handle = rt.clone();
                            tauri::execute_promise(
                                _webview,
                                move || {
                                    handle
                                        .block_on(
                                            CLIENT
                                                .lock()
                                                .expect("Couldn't lock Client Mutex")
                                                .logout(),
                                        )
                                        .map_err(|e| e.into())
                                },
                                callback,
                                error,
                            )
                        }
                        CacheGet {
                            key,
                            callback,
                            error,
                        } => {
                            if let Some(v) =
                                CACHE.lock().expect("Couldn't lock Cache Mutex").get(&key)
                            {
                                let v = v.to_owned();
                                tauri::execute_promise(_webview, move || Ok(v), callback, error)
                            }
                        }
                        CacheSet {
                            key,
                            value,
                            callback,
                            error,
                        } => {
                            let updated = CACHE
                                .lock()
                                .expect("Couldn't lock Cache Mutex")
                                .insert(key, value)
                                .is_some();
                            tauri::execute_promise(_webview, move || Ok(updated), callback, error)
                        }
                        Delete {
                            pages,
                            callback,
                            error,
                        } => {
                            let handle = rt.clone();
                            tauri::execute_promise(
                                _webview,
                                move || match handle.block_on(api::delete::delete(
                                    &*CLIENT.lock().expect("Couldn't lock Client Mutex"),
                                    &pages[..],
                                )) {
                                    Ok(_) => Ok(()),
                                    Err(err) => Err(err.into()),
                                },
                                callback,
                                error,
                            )
                        }
                        Download {
                            files,
                            callback,
                            error,
                        } => {
                            let handle = rt.clone();

                            tauri::execute_promise(
                                _webview,
                                move || match handle.block_on(api::download::download(
                                    &*CLIENT.lock().expect("Couldn't lock Client Mutex"),
                                    &files,
                                )) {
                                    Ok(_) => Ok(()),
                                    Err(err) => Err(err.into()),
                                },
                                callback,
                                error,
                            )
                        }
                        Edit {
                            title,
                            content,
                            summary,
                            callback,
                            error,
                        } => {
                            let handle = rt.clone();

                            tauri::execute_promise(
                                _webview,
                                move || match handle.block_on(api::edit::edit(
                                    &*CLIENT.lock().expect("Couldn't lock Client Mutex"),
                                    title,
                                    content,
                                    Some(summary),
                                )) {
                                    Ok(s) => Ok(s),
                                    Err(err) => Err(err.into()),
                                },
                                callback,
                                error,
                            )
                        }
                        GetPage {
                            page,
                            patterns,
                            callback,
                            error,
                        } => {
                            let handle = rt.clone();

                            tauri::execute_promise(
                                _webview,
                                move || match handle.block_on(api::parse::get_page_content(
                                    &*CLIENT.lock().expect("Couldn't lock Client Mutex"),
                                    page,
                                )) {
                                    Ok(s) => {
                                        let mut s = s;
                                        for pat in patterns {
                                            if !pat.find.is_empty() {
                                                if pat.is_regex {
                                                    let re = regex::Regex::new(&pat.find)?;
                                                    s = re
                                                        .replace_all(&s, pat.replace.as_str())
                                                        .to_string();
                                                } else {
                                                    s = s.replace(&pat.find, &pat.replace);
                                                }
                                            }
                                        }
                                        Ok(s)
                                    }
                                    Err(err) => Err(err.into()),
                                },
                                callback,
                                error,
                            )
                        }
                        List {
                            listtype,
                            param,
                            callback,
                            error,
                        } => {
                            let handle = rt.clone();

                            tauri::execute_promise(
                                _webview,
                                move || {
                                    let client = CLIENT.lock().expect("Couldn't lock Client Mutex");
                                    let res = match listtype.as_str() {
                                        "allimages" => {
                                            handle.block_on(api::list::allimages(&*client))
                                        }
                                        "allpages" => handle.block_on(api::list::allpages(
                                            &*client,
                                            param.as_deref(),
                                        )),
                                        "alllinks" => {
                                            handle.block_on(api::list::alllinks(&*client))
                                        }
                                        "allcategories" => {
                                            handle.block_on(api::list::allcategories(&*client))
                                        }
                                        "backlinks" => handle.block_on(api::list::backlinks(
                                            &*client,
                                            param.as_deref(),
                                        )),
                                        "categorymembers" => handle.block_on(
                                            api::list::categorymembers(&*client, param.as_deref()),
                                        ),
                                        "embeddedin" => handle.block_on(api::list::embeddedin(
                                            &*client,
                                            param.as_deref(),
                                        )),
                                        "imageusage" => handle.block_on(api::list::imageusage(
                                            &*client,
                                            param.as_deref(),
                                        )),
                                        "search" => handle.block_on(api::list::search(
                                            &*client,
                                            param.as_deref(),
                                        )),
                                        "protectedtitles" => {
                                            handle.block_on(api::list::protectedtitles(&*client))
                                        }
                                        "querypage" => handle.block_on(api::list::querypage(
                                            &*client,
                                            param.as_deref(),
                                        )),
                                        "allinfoboxes" => {
                                            handle.block_on(api::list::allinfoboxes(&*client))
                                        }
                                        _ => handle.block_on(api::list::allimages(&*client)),
                                    };
                                    match res {
                                        Ok(list) => Ok(list),
                                        Err(err) => Err(err.into()),
                                    }
                                },
                                callback,
                                error,
                            )
                        }
                        Move {
                            from,
                            to,
                            callback,
                            error,
                        } => {
                            let handle = rt.clone();

                            tauri::execute_promise(
                                _webview,
                                move || match handle.block_on(api::rename::rename(
                                    &*CLIENT.lock().expect("Couldn't lock Client Mutex"),
                                    from,
                                    Some(api::rename::Destination::Plain(to)),
                                    None,
                                    None,
                                )) {
                                    Ok(_) => Ok(()),
                                    Err(err) => Err(err.into()),
                                },
                                callback,
                                error,
                            )
                        }
                        Purge {
                            is_nulledit,
                            pages,
                            callback,
                            error,
                        } => {
                            let handle = rt.clone();

                            tauri::execute_promise(
                                _webview,
                                move || {
                                    let client = CLIENT.lock().expect("Couldn't lock Client Mutex");
                                    match match is_nulledit {
                                        true => handle
                                            .block_on(api::edit::nulledit(&*client, &pages[..])),
                                        false => handle.block_on(api::purge::purge(
                                            &*client,
                                            &pages[..],
                                            true,
                                        )),
                                    } {
                                        Ok(_) => Ok(()),
                                        Err(err) => Err(err.into()),
                                    }
                                },
                                callback,
                                error,
                            )
                        }
                        UploadDialog { callback, error } => {
                            let handle = rt.clone();
                            tauri::execute_promise(
                                _webview,
                                move || {
                                    let result =
                                        handle.block_on(rfd::AsyncFileDialog::new().pick_files());

                                    if let Some(selected_files) = result {
                                        let arr: Vec<String> = match FILES_HELPER.lock() {
                                            Ok(mut x) => {
                                                *x = selected_files
                                                    .iter()
                                                    .map(|f| f.path().to_path_buf())
                                                    .collect();
                                                x.iter().map(|x| x.display().to_string()).collect()
                                            }
                                            Err(_) => panic!("Mutex poisoned!"),
                                        };
                                        Ok(arr)
                                    } else {
                                        Err(anyhow::anyhow!("No files selected"))
                                    }
                                },
                                callback,
                                error,
                            )
                        }
                        Upload {
                            text,
                            callback,
                            error,
                        } => {
                            let handle = rt.clone();
                            tauri::execute_promise(
                                _webview,
                                move || match handle.block_on(api::upload::upload_multiple(
                                    &*CLIENT.lock().expect("Couldn't lock Client Mutex"),
                                    &*FILES_HELPER.lock().expect("Couldn't lock Files Mutex"),
                                    Some(text),
                                )) {
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(e.into()),
                                },
                                callback,
                                error,
                            )
                        }
                    }
                    Ok(())
                }
            }
        })
        .build()
        .run();
}

#[derive(Clone, Debug, Default, Serialize)]
struct SavedState {
    wikiurl: String,
    loginname: String,
    password: String,
    is_persistent: bool,
}

impl SavedState {
    async fn load() -> Result<SavedState> {
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

    async fn save(self) -> Result<()> {
        storage::insert_multiple(&[
            ("b9c95dde", storage::encrypt(&self.loginname)?.as_slice()),
            ("d7f0942b", storage::encrypt(&self.password)?.as_slice()),
            ("wikiurl", self.wikiurl.as_bytes()),
            ("is_persistent", self.is_persistent.to_string().as_bytes()),
        ])
        .await
    }
}
