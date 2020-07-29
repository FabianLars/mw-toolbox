use anyhow::{anyhow, Result};
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, CHACHA20_POLY1305};
use ring::rand::{SecureRandom, SystemRandom};
use serde_json::{json, Value};
use tokio::prelude::*;

fn path() -> std::path::PathBuf {
    let mut path =
        if let Some(project_dirs) = directories::ProjectDirs::from("de", "FabianLars", "wtools") {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap()
        };

    path.push("wtools.json");

    path
}

pub(crate) async fn insert(key: &str, val: &str) -> Result<()> {
    let path = path();

    let mut json: Value;

    if let Some(dir) = path.parent() {
        tokio::fs::create_dir_all(dir).await?;
    }

    {
        {
            json = match load().await {
                Ok(c) if !c.is_empty() => serde_json::from_str(&c).unwrap_or(json!({ "wtools": "persistent data" })),
                _ => json!({ "wtools": "persistent data" }),
            };

            json.as_object_mut()
                .unwrap()
                .insert(key.to_string(), json!(val));
        }

        {
            let mut file = tokio::fs::File::create(path).await?;

            file.write_all(serde_json::to_string_pretty(&json)?.as_bytes())
                .await?;
        }
    }

    Ok(())
}

async fn load() -> Result<String> {
    let mut contents = String::new();

    if let Some(dir) = path().parent() {
        tokio::fs::create_dir_all(dir).await?;
    }

    let mut file = tokio::fs::File::open(path()).await?;

    file.read_to_string(&mut contents).await?;

    Ok(contents)
}

pub(crate) async fn get(key: &str) -> Result<String> {
    let contents = load().await.unwrap_or(String::new());

    if contents.is_empty() {
        return Err(anyhow!("empty appdata"))
    }

    let json: Value = serde_json::from_str(&contents)?;

    if let Some(v) = json.get(key) {
        Ok(v.as_str().unwrap().to_string())
    } else {
        Err(anyhow!("Error getting value by key from file"))
    }
}

pub async fn insert_secure(key: &str, val: &str, to_ad: &str) -> Result<()> {
    let lskey = LessSafeKey::new(
        UnboundKey::new(&CHACHA20_POLY1305, env!("WTOOLS_AEAD_KEY").as_bytes())
            .map_err(|_| anyhow!("lskey error"))?,
    );

    let mut data = vec![0; val.len() + env!("WTOOLS_SECRET").len() + 28];
    let (nonce_storage, in_out) = data.split_at_mut(12);
    let (in_out, tag_storage) = in_out.split_at_mut(env!("WTOOLS_SECRET").len() + val.len());
    in_out.copy_from_slice(&[env!("WTOOLS_SECRET").as_bytes(), val.as_bytes()].concat());

    SystemRandom::new()
        .fill(nonce_storage)
        .expect("couldn't random fill nonce");
    let nonce = Nonce::try_assume_unique_for_key(nonce_storage).expect("try_assume_unique");

    let tag = lskey
        .seal_in_place_separate_tag(nonce, Aad::from(to_ad.as_bytes()), in_out)
        .map_err(|_| anyhow!("seal in place"))?;
    tag_storage.copy_from_slice(tag.as_ref());

    insert(key, &base64::encode(&data)).await
}

pub async fn get_secure(key: &str, to_ad: &str) -> Result<String> {
    let mut data = base64::decode(get(key).await?)?;
    if data.len() <= 12 {
        return Err(anyhow!("data len <= 12"));
    }

    let lskey = LessSafeKey::new(
        UnboundKey::new(&CHACHA20_POLY1305, env!("WTOOLS_AEAD_KEY").as_bytes())
            .map_err(|_| anyhow!("lskey error"))?,
    );

    let (nonce, mut sealed) = data.split_at_mut(12);
    //let (in_out, tag_storage) = in_out.split_at_mut(env!("WTOOLS_SECRET").len() + val.len());
    let nonce = Nonce::try_assume_unique_for_key(nonce).expect("invalid length of nonce");
    let unsealed = lskey
        .open_in_place(nonce, Aad::from(to_ad.as_bytes()), &mut sealed)
        .map_err(|_| anyhow!("open in place"))?;
    let (_, unsealed) = unsealed.split_at(env!("WTOOLS_SECRET").len());

    ::std::str::from_utf8(unsealed)
        .map(|s| s.to_string())
        .map_err(|_| anyhow!("utf8 error"))
}
