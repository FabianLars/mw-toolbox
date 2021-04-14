#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};
use rand::prelude::*;
use tokio::{fs::File, io::AsyncWriteExt};

fn path() -> std::path::PathBuf {
    let mut path = directories_next::ProjectDirs::from("", "", "mw-toolbox")
        .map(|pd| pd.data_dir().to_path_buf())
        .expect("Can't create or find app data folder!");

    path.push("storage");

    path
}

async fn load() -> Result<BTreeMap<String, Vec<u8>>> {
    if let Some(dir) = path().parent() {
        tokio::fs::create_dir_all(dir).await?;
    }

    let contents = tokio::fs::read(path()).await?;

    bincode::deserialize(&contents).map_err(|e| anyhow!("Error while deserializing: {:?}", e))
}

pub fn encrypt<T: AsRef<[u8]>>(val: T) -> Result<Vec<u8>> {
    let cryptkey = Key::from_slice(env!("WTOOLS_AEAD_KEY").as_bytes());
    let aead = XChaCha20Poly1305::new(cryptkey);

    let mut rng_nonce = [0u8; 24];
    rand_chacha::ChaCha20Rng::from_entropy().fill(&mut rng_nonce[..]);

    let nonce = XNonce::from_slice(&rng_nonce);
    let mut ciphertext = aead
        .encrypt(
            nonce,
            [env!("WTOOLS_SECRET").as_bytes(), val.as_ref()]
                .concat()
                .as_slice(),
        )
        .map_err(|e| anyhow!("Error encrypting value: {:?}", e))?;

    let mut data = rng_nonce.to_vec();
    data.append(&mut ciphertext);

    Ok(data)
}

pub fn decrypt(data: Vec<u8>) -> Result<String> {
    if data.len() <= 24 {
        return Err(anyhow!("Value given is too short to be encrypted data"));
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

pub async fn get(key: &str) -> Result<String> {
    String::from_utf8(get_u8(key).await?)
        .map_err(|e| anyhow!("Error converting u8 to String: {:?}", e))
}

pub async fn get_secure(key: &str) -> Result<String> {
    let data = get_u8(key).await?;

    decrypt(data)
}

pub async fn insert<T: AsRef<[u8]>>(key: &str, val: T) -> Result<()> {
    insert_multiple(&[(key, val)]).await
}

pub async fn insert_multiple<T: AsRef<[u8]>>(data: &[(&str, T)]) -> Result<()> {
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

    for (k, v) in data {
        map.insert(k.to_string(), v.as_ref().to_vec());
    }

    let mut file = File::create(path).await?;
    file.write_all(
        &bincode::serialize(&map).map_err(|e| anyhow!("Error while serializing: {:?}", e))?,
    )
    .await?;

    Ok(())
}

pub async fn insert_secure(key: &str, val: &str) -> Result<()> {
    insert(key, encrypt(val)?).await
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn storage() {
        use rand::{distributions::Alphanumeric, Rng};

        super::insert("libtest_unsec", "abcdefghi123456789")
            .await
            .unwrap();

        assert_eq!(
            super::get("libtest_unsec").await.unwrap(),
            "abcdefghi123456789".to_string()
        );

        let test_string = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(18)
            .collect::<Vec<u8>>();

        let test_string = String::from_utf8(test_string).unwrap();

        super::insert_secure("unittest_secure", &test_string)
            .await
            .unwrap();

        assert_eq!(
            super::get_secure("unittest_secure").await.unwrap(),
            test_string
        );
    }
}
