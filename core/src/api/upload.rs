use std::path::Path;

use crate::error::ApiError;
use crate::WikiClient;

pub async fn upload<C: AsRef<WikiClient>, P: AsRef<Path>>(
    client: C,
    files: Vec<P>,
    text: Option<String>,
) -> Result<(), ApiError> {
    let client = client.as_ref();
    let text = text.unwrap_or_else(|| "".to_string());

    for file in files {
        let file = file.as_ref();
        let file_name = match file.file_name() {
            Some(name) => (name.to_string_lossy().to_string()),
            None => {
                return Err(ApiError::InvalidInput(format!(
                    "Invalid file_name: {:?}",
                    file.display()
                )))
            }
        };

        let mime = match file.extension().unwrap().to_str().unwrap() {
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
        let contents = tokio::fs::read(file).await?;
        let part = reqwest::multipart::Part::bytes(contents)
            .file_name(file_name.clone())
            .mime_str(mime)?;
        let form = reqwest::multipart::Form::new().part("file", part);

        log::info!(
            "{:?}",
            client
                .send_multipart(
                    &[
                        ("action", "upload"),
                        ("text", &text),
                        ("filename", &file_name),
                        ("ignorewarnings", "1"),
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
