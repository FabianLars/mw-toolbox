use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::command;

use mw_tools::api;

use crate::{SavedState, CLIENT};

type Cache = parking_lot::Mutex<HashMap<String, Value>>;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    username: String,
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct FindReplace {
    #[serde(default)]
    pub find: String,
    #[serde(default)]
    pub replace: String,
    #[serde(rename = "isRegex", default)]
    pub is_regex: bool,
}

// TODO: Use actual errors instead of error strings

#[command]
pub fn cache_get(key: String, cache: tauri::State<'_, Cache>) -> Option<Value> {
    if let Some(v) = cache.lock().get(&key) {
        let v = v.to_owned();
        Some(v)
    } else {
        None
    }
}

#[command]
pub fn cache_set(key: String, value: Value, cache: tauri::State<'_, Cache>) -> Result<bool, ()> {
    let updated = cache.lock().insert(key, value).is_some();
    Ok(updated)
}

#[command]
pub async fn delete(pages: Vec<String>) -> Result<(), String> {
    let client = CLIENT.lock().await;
    api::delete::delete(&*client, &pages[..])
        .await
        .map_err(|err| err.to_string())
}

#[command]
pub async fn download(files: Vec<String>) -> Result<(), String> {
    api::download::download(&*CLIENT.lock().await, &files)
        .await
        .map_err(|err| err.to_string())
}

#[command]
pub async fn edit(
    title: String,
    content: String,
    summary: Option<String>,
) -> Result<String, String> {
    api::edit::edit(&*CLIENT.lock().await, title, content, summary)
        .await
        .map_err(|err| err.to_string())
}

#[command]
pub async fn get_page(page: String, patterns: Vec<FindReplace>) -> Result<String, String> {
    match api::parse::get_page_content(&*CLIENT.lock().await, page).await {
        Ok(s) => {
            let mut s = s;
            for pat in patterns {
                if !pat.find.is_empty() {
                    if pat.is_regex {
                        let re = regex::Regex::new(&pat.find).map_err(|err| err.to_string())?;
                        s = re
                            .replace_all(
                                &s,
                                unescape::unescape(&pat.replace).unwrap_or(pat.replace),
                            )
                            .to_string();
                    } else {
                        s = s.replace(&pat.find, &pat.replace);
                    }
                }
            }
            Ok(s)
        }
        Err(err) => Err(err.to_string()),
    }
}

#[command]
pub async fn init() -> SavedState {
    SavedState::load().await
}

#[command]
pub async fn list(listtype: String, param: Option<String>) -> Result<Vec<String>, String> {
    let client = CLIENT.lock().await;
    let res = match listtype.as_str() {
        "allimages" => api::list::allimages(&*client).await,
        "allpages" => api::list::allpages(&*client, param.as_deref()).await,
        "alllinks" => api::list::alllinks(&*client).await,
        "allcategories" => api::list::allcategories(&*client).await,
        "backlinks" => api::list::backlinks(&*client, param.as_deref()).await,
        "categorymembers" => api::list::categorymembers(&*client, param.as_deref()).await,
        "embeddedin" => api::list::embeddedin(&*client, param.as_deref()).await,
        "imageusage" => api::list::imageusage(&*client, param.as_deref()).await,
        "search" => api::list::search(&*client, param.as_deref()).await,
        "protectedtitles" => api::list::protectedtitles(&*client).await,
        "querypage" => api::list::querypage(&*client, param.as_deref()).await,
        "allinfoboxes" => api::list::allinfoboxes(&*client).await,
        _ => api::list::allimages(&*client).await,
    };
    match res {
        Ok(list) => Ok(list),
        Err(err) => Err(err.to_string()),
    }
}

#[command]
pub async fn login(
    loginname: String,
    password: String,
    wikiurl: String,
    is_persistent: bool,
) -> Result<LoginResponse, String> {
    let mut client = CLIENT.lock().await;
    client.url(&wikiurl);
    client.credentials(&loginname, &password);
    let callback_val = LoginResponse {
        username: loginname.clone(),
        url: wikiurl.clone(),
    };

    let client_res = client.login().await;

    if client_res.is_err() {
        return Err(client_res.unwrap_err().to_string());
    }

    let (loginname, password) = if is_persistent {
        (loginname, password)
    } else {
        ("".to_string(), "".to_string())
    };

    let save_res = SavedState {
        wikiurl,
        loginname,
        password,
        is_persistent,
    }
    .save()
    .await;

    save_res
        .and(Ok(callback_val))
        .map_err(|err| err.to_string())
}

#[command]
pub async fn logout() -> Result<(), String> {
    CLIENT
        .lock()
        .await
        .logout()
        .await
        .map_err(|err| err.to_string())
}

#[command]
pub async fn r#move(from: Vec<String>, to: Vec<String>) -> Result<(), String> {
    api::rename::rename(
        &*CLIENT.lock().await,
        from,
        Some(api::rename::Destination::Plain(to)),
        None,
        None,
    )
    .await
    .map_err(|err| err.to_string())
}

#[command]
pub async fn purge(is_nulledit: bool, pages: Vec<String>) -> Result<(), String> {
    let client = CLIENT.lock().await;
    match match is_nulledit {
        true => api::edit::nulledit(&*client, &pages[..]).await,
        false => api::purge::purge(&*client, &pages[..], true).await,
    } {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[command]
pub async fn upload(text: String, files: Vec<String>) -> Result<(), String> {
    api::upload::upload_multiple(&*CLIENT.lock().await, &files, Some(text))
        .await
        .map_err(|err| err.to_string())
}
