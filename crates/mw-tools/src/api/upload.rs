use std::path::Path;

use crate::{response::upload::Upload, Client, Error};

pub async fn upload<P: AsRef<Path>>(
    client: &Client,
    file: P,
    text: Option<&str>,
) -> Result<String, Error> {
    let file = file.as_ref();
    let text = text.unwrap_or_default();

    let file_name = file
        .file_name()
        .and_then(|f| f.to_str())
        .ok_or_else(|| Error::InvalidInput(format!("Invalid file name: {:?}", file.display())))?
        .to_string();

    let file_content = tokio::fs::read(file).await?;
    let part = reqwest::multipart::Part::bytes(file_content).file_name(file_name.clone());

    let response: Upload = client
        .send_multipart(
            &[
                ("action", "upload"),
                ("text", text),
                ("filename", &file_name),
                ("ignorewarnings", ""),
            ],
            part,
        )
        .await?
        .json()
        .await?;

    Ok(response.upload.result)
}

pub async fn upload_multiple<P: AsRef<Path>>(
    client: &Client,
    files: &[P],
    text: Option<&str>,
) -> Result<(), Error> {
    for file in files {
        if let Err(err) = upload(client, file, text).await {
            return Err(err);
        }
    }

    Ok(())
}
