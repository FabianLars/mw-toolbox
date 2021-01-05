#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use std::sync::Arc;

use serde::Serialize;
use tauri::{event::emit, Result};
use wtools::{api, PathType, WikiClient};

#[derive(Serialize)]
struct Response<'a> {
    message: &'a str,
}

#[derive(Serialize)]
struct LoggedIn<'a> {
    username: &'a str,
    url: &'a str,
}

fn main() {
    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());

    pretty_env_logger::init();

    let mut state = rt.block_on(SavedState::load()).unwrap();

    let mut client = WikiClient::new().unwrap();
    tauri::AppBuilder::new()
        //.setup(|webview, _source| {})
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
                        // TODO: Delete previously saved password if is_persistent is false
                        Login {
                            loginname,
                            password,
                            wikiurl,
                            is_persistent,
                            callback,
                            error,
                        } => {
                            // This blocks the ui, but works best for now
                            // TODO: Handle malformed url rejections
                            client = WikiClient::from(&wikiurl).unwrap();
                            client.credentials(&loginname, &password);
                            rt.block_on(client.login()).unwrap();
                            state = SavedState {
                                wikiurl: wikiurl.clone(),
                                loginname: loginname.clone(),
                                password: password.clone(),
                                is_persistent,
                            };
                            if is_persistent {
                                rt.block_on(
                                    SavedState {
                                        wikiurl,
                                        loginname,
                                        password,
                                        is_persistent,
                                    }
                                    .save(),
                                )
                                .unwrap()
                            } else {
                                rt.block_on(
                                    SavedState {
                                        wikiurl,
                                        loginname: String::new(),
                                        password: String::new(),
                                        is_persistent,
                                    }
                                    .save(),
                                )
                                .unwrap()
                            }
                            // TODO: remove this emit in favor of success_callback or sth
                            emit(
                                &mut _webview.as_mut(),
                                "loggedin",
                                Some(LoggedIn {
                                    username: &state.loginname,
                                    url: &state.wikiurl,
                                }),
                            )
                            .unwrap();
                        }
                        List { callback, error } => {
                            let client = client.clone();
                            let handle = rt.clone();
                            tauri::execute_promise(
                                _webview,
                                move || {
                                    println!(
                                        "{:?}",
                                        handle
                                            .block_on(api::delete::delete_pages(client, &["Test"]))
                                            .unwrap()
                                    );
                                    Ok(Response {
                                        message: "ICH WEINE immernoch",
                                    })
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
    //});
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
