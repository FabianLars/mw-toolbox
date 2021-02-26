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

        let contents = tokio::fs::read(file).await?;
        let part = reqwest::multipart::Part::bytes(contents).file_name(file_name.clone());

        log::info!(
            "{:?}",
            client
                .send_multipart(
                    &[
                        ("action", "upload"),
                        ("text", &text),
                        ("filename", &file_name),
                        ("ignorewarnings", ""),
                    ],
                    part,
                )
                .await?
                .text()
                .await?
        );
    }

    Ok(())
}
