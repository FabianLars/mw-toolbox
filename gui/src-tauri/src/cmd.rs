use std::{collections::HashMap, sync::atomic::Ordering};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::command;

use mw_tools::{api, Error};

use crate::{CANCEL_ACTION, CLIENT};

type Cache = parking_lot::Mutex<HashMap<String, Value>>;
type Result<T, E = Error> = core::result::Result<T, E>;

/// Struct to store users. Each profile is stored in a new instance.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Profile {
    /// Name of the profile
    profile: String,
    /// Username of the account
    username: String,
    /// Password of the account
    password: String,
    /// Wiki-URL of the account
    url: String,
    /// Whether the login credentials should be saved locally.
    #[serde(rename = "savePassword")]
    save_password: bool,
}

/// Return value of [get_page].
#[derive(Debug, Serialize)]
pub struct GetPage {
    /// Page content
    content: String,
    /// Whether the content got edited via regex or not.
    edited: bool,
}

/// Struct for Find&Replace operations. Each operation is stored in a new instance.
#[derive(Clone, Debug, Deserialize)]
pub struct FindReplace {
    /// Value to replace
    #[serde(default)]
    pub find: String,
    /// Value to insert
    #[serde(default)]
    pub replace: String,
    /// Whether find and replace fields should be interpreted as a regular expression.
    #[serde(rename = "isRegex", default)]
    pub is_regex: bool,
}

/// Get json-compatible ([`serde_json::Value`]) objects from runtime cache.
#[command]
pub fn cache_get(key: &str, cache: tauri::State<Cache>) -> Option<Value> {
    cache.lock().get(key).map(|v| v.to_owned())
}

/// Store json-compatible ([`serde_json::Value`]) objects in runtime cache.
#[command]
pub fn cache_set(key: String, value: Value, cache: tauri::State<Cache>) -> bool {
    cache.lock().insert(key, value).is_some()
}

/// Command to delete pages.
#[command]
pub async fn delete(pages: Vec<&str>, reason: Option<&str>) -> Result<()> {
    api::delete::delete(&*CLIENT.lock().await, &pages, reason).await
}

/// Command to download files.
#[command]
pub async fn download(files: Vec<&str>) -> Result<()> {
    api::download::download(&*CLIENT.lock().await, &files).await
}

/// Command to save edited pages.
#[command]
pub async fn edit(title: &str, content: &str, summary: Option<&str>) -> Result<String> {
    api::edit::edit(&*CLIENT.lock().await, title, content, summary).await
}

/// Command that runs the editor in auto-save mode.
#[command]
pub async fn auto_edit(
    titles: Vec<&str>,
    patterns: Vec<FindReplace>,
    summary: Option<&str>,
    window: tauri::Window,
) -> Result<()> {
    CANCEL_ACTION.store(false, Ordering::Relaxed);

    for t in titles {
        if CANCEL_ACTION.load(Ordering::Relaxed) {
            println!("cancel_edit true");
            break;
        }
        let gp = get_page(t, patterns.clone()).await?;
        if gp.edited {
            let _ = api::edit::edit(&*CLIENT.lock().await, t, &gp.content, summary).await?;
            window
                .emit("page-edited", t)
                .map_err(|_| Error::Other("Couldn't emit event to window".to_string()))?;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        } else {
            window
                .emit("page-skipped", t)
                .map_err(|_| Error::Other("Couldn't emit event to window".to_string()))?;
        }
    }
    Ok(())
}

/// Command to get page content. Runs Find&Replace operations before returning.
#[command]
pub async fn get_page(page: &str, patterns: Vec<FindReplace>) -> Result<GetPage> {
    let mut s = api::parse::get_page_content(&*CLIENT.lock().await, page).await?;
    let mut edited = false;
    for pat in patterns {
        if !pat.find.is_empty() {
            if pat.is_regex {
                let re =
                    regex::Regex::new(&pat.find).map_err(|err| Error::Other(err.to_string()))?;
                let new = re
                    .replace_all(&s, unescape::unescape(&pat.replace).unwrap_or(pat.replace))
                    .to_string();
                if new != s {
                    edited = true;
                }
                s = new;
            } else {
                let new = s.replace(&pat.find, &pat.replace);
                if new != s {
                    edited = true;
                }
                s = new;
            }
        }
    }
    Ok(GetPage { content: s, edited })
}

/// Command to get locally saved users and the index of the last active profile.
#[command]
pub async fn init() -> (Vec<Profile>, usize) {
    storage::load_secure::<(Vec<Profile>, usize)>("oB9uBQDs")
        .await
        .unwrap_or_default()
}

/// Command to get wiki-generated page lists.
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
        _ => Err(Error::InvalidInput(format!(
            "Invalid listtype provided: \"{}\"",
            listtype
        ))),
    }
}

/// Command to login.
#[command]
pub async fn login(profiles: Vec<Profile>, current: usize) -> Result<usize> {
    let current_profile = &profiles[current];

    let mut client = CLIENT.lock().await;
    client.set_url(&current_profile.url);

    client
        .login(&current_profile.username, &current_profile.password)
        .await?;

    update_profile_store(profiles, current)
        .await
        .and(Ok(current))
}

/// Command to logout.
#[command]
pub async fn logout() -> Result<()> {
    CLIENT.lock().await.logout().await
}

/// Command to move pages.
#[command]
pub async fn rename(from: Vec<String>, to: Vec<String>) -> Result<()> {
    api::rename::rename(
        &*CLIENT.lock().await,
        from,
        Some(api::rename::Destination::Plain(to)),
        None,
        None,
    )
    .await
}

/// Command to purge or nulledit pages.
#[command]
pub async fn purge(is_nulledit: bool, pages: Vec<&str>) -> Result<()> {
    let client = CLIENT.lock().await;
    if is_nulledit {
        api::edit::nulledit(&*client, &pages).await
    } else {
        api::purge::purge(&*client, &pages, true).await
    }
}

/// Command to update locally saved users.
#[command]
pub async fn update_profile_store(mut profiles: Vec<Profile>, current: usize) -> Result<()> {
    for p in &mut profiles {
        if !p.save_password {
            p.password = "".to_string();
        }
    }

    storage::save_secure("oB9uBQDs", (profiles, current))
        .await
        .map_err(|err| Error::Other(err.to_string()))
}

/// Command to upload files.
#[command]
pub async fn upload(text: &str, files: Vec<&str>, window: tauri::Window) -> Result<()> {
    CANCEL_ACTION.store(false, Ordering::Relaxed);
    let mut file_iter = files.iter();
    while !CANCEL_ACTION.load(Ordering::Relaxed) {
        if let Some(file) = file_iter.next() {
            // Check if path resolves to a file. Skip upload otherwise.
            if std::fs::metadata(file)?.is_file() {
                api::upload::upload(&*CLIENT.lock().await, file, Some(text)).await?;
            }
            // Emit uploaded event no matter if it's a file or a folder, to remove it from the frontend.
            window
                .emit("file-uploaded", file)
                .map_err(|err| Error::Other(err.to_string()))?;
        } else {
            break;
        }
    }
    Ok(())
}
