use std::{collections::HashMap, sync::atomic::Ordering};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::command;

use mw_tools::{api, error::ToolsError};

use crate::{SavedState, CANCEL_UPLOAD, CLIENT};

type Cache = parking_lot::Mutex<HashMap<String, Value>>;
type Result<T, E = ToolsError> = core::result::Result<T, E>;

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

#[command]
pub fn cache_get(key: String, cache: tauri::State<Cache>) -> Option<Value> {
    if let Some(v) = cache.lock().get(&key) {
        let v = v.to_owned();
        Some(v)
    } else {
        None
    }
}

#[command]
pub fn cache_set(key: String, value: Value, cache: tauri::State<Cache>) -> bool {
    cache.lock().insert(key, value).is_some()
}

#[command]
pub async fn delete(pages: Vec<&str>, reason: Option<&str>) -> Result<()> {
    let client = CLIENT.lock().await;
    api::delete::delete(&*client, &pages, reason).await
}

#[command]
pub async fn download(files: Vec<&str>) -> Result<()> {
    api::download::download(&*CLIENT.lock().await, &files).await
}

#[command]
pub async fn edit(title: &str, content: &str, summary: Option<&str>) -> Result<String> {
    api::edit::edit(&*CLIENT.lock().await, title, content, summary).await
}

#[command]
pub async fn get_page(page: &str, patterns: Vec<FindReplace>) -> Result<String> {
    let mut s = api::parse::get_page_content(&*CLIENT.lock().await, page).await?;
    for pat in patterns {
        if !pat.find.is_empty() {
            if pat.is_regex {
                let re = regex::Regex::new(&pat.find)
                    .map_err(|err| ToolsError::Other(err.to_string()))?;
                s = re
                    .replace_all(&s, unescape::unescape(&pat.replace).unwrap_or(pat.replace))
                    .to_string();
            } else {
                s = s.replace(&pat.find, &pat.replace);
            }
        }
    }
    Ok(s)
}

#[command]
pub async fn init() -> SavedState {
    SavedState::load().await
}

#[command]
pub async fn list(listtype: &str, param: Option<&str>) -> Result<Vec<String>> {
    let client = CLIENT.lock().await;
    match listtype {
        "allimages" => api::list::allimages(&*client).await,
        "allpages" => api::list::allpages(&*client, param).await,
        "alllinks" => api::list::alllinks(&*client).await,
        "allcategories" => api::list::allcategories(&*client).await,
        "backlinks" => api::list::backlinks(&*client, param).await,
        "categorymembers" => api::list::categorymembers(&*client, param).await,
        "embeddedin" => api::list::embeddedin(&*client, param).await,
        "imageusage" => api::list::imageusage(&*client, param).await,
        "search" => api::list::search(&*client, param).await,
        "protectedtitles" => api::list::protectedtitles(&*client).await,
        "querypage" => api::list::querypage(&*client, param).await,
        "allinfoboxes" => api::list::allinfoboxes(&*client).await,
        _ => Err(ToolsError::InvalidInput(format!(
            "Invalid listtype provided: \"{}\"",
            listtype
        ))),
    }
}

#[command]
pub async fn login(
    loginname: String,
    password: String,
    wikiurl: String,
    is_persistent: bool,
) -> Result<LoginResponse> {
    let mut client = CLIENT.lock().await;
    client.url(&wikiurl);
    client.credentials(&loginname, &password);
    let callback_val = LoginResponse {
        username: loginname.clone(),
        url: wikiurl.clone(),
    };

    client.login().await?;

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

    save_res.and(Ok(callback_val))
}

#[command]
pub async fn logout() -> Result<()> {
    CLIENT.lock().await.logout().await
}

#[command]
pub async fn r#move(from: Vec<String>, to: Vec<String>) -> Result<()> {
    api::rename::rename(
        &*CLIENT.lock().await,
        from,
        Some(api::rename::Destination::Plain(to)),
        None,
        None,
    )
    .await
}

#[command]
pub async fn purge(is_nulledit: bool, pages: Vec<&str>) -> Result<()> {
    let client = CLIENT.lock().await;
    if is_nulledit {
        api::edit::nulledit(&*client, &pages).await
    } else {
        api::purge::purge(&*client, &pages, true).await
    }
}

#[command]
pub async fn upload<P: tauri::Params<Event = String>>(
    text: &str,
    files: Vec<&str>,
    window: tauri::Window<P>,
) -> Result<()> {
    let mut file_iter = files.iter();
    while !CANCEL_UPLOAD.load(Ordering::Relaxed) {
        if let Some(file) = file_iter.next() {
            api::upload::upload(&*CLIENT.lock().await, file, Some(text)).await?;
            window
                .emit("file-uploaded", file)
                .map_err(|err| ToolsError::Other(err.to_string()))?;
        } else {
            break;
        }
    }
    CANCEL_UPLOAD.store(false, Ordering::Relaxed);
    Ok(())
}
