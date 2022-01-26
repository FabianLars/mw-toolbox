#![forbid(unsafe_code)]

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};
use once_cell::sync::Lazy;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};

static PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = dirs_next::config_dir().expect("Unable to determine config dir.");
    path.push("de.FabianLars.mw-toolbox/Storage/");
    std::fs::create_dir_all(&path).expect("Unable to create storage directory.");
    path
});

pub fn encrypt(data: &[u8]) -> Result<Vec<u8>> {
    let cryptkey = Key::from_slice(env!("WTOOLS_AEAD_KEY").as_bytes());
    let aead = XChaCha20Poly1305::new(cryptkey);

    let mut rng_nonce = [0u8; 24];
    ChaCha20Rng::from_entropy().fill_bytes(&mut rng_nonce[..]);

    let nonce = XNonce::from_slice(&rng_nonce);
    let mut ciphertext = aead
        .encrypt(nonce, data)
        .map_err(|e| anyhow!("Error encrypting value: {:?}", e))?;

    ciphertext.splice(0..0, rng_nonce.iter().copied());

    Ok(ciphertext)
}

pub fn decrypt(data: &[u8]) -> Result<Vec<u8>> {
    if data.len() <= 24 {
        return Err(anyhow!("Value given is too short to be encrypted data."));
    }
    let cryptkey = Key::from_slice(env!("WTOOLS_AEAD_KEY").as_bytes());
    let aead = XChaCha20Poly1305::new(cryptkey);

    let (nonced, ciphertext) = data.split_at(24);
    let nonce = XNonce::from_slice(nonced);

    let plaintext = aead
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow!("Error decrypting value: {:?}", e))?;

    Ok(plaintext)
}

pub async fn save<T: Serialize>(name: &str, object: T) -> Result<()> {
    let mut path = PATH.clone();
    path.push(format!("{}.mwt", name));

    let mut file = File::create(path).await?;
    file.write_all(
        &bincode::serialize(&object).map_err(|e| anyhow!("Error while serializing: {:?}", e))?,
    )
    .await?;

    Ok(())
}

pub async fn save_secure<T: Serialize>(name: &str, object: T) -> Result<()> {
    let mut path = PATH.clone();
    path.push(format!("{}.mwts", name));

    let mut file = File::create(path).await?;
    file.write_all(&encrypt(
        &bincode::serialize(&object).map_err(|e| anyhow!("Error while serializing: {:?}", e))?,
    )?)
    .await?;

    Ok(())
}

pub async fn load<T: Serialize + DeserializeOwned>(name: &str) -> Result<T> {
    let mut path = PATH.clone();
    path.push(format!("{}.mwt", name));

    let file_contents = tokio::fs::read(path).await?;

    bincode::deserialize(&file_contents).map_err(|e| anyhow!("Error while deserializing: {:?}", e))
}

pub async fn load_secure<T: Serialize + DeserializeOwned>(name: &str) -> Result<T> {
    let mut path = PATH.clone();
    path.push(format!("{}.mwts", name));

    let file_contents = tokio::fs::read(path).await?;

    bincode::deserialize(&decrypt(&file_contents)?)
        .map_err(|e| anyhow!("Error while deserializing: {:?}", e))
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestObj {
        bool: bool,
        usize: usize,
        string: String,
        map: HashMap<String, String>,
    }
    #[tokio::test]
    async fn storage() {
        use rand::{distributions::Alphanumeric, thread_rng, Rng};

        let bool = thread_rng().gen_bool(0.5);

        let usize: usize = thread_rng().gen();

        let mut map = HashMap::new();
        map.insert(
            "libtest".to_string(),
            String::from_utf8(
                thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(32)
                    .collect::<Vec<u8>>(),
            )
            .unwrap(),
        );

        let string = String::from_utf8(
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .collect::<Vec<u8>>(),
        )
        .unwrap();

        let test_obj = TestObj {
            bool,
            usize,
            string,
            map,
        };

        super::save("libtest_unsec", &test_obj).await.unwrap();

        super::save_secure("libtest_sec", &test_obj).await.unwrap();

        let loaded_unsec: TestObj = super::load("libtest_unsec").await.unwrap();

        let loaded_sec: TestObj = super::load_secure("libtest_sec").await.unwrap();

        assert_eq!(test_obj, loaded_unsec);
        assert_eq!(test_obj, loaded_sec);
        assert_eq!(loaded_sec, loaded_unsec);
    }
}
