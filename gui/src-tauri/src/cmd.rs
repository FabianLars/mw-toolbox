use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::command;

use mw_tools::api;

use crate::{SavedState, CACHE, CLIENT, FILES_HELPER, SAVED_STATE};

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
pub async fn cache_get(key: String) -> Option<Value> {
    if let Some(v) = CACHE.lock().await.get(&key) {
        let v = v.to_owned();
        Some(v)
    } else {
        None
    }
}

#[command]
pub async fn cache_set(key: String, value: Value) -> Result<bool, ()> {
    let updated = CACHE.lock().await.insert(key, value).is_some();
    Ok(updated)
}

#[command]
pub async fn clear_files() {
    FILES_HELPER.lock().await.clear();
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
pub async fn init() -> Result<SavedState, String> {
    let locked_state = SAVED_STATE.lock().await;
    Ok(locked_state.clone())
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
    .save_async()
    .await;

    match save_res {
        Ok(_) => Ok(callback_val),
        Err(err) => Err(err.to_string()),
    }
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
pub async fn upload_dialog() -> Result<Vec<String>, String> {
    let result = rfd::FileDialog::new().pick_files();

    if let Some(selected_files) = result {
        let mut helper = FILES_HELPER.lock().await;
        *helper = selected_files;
        let arr: Vec<String> = helper.iter().map(|x| x.display().to_string()).collect();
        Ok(arr)
    } else {
        Err("No files selected".to_string())
    }
}

#[command]
pub async fn upload(text: String) -> Result<(), String> {
    api::upload::upload_multiple(
        &*CLIENT.lock().await,
        &*FILES_HELPER.lock().await,
        Some(text),
    )
    .await
    .map_err(|err| err.to_string())
}

#[derive(Debug)]
pub struct TestState {
    pub val: usize,
    pub st: String,
}

#[command]
pub async fn test(arg: String, state: tauri::State<'_, std::sync::Mutex<TestState>>) {
    let mut lock = state.lock().unwrap();
    lock.val += 1;
    lock.st = arg;

    println!("{:?} {:?}", state.inner(), &lock);
}
