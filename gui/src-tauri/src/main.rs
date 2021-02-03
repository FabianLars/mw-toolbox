#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use mw_toolbox::{api, WikiClient};
use serde::Serialize;
use tauri::Result;

#[derive(Serialize)]
struct Response<'a> {
    message: &'a str,
}

#[derive(Serialize)]
struct ListResponse {
    list: Vec<String>,
}

#[derive(Serialize)]
struct LoginResponse {
    username: String,
    url: String,
}

#[derive(Serialize)]
struct UploadDialogResponse {
    files: Vec<String>,
}

#[derive(Serialize)]
struct GetPageResponse {
    content: String,
}

fn main() {
    pretty_env_logger::init();

    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let files: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::new()));
    let files_handle = files.clone();
    let mut state = rt.block_on(SavedState::load()).unwrap();
    let mut client = WikiClient::new().unwrap();

    tauri::AppBuilder::new()
        .setup(move |_webview, _source| {
            let f = files_handle.clone();
            tauri::event::listen("clear-files", move |_| {
                if let Ok(mut x) = f.lock() {
                    x.clear();
                }
            })
        })
        .invoke_handler(move |_webview, arg| {
            use cmd::Cmd::*;
            let state_inner = state.clone();
            match serde_json::from_str(arg) {
                Err(e) => Err(e.to_string()),
                Ok(command) => {
                    match command {
                        Init { callback, error } => tauri::execute_promise(
                            _webview,
                            move || Ok(state_inner),
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
                            client.url(&wikiurl);
                            client.credentials(&loginname, &password);
                            let callback_val = LoginResponse {
                                username: loginname.clone(),
                                url: wikiurl.clone(),
                            };
                            // This kinda blocks the ui, but works best for now
                            let client_res = rt.block_on(client.login());

                            if client_res.is_ok() {
                                state = SavedState {
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
                        Delete {
                            pages,
                            callback,
                            error,
                        } => {
                            let client = client.clone();
                            let handle = rt.clone();
                            tauri::execute_promise(
                                _webview,
                                move || match handle
                                    .block_on(api::delete::delete(&client, &pages[..]))
                                {
                                    Ok(_) => Ok(Response {
                                        message: "Delete successful!",
                                    }),
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
                            let client = client.clone();
                            let handle = rt.clone();

                            tauri::execute_promise(
                                _webview,
                                move || match handle
                                    .block_on(api::download::download(client, &files))
                                {
                                    Ok(_) => Ok(Response {
                                        message: "Download successful! Check your download folder.",
                                    }),
                                    Err(err) => Err(err.into()),
                                },
                                callback,
                                error,
                            )
                        }
                        Edit {
                            title,
                            content,
                            callback,
                            error,
                        } => {
                            let client = client.clone();
                            let handle = rt.clone();

                            tauri::execute_promise(
                                _webview,
                                move || match handle
                                    .block_on(api::edit::edit(&client, title, content))
                                {
                                    Ok(s) => Ok(s),
                                    Err(err) => Err(err.into()),
                                },
                                callback,
                                error,
                            )
                        }
                        GetPage {
                            page,
                            callback,
                            error,
                        } => {
                            let client = client.clone();
                            let handle = rt.clone();

                            tauri::execute_promise(
                                _webview,
                                move || match handle
                                    .block_on(api::parse::get_page_content(&client, page))
                                {
                                    Ok(s) => Ok(GetPageResponse { content: s }),
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
                            let client = client.clone();
                            let handle = rt.clone();

                            tauri::execute_promise(
                                _webview,
                                move || {
                                    let res = match listtype.as_str() {
                                        "allimages" => {
                                            handle.block_on(api::list::allimages(&client))
                                        }
                                        "allpages" => handle.block_on(api::list::allpages(
                                            &client,
                                            param.as_deref(),
                                        )),
                                        "alllinks" => handle.block_on(api::list::alllinks(&client)),
                                        "allcategories" => {
                                            handle.block_on(api::list::allcategories(&client))
                                        }
                                        "backlinks" => handle.block_on(api::list::backlinks(
                                            &client,
                                            param.as_deref(),
                                        )),
                                        "categorymembers" => handle.block_on(
                                            api::list::categorymembers(&client, param.as_deref()),
                                        ),
                                        "embeddedin" => handle.block_on(api::list::embeddedin(
                                            &client,
                                            param.as_deref(),
                                        )),
                                        "imageusage" => handle.block_on(api::list::imageusage(
                                            &client,
                                            param.as_deref(),
                                        )),
                                        "search" => handle
                                            .block_on(api::list::search(&client, param.as_deref())),
                                        "protectedtitles" => {
                                            handle.block_on(api::list::protectedtitles(&client))
                                        }
                                        "querypage" => handle.block_on(api::list::querypage(
                                            &client,
                                            param.as_deref(),
                                        )),
                                        "allinfoboxes" => {
                                            handle.block_on(api::list::allinfoboxes(&client))
                                        }
                                        _ => handle.block_on(api::list::allimages(&client)),
                                    };
                                    match res {
                                        Ok(list) => Ok(ListResponse { list }),
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
                            let client = client.clone();
                            let handle = rt.clone();

                            tauri::execute_promise(
                                _webview,
                                move || match handle.block_on(api::rename::rename(
                                    &client,
                                    from,
                                    Some(api::rename::Destination::Plain(to)),
                                    None,
                                    None,
                                )) {
                                    Ok(_) => Ok(Response {
                                        message: "Successfully moved pages.",
                                    }),
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
                            let client = client.clone();
                            let handle = rt.clone();

                            tauri::execute_promise(
                                _webview,
                                move || match match is_nulledit {
                                    true => handle.block_on(api::purge::purge(
                                        &client,
                                        &pages[..],
                                        true,
                                    )),
                                    false => {
                                        handle.block_on(api::edit::nulledit(&client, &pages[..]))
                                    }
                                } {
                                    Ok(_) => Ok(Response {
                                        message: "Purge successful!",
                                    }),
                                    Err(err) => Err(err.into()),
                                },
                                callback,
                                error,
                            )
                        }
                        UploadDialog { callback, error } => {
                            let files = files.clone();
                            tauri::execute_promise(
                                _webview,
                                move || {
                                    let result =
                                        native_dialog::FileDialog::new().show_open_multiple_file();
                                    match result {
                                        Ok(f) => {
                                            let arr: Vec<String> = match files.lock() {
                                                Ok(mut x) => {
                                                    *x = f;
                                                    x.iter()
                                                        .map(|x| x.display().to_string())
                                                        .collect()
                                                }
                                                Err(_) => panic!("Mutex poisoned"),
                                            };
                                            Ok(UploadDialogResponse { files: arr })
                                        }
                                        Err(e) => Err(e.into()),
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
                            let client = client.clone();
                            let handle = rt.clone();
                            let files = files.clone().lock().expect("Mutex poisoned").clone();
                            // WebView2 forces us to use Mutex & execute_promise instead of sth simpler
                            // Otherwise, every file dialog crate results in the same type of crash
                            // Linux & Mac (& WebView1?) seem to be uneffected
                            tauri::execute_promise(
                                _webview,
                                move || match handle
                                    .block_on(api::upload::upload(&client, files, text))
                                {
                                    Ok(_) => Ok(Response {
                                        message: "Upload successful!",
                                    }),
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

#[derive(Clone, Debug, Serialize)]
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
