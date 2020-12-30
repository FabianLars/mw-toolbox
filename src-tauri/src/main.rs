#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use serde::Serialize;
use tauri::{event::emit, Result};
use wtools::{api, PathType, WikiClient};

#[derive(Serialize)]
struct Response<'a> {
    message: &'a str,
}

fn main() {
    async_std::task::block_on(async {
        pretty_env_logger::init();

        let state = SavedState::load().await.unwrap();

        let mut client = WikiClient::new().unwrap();
        tauri::AppBuilder::new()
            //.setup(|webview, _source| {})
            .invoke_handler(move |_webview, arg| {
                use cmd::Cmd::*;
                let state = state.clone();
                match serde_json::from_str(arg) {
                    Err(e) => Err(e.to_string()),
                    Ok(command) => {
                        match command {
                            Init { callback, error } => tauri::execute_promise(
                                _webview,
                                move || async_std::task::block_on(SavedState::load()),
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
                                async_std::task::block_on(client.login()).unwrap();
                                if is_persistent {
                                    async_std::task::block_on(
                                        SavedState {
                                            wikiurl,
                                            loginname,
                                            password,
                                            is_persistent,
                                        }
                                        .save(),
                                    )
                                    .unwrap()
                                }
                                // TODO: remove this emit in favor of success_callback
                                emit(
                                    &mut _webview.as_mut(),
                                    "loggedin",
                                    Some("sollte logged in sein."),
                                )
                                .unwrap();
                            }
                            List { callback, error } => {
                                let client2 = client.clone();
                                tauri::execute_promise(
                                    _webview,
                                    move || {
                                        println!(
                                            "{:?}",
                                            async_std::task::block_on(api::delete::delete_pages(
                                                client2,
                                                &["Test"]
                                            ))
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
    });
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
