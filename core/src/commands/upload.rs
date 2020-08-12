use reqwest::multipart::{Form, Part};
use serde_json::Value;

use crate::util::PathType;
use crate::Api;

impl Api {
    pub async fn upload(&self, path: PathType) -> anyhow::Result<()> {
        let mut pages = String::new();
        let mut files: Vec<std::path::PathBuf> = Vec::new();

        match path {
            PathType::File(x) => {
                files.push(x);
            }
            PathType::Files(v) => files = v,
            PathType::Folder(x) => {
                let mut entries = tokio::fs::read_dir(x).await?;
                while let Some(entry) = entries.next_entry().await? {
                    files.push(entry.path());
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

        let json: Value = self
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
            let contents = tokio::fs::read(f).await?;
            let part = Part::bytes(contents)
                .file_name(file_name.clone())
                .mime_str("multipart/form-data")?;
            let form = Form::new().part("file", part);

            println!(
                "{:?}",
                self.send_multipart(
                    &[
                        ("action", "upload"),
                        ("text", "{{Dateienkategorisierung}}"),
                        ("format", "json"),
                        ("filename", &file_name),
                        ("ignorewarnings", "1"),
                        ("token", &edit_token),
                    ],
                    form,
                )
                .await?
            );
        }

        Ok(())
    }
}
