use async_std::prelude::*;
use serde_json::Value;

use crate::util::PathType;
use crate::WikiClient;

pub async fn upload<C: AsRef<WikiClient>>(client: C, path: PathType) -> anyhow::Result<()> {
    let client = client.as_ref();
    let mut pages = String::new();
    let mut files: Vec<async_std::path::PathBuf> = Vec::new();

    match path {
        PathType::File(x) => {
            files.push(x);
        }
        PathType::Files(v) => files = v,
        PathType::Folder(x) => {
            let mut entries = async_std::fs::read_dir(x).await?;
            while let Some(entry) = entries.next().await {
                files.push(entry?.path());
            }
        }
    }

    for f in &files {
        pages.push_str(&format!(
            "Datei:{}|",
            f.file_name().unwrap().to_os_string().to_str().unwrap()
        ))
    }
    pages.pop();

    let json: Value = client
        .request_json(&[
            ("action", "query"),
            ("format", "json"),
            ("prop", "info"),
            ("intoken", "edit"),
            ("titles", &pages),
        ])
        .await?;

    let (_i, o) = json["query"]["pages"]
        .as_object()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let edit_token = String::from(o["edittoken"].as_str().unwrap());

    for f in files {
        let file_name = f
            .file_name()
            .unwrap()
            .to_os_string()
            .to_str()
            .unwrap()
            .to_string();

        println!(
            "{:?}",
            client
                .upload_file(
                    &[
                        ("action", "upload"),
                        ("text", "{{Dateienkategorisierung}}"),
                        ("format", "json"),
                        ("filename", &file_name),
                        ("ignorewarnings", "1"),
                        ("token", &edit_token),
                    ],
                    f.as_path(),
                )
                .await?
        );
    }

    Ok(())
}
