use std::{collections::HashMap, sync::atomic::Ordering};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::command;

use mw_tools::{api, error::ToolsError};

use crate::{CANCEL_EDIT, CANCEL_UPLOAD, CLIENT};

type Cache = parking_lot::Mutex<HashMap<String, Value>>;
type Result<T, E = ToolsError> = core::result::Result<T, E>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Profile {
    profile: String,
    username: String,
    password: String,
    url: String,
    #[serde(rename = "savePassword")]
    save_password: bool,
}

#[derive(Debug, Serialize)]
pub struct GetPage {
    content: String,
    edited: bool,
}

#[derive(Clone, Debug, Deserialize)]
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
pub async fn auto_edit(
    titles: Vec<&str>,
    patterns: Vec<FindReplace>,
    summary: Option<&str>,
    window: tauri::Window,
) -> Result<()> {
    CANCEL_EDIT.store(false, Ordering::Relaxed);

    for t in titles {
        if CANCEL_EDIT.load(Ordering::Relaxed) {
            println!("cancel_edit true");
            break;
        }
        let gp = get_page(t, patterns.clone()).await?;
        if gp.edited {
            let _ = api::edit::edit(&*CLIENT.lock().await, t, &gp.content, summary).await?;
            window
                .emit("page-edited", t)
                .map_err(|_| ToolsError::Other("Couldn't emit event to window".to_string()))?;
            //tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        } else {
            window
                .emit("page-skipped", t)
                .map_err(|_| ToolsError::Other("Couldn't emit event to window".to_string()))?;
        }
    }
    Ok(())
}

#[command]
pub async fn get_page(page: &str, patterns: Vec<FindReplace>) -> Result<GetPage> {
    let mut s = api::parse::get_page_content(&*CLIENT.lock().await, page).await?;
    let mut edited = false;
    for pat in patterns {
        if !pat.find.is_empty() {
            if pat.is_regex {
                let re = regex::Regex::new(&pat.find)
                    .map_err(|err| ToolsError::Other(err.to_string()))?;
                let new = re
                    .replace_all(&s, unescape::unescape(&pat.replace).unwrap_or(pat.replace))
                    .to_string();
                if new != s {
                    edited = true
                }
                s = new;
            } else {
                let new = s.replace(&pat.find, &pat.replace);
                if new != s {
                    edited = true
                }
                s = new;
            }
        }
    }
    Ok(GetPage { content: s, edited })
}

#[command]
pub async fn init() -> (Vec<Profile>, usize) {
    storage::load_secure::<(Vec<Profile>, usize)>("a7caf1a8")
        .await
        .unwrap_or_default()
}

#[command]
pub async fn list(listtype: &str, param: Option<&str>) -> Result<Vec<String>> {
    let client = CLIENT.lock().await;
    let param = param.unwrap_or_default();
    match listtype {
        "allimages" => api::list::allimages(&*client).await,
        "allpages" => api::list::allpages(&*client, Some(param)).await,
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
pub async fn login(profiles: Vec<Profile>, current: usize) -> Result<usize> {
    let current_profile = &profiles[current];

    let mut client = CLIENT.lock().await;
    client.url(&current_profile.url);
    client.credentials(&current_profile.username, &current_profile.password);

    client.login().await?;

    update_profile_store(profiles, current)
        .await
        .and(Ok(current))
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
pub async fn update_profile_store(mut profiles: Vec<Profile>, current: usize) -> Result<()> {
    for p in profiles.iter_mut() {
        if !p.save_password {
            p.password = "".to_string();
        }
    }

    storage::save_secure("a7caf1a8", (profiles, current))
        .await
        .map_err(|err| ToolsError::Other(err.to_string()))
}

#[command]
pub async fn upload<P: tauri::Params<Event = String>>(
    text: &str,
    files: Vec<&str>,
    window: tauri::Window<P>,
) -> Result<()> {
    CANCEL_UPLOAD.store(false, Ordering::Relaxed);
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
    Ok(())
}
