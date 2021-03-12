#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use mw_tools::{api, WikiClient};
use once_cell::sync::Lazy;
use serde::Serialize;
use tokio::sync::{mpsc, oneshot, Mutex};

#[derive(tauri::FromTauriContext)]
struct Context;

#[derive(Debug)]
pub enum ClientCommand {
    Delete(Vec<String>),
    Download(Vec<String>),
    Edit {
        title: String,
        content: String,
        summary: String,
    },
    GetPage {
        page: String,
        patterns: Vec<cmd::FindReplace>,
    },
    List {
        listtype: String,
        param: Option<String>,
    },
    Login {
        loginname: String,
        password: String,
        wikiurl: String,
        is_persistent: bool,
    },
    Logout,
    Move {
        from: Vec<String>,
        to: Vec<String>,
    },
    Purge,
}

#[derive(Debug)]
pub enum ClientResponse {
    Ok,
    String(String),
    Vec(Vec<String>),
    LoginResponse(cmd::LoginResponse),
    None,
    Error(String),
}

// There is nothing we can do if init fails, so let's panic in the disco.
static CLIENT: Lazy<Mutex<WikiClient>> = Lazy::new(|| Mutex::new(WikiClient::new().unwrap()));
static CACHE: Lazy<Mutex<HashMap<String, serde_json::Value>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static FILES_HELPER: Lazy<Mutex<Vec<PathBuf>>> = Lazy::new(|| Mutex::new(Vec::new()));
static SAVED_STATE: Lazy<Mutex<SavedState>> = Lazy::new(|| Mutex::new(SavedState::default()));
static MPSC: Lazy<
    Mutex<(
        mpsc::Sender<(ClientCommand, oneshot::Sender<ClientResponse>)>,
        mpsc::Receiver<(ClientCommand, oneshot::Sender<ClientResponse>)>,
    )>,
> = Lazy::new(|| {
    Mutex::new(mpsc::channel::<(
        ClientCommand,
        oneshot::Sender<ClientResponse>,
    )>(100))
});

fn main() {
    pretty_env_logger::init();
    let rt = tokio::runtime::Runtime::new().unwrap();

    tauri::AppBuilder::<Context>::new()
        .setup(|_| async move {
            let saved_state = SavedState::load().await.unwrap_or_default();
            *SAVED_STATE.lock().await = saved_state;

            /*
            let mut client = WikiClient::new().unwrap();

            while let Some((cmd, response)) = MPSC.lock().await.1.recv().await {
                match cmd {
                    ClientCommand::Delete(pages) => {
                        match api::delete::delete(&client, &pages[..]).await {
                            Ok(_) => response.send(ClientResponse::Ok).unwrap(),
                            Err(err) => response
                                .send(ClientResponse::Error(err.to_string()))
                                .unwrap(),
                        };
                    }
                    ClientCommand::Download(files) => {
                        match api::download::download(&client, &files).await {
                            Ok(_) => response.send(ClientResponse::Ok).unwrap(),
                            Err(err) => response
                                .send(ClientResponse::Error(err.to_string()))
                                .unwrap(),
                        };
                    }
                    ClientCommand::Edit {
                        title,
                        content,
                        summary,
                    } => {
                        match api::edit::edit(&client, title, content, Some(summary)).await {
                            Ok(_) => response.send(ClientResponse::Ok).unwrap(),
                            Err(err) => response
                                .send(ClientResponse::Error(err.to_string()))
                                .unwrap(),
                        };
                    }
                    ClientCommand::GetPage { page, patterns } => {
                        match api::parse::get_page_content(&client, page).await {
                            Ok(s) => {
                                let mut s = s;
                                for pat in patterns {
                                    if !pat.find.is_empty() {
                                        if pat.is_regex {
                                            match regex::Regex::new(&pat.find)
                                                .map_err(|err| err.to_string())
                                            {
                                                Ok(re) => {
                                                    s = re
                                                        .replace_all(&s, pat.replace.as_str())
                                                        .to_string();
                                                }
                                                Err(err) => response
                                                    .send(ClientResponse::Error(err))
                                                    .unwrap(),
                                            }
                                        } else {
                                            s = s.replace(&pat.find, &pat.replace);
                                        }
                                    }
                                }
                                response.send(ClientResponse::String(s)).unwrap();
                            }
                            Err(err) => response
                                .send(ClientResponse::Error(err.to_string()))
                                .unwrap(),
                        }
                    }
                    ClientCommand::List { listtype, param } => {
                        let res = match listtype.as_str() {
                            "allimages" => api::list::allimages(&client).await,
                            "allpages" => api::list::allpages(&client, param.as_deref()).await,
                            "alllinks" => api::list::alllinks(&client).await,
                            "allcategories" => api::list::allcategories(&client).await,
                            "backlinks" => api::list::backlinks(&client, param.as_deref()).await,
                            "categorymembers" => {
                                api::list::categorymembers(&client, param.as_deref()).await
                            }
                            "embeddedin" => api::list::embeddedin(&client, param.as_deref()).await,
                            "imageusage" => api::list::imageusage(&client, param.as_deref()).await,
                            "search" => api::list::search(&client, param.as_deref()).await,
                            "protectedtitles" => api::list::protectedtitles(&client).await,
                            "querypage" => api::list::querypage(&client, param.as_deref()).await,
                            "allinfoboxes" => api::list::allinfoboxes(&client).await,
                            _ => api::list::allimages(&client).await,
                        };
                        match res {
                            Ok(list) => response.send(ClientResponse::Vec(list)).unwrap(),
                            Err(err) => response
                                .send(ClientResponse::Error(err.to_string()))
                                .unwrap(),
                        }
                    }
                    ClientCommand::Login {
                        loginname,
                        password,
                        wikiurl,
                        is_persistent,
                    } => {
                        client.url(&wikiurl);
                        client.credentials(&loginname, &password);
                        let callback_val = cmd::LoginResponse {
                            username: loginname.clone(),
                            url: wikiurl.clone(),
                        };
                        // This kinda blocks the ui, but works best for now
                        let client_res = client.login().await;

                        if client_res.is_ok() {
                            *SAVED_STATE.lock().await = SavedState {
                                wikiurl: wikiurl.clone(),
                                loginname: loginname.clone(),
                                password: password.clone(),
                                is_persistent,
                            }
                        }

                        let loginname = match is_persistent {
                            true => loginname,
                            false => String::new(),
                        };
                        let password = match is_persistent {
                            true => password,
                            false => String::new(),
                        };

                        let save_res = SavedState {
                            wikiurl,
                            loginname,
                            password,
                            is_persistent,
                        }
                        .save()
                        .await;

                        match save_res {
                            Ok(_) => response
                                .send(ClientResponse::LoginResponse(callback_val))
                                .unwrap(),
                            Err(err) => response
                                .send(ClientResponse::Error(err.to_string()))
                                .unwrap(),
                        }
                    }
                    ClientCommand::Logout => {
                        match client.logout().await {
                            Ok(_) => response.send(ClientResponse::Ok).unwrap(),
                            Err(err) => response
                                .send(ClientResponse::Error(err.to_string()))
                                .unwrap(),
                        };
                    }
                    ClientCommand::Move { from, to } => {
                        match api::rename::rename(
                            &client,
                            from,
                            Some(api::rename::Destination::Plain(to)),
                            None,
                            None,
                        )
                        .await
                        {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                    ClientCommand::Purge => {}
                };
            } */
        })
        .invoke_handler(tauri::generate_handler![
            cmd::cache_get,
            cmd::cache_set,
            cmd::clear_files,
            //cmd::delete,
            //cmd::download,
            //cmd::edit,
            /*cmd::get_page,
            cmd::init,
            cmd::list,
            cmd::login,
            cmd::logout,
            cmd::r#move,
            cmd::purge,
            cmd::upload_dialog,
            cmd::upload */
        ])
        .build()
        .unwrap()
        .run();
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SavedState {
    wikiurl: String,
    loginname: String,
    password: String,
    is_persistent: bool,
}

impl SavedState {
    async fn load() -> Result<SavedState, ()> {
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

    async fn save(self) -> Result<(), anyhow::Error> {
        storage::insert_multiple(&[
            ("b9c95dde", storage::encrypt(&self.loginname)?.as_slice()),
            ("d7f0942b", storage::encrypt(&self.password)?.as_slice()),
            ("wikiurl", self.wikiurl.as_bytes()),
            ("is_persistent", self.is_persistent.to_string().as_bytes()),
        ])
        .await
    }
}
