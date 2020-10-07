use std::path::PathBuf;

use crate::WikiClient;
use crate::{error::ApiError, PathType};

pub async fn upload<C: AsRef<WikiClient>>(client: C, path: PathType) -> Result<(), ApiError> {
    let client = client.as_ref();
    let mut files: Vec<PathBuf> = Vec::new();

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

    let edit_token = client.get_csrf_token().await?;

    for f in files {
        let file_name = f
            .file_name()
            .unwrap()
            .to_os_string()
            .to_str()
            .unwrap()
            .to_string();

        let mime = match f.extension().unwrap().to_str().unwrap() {
            "png" => "image/png",
            "gif" => "image/gif",
            "jpg" | "jpeg" => "image/jpeg",
            "ico" => "image/ico",
            "pdf" => "application/pdf",
            "svg" => "image/svg+xml",
            "odt" => "application/vnd.oasis.opendocument.text",
            "ods" => "application/vnd.oasis.opendocument.spreadsheet",
            "odp" => "application/vnd.oasis.opendocument.presentation",
            "odg" => "application/vnd.oasis.opendocument.graphics",
            "odc" => "application/vnd.oasis.opendocument.chart",
            "odf" => "application/vnd.oasis.opendocument.formula",
            "odi" => "application/vnd.oasis.opendocument.image",
            "odm" => "application/vnd.oasis.opendocument.text-master",
            "ogg" | "oga" => "audio/ogg",
            "ogv" => "video/ogg",
            _ => "image/png",
        };
        let contents = tokio::fs::read(f).await?;
        let part = reqwest::multipart::Part::bytes(contents)
            .file_name(file_name.clone())
            .mime_str(mime)?;
        let form = reqwest::multipart::Form::new().part("file", part);

        println!(
            "{:?}",
            client
                .send_multipart(
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
                .text()
                .await?
        );
    }

    Ok(())
}
