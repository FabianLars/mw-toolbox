use std::path::Path;

use crate::{error::ToolsError, response::upload::Upload, WikiClient};

pub async fn upload<C: AsRef<WikiClient>, P: AsRef<Path>, S: Into<String>>(
    client: C,
    file: P,
    text: Option<S>,
) -> Result<String, ToolsError> {
    let client = client.as_ref();
    let file = file.as_ref();
    let text = match text {
        Some(s) => s.into(),
        None => "".to_string(),
    };

    let file_name = match file.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => {
            return Err(ToolsError::InvalidInput(format!(
                "Invalid file name: {:?}",
                file.display()
            )))
        }
    };

    let file_content = tokio::fs::read(file).await?;
    let part = reqwest::multipart::Part::bytes(file_content).file_name(file_name.clone());

    let response: Upload = client
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
        .json()
        .await?;

    return match response {
        Upload::Succes { upload } => Ok(upload.result),
        Upload::Failure { mut errors } => Err(ToolsError::MediaWikiError(errors.remove(0))),
    };
}

pub async fn upload_multiple<C: AsRef<WikiClient>, P: AsRef<Path>>(
    client: C,
    files: &[P],
    text: Option<String>,
) -> Result<(), ToolsError> {
    for file in files {
        if let Err(err) = upload(&client, file, text.as_ref()).await {
            return Err(err);
        }
    }

    Ok(())
}
