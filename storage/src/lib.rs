use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};
use rand::prelude::*;
use tokio::{fs::File, prelude::*};

fn path() -> std::path::PathBuf {
    let mut path =
        if let Some(project_dirs) = directories::ProjectDirs::from("de", "FabianLars", "wtools") {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap()
        };

    path.push("wtools");

    path
}

async fn load() -> Result<BTreeMap<String, Vec<u8>>> {
    if let Some(dir) = path().parent() {
        tokio::fs::create_dir_all(dir).await?;
    }

    let contents = tokio::fs::read(path()).await?;

    bincode::deserialize(&contents).map_err(|e| anyhow!("Error while deserializing: {:?}", e))
}

pub async fn get(key: &str) -> Result<String> {
    String::from_utf8(get_u8(key).await?)
        .map_err(|e| anyhow!("Error converting u8 to String: {:?}", e))
}

async fn get_u8(key: &str) -> Result<Vec<u8>> {
    let data = load().await.unwrap_or_default();

    if data.is_empty() {
        return Err(anyhow!("empty appdata"));
    }

    if let Some(v) = data.get(key) {
        Ok(v.to_vec())
    } else {
        Err(anyhow!("Error getting value by key from file"))
    }
}

pub async fn get_secure(key: &str) -> Result<String> {
    let data = get_u8(key).await?;
    if data.len() <= 24 {
        return Err(anyhow!("Missing data. Extracted value is too short"));
    }
    let cryptkey = Key::from_slice(env!("WTOOLS_AEAD_KEY").as_bytes());
    let aead = XChaCha20Poly1305::new(cryptkey);

    let (nonced, ciphertext) = data.split_at(24);
    let nonce = XNonce::from_slice(nonced);

    let plaintext = aead
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow!("Error decrypting value: {:?}", e))?;

    let (_, plaintext) = plaintext.split_at(env!("WTOOLS_SECRET").len());

    String::from_utf8(plaintext.to_vec())
        .map_err(|e| anyhow!("Error converting decrypted u8-value to String: {:?}", e))
}

pub async fn insert(key: &str, val: &str) -> Result<()> {
    insert_u8(key, val.as_bytes().to_vec()).await
}

async fn insert_u8(key: &str, val: Vec<u8>) -> Result<()> {
    let path = path();

    let mut map: BTreeMap<String, Vec<u8>>;

    if let Some(dir) = path.parent() {
        tokio::fs::create_dir_all(dir).await?;
    }

    let loaded = load().await;
    map = match loaded {
        Ok(c) => c,
        _ => BTreeMap::new(),
    };
    map.insert(key.to_string(), val);

    let mut file = File::create(path).await?;
    file.write_all(
        &bincode::serialize(&map).map_err(|e| anyhow!("Error while serializing: {:?}", e))?,
    )
    .await?;

    Ok(())
}

pub async fn insert_secure(key: &str, val: &str) -> Result<()> {
    let cryptkey = Key::from_slice(env!("WTOOLS_AEAD_KEY").as_bytes());
    let aead = XChaCha20Poly1305::new(cryptkey);

    let mut rng_nonce = [0u8; 24];
    rand_chacha::ChaCha20Rng::from_entropy().fill(&mut rng_nonce[..]);

    let nonce = XNonce::from_slice(&rng_nonce);
    let mut ciphertext = aead
        .encrypt(nonce, [env!("WTOOLS_SECRET"), val].concat().as_bytes())
        .map_err(|e| anyhow!("Error encrypting value: {:?}", e))?;

    let mut data = rng_nonce.to_vec();
    data.append(&mut ciphertext);

    insert_u8(key, data).await
}
