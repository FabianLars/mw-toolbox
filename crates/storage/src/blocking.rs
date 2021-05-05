use std::{collections::BTreeMap, fs::File, io::Write};

use anyhow::{anyhow, Result};

use crate::path;
pub use crate::{decrypt, encrypt};

fn load() -> Result<BTreeMap<String, Vec<u8>>> {
    if let Some(dir) = path().parent() {
        std::fs::create_dir_all(dir)?;
    }

    let contents = std::fs::read(path())?;

    bincode::deserialize(&contents).map_err(|e| anyhow!("Error while deserializing: {:?}", e))
}

fn get_u8(key: &str) -> Result<Vec<u8>> {
    let data = load().unwrap_or_default();

    if data.is_empty() {
        return Err(anyhow!("empty appdata"));
    }

    if let Some(v) = data.get(key) {
        Ok(v.to_vec())
    } else {
        Err(anyhow!("Error getting value by key from file"))
    }
}

pub fn get(key: &str) -> Result<String> {
    String::from_utf8(get_u8(key)?).map_err(|e| anyhow!("Error converting u8 to String: {:?}", e))
}

pub fn get_secure(key: &str) -> Result<String> {
    let data = get_u8(key)?;

    decrypt(data)
}

pub fn insert<T: AsRef<[u8]>>(key: &str, val: T) -> Result<()> {
    insert_multiple(&[(key, val)])
}

pub fn insert_multiple<T: AsRef<[u8]>>(data: &[(&str, T)]) -> Result<()> {
    let path = path();

    let mut map: BTreeMap<String, Vec<u8>>;

    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }

    let loaded = load();
    map = loaded.unwrap_or_default();

    for (k, v) in data {
        map.insert(k.to_string(), v.as_ref().to_vec());
    }

    let mut file = File::create(path)?;
    file.write_all(
        &bincode::serialize(&map).map_err(|e| anyhow!("Error while serializing: {:?}", e))?,
    )?;

    Ok(())
}

pub fn insert_secure(key: &str, val: &str) -> Result<()> {
    insert(key, encrypt(val)?)
}
