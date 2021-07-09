use crate::error::ToolsError;
use crate::response::delete::Delete;
use crate::WikiClient;

pub async fn delete(
    client: &WikiClient,
    titles: &[&str],
    reason: Option<&str>,
) -> Result<(), ToolsError> {
    for title in titles {
        let res: Result<Delete, ToolsError> = client
            .post(&[
                ("action", "delete"),
                ("reason", reason.unwrap_or("automated action")),
                ("title", title),
            ])
            .await;
        match res {
            Ok(_) => log::info!("successfully deleted \"{}\"", title),
            Err(err) => match err {
                ToolsError::MediaWikiApi(err) => log::error!(
                    "deleting \"{}\" failed. reason: {} - {}",
                    title,
                    err.code,
                    err.description
                ),
                _ => log::error!("deleting \"{}\" failed. reason: {}", title, err.to_string()),
            },
        };
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
