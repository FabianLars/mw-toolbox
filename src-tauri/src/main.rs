#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use serde::Serialize;
use tauri::event::emit;
use wtools::{api, PathType, WikiClient};

#[derive(Serialize)]
struct Response<'a> {
    message: &'a str,
}

//#[async_std::main]
fn main() {
    pretty_env_logger::init();
    let mut client = WikiClient::new().unwrap();
    tauri::AppBuilder::new()
        .invoke_handler(move |_webview, arg| {
            use cmd::Cmd::*;
            match serde_json::from_str(arg) {
                Err(e) => Err(e.to_string()),
                Ok(command) => {
                    match command {
                        // definitions for your custom commands from Cmd here
                        Login {
                            loginname,
                            password,
                            url,
                            callback,
                            error,
                        } => {
                            // This blocks the ui, but works best for now
                            // TODO: Handle malformed url rejections
                            client = WikiClient::from(url).unwrap();
                            client.credentials(loginname, password);
                            async_std::task::block_on(client.login()).unwrap();
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
}
