#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use std::sync::Arc;

use api::purge;
use serde::Serialize;
use tauri::Result;
use wtools::{api, WikiClient};

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

fn main() {
    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());

    pretty_env_logger::init();

    let mut state = rt.block_on(SavedState::load()).unwrap();

    let mut client = WikiClient::new().unwrap();
    tauri::AppBuilder::new()
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
                            // This blocks the ui, but works best for now
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
                                    .block_on(api::delete::delete_pages(&client, &pages[..]))
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
