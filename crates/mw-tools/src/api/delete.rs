use crate::error::ToolsError;
use crate::response::delete::Delete;
use crate::WikiClient;

pub async fn delete(
    client: &WikiClient,
    titles: &[&str],
    reason: Option<&str>,
) -> Result<(), ToolsError> {
    for title in titles {
        let res: Delete = client
            .post(&[
                ("action", "delete"),
                ("reason", reason.unwrap_or("automated action")),
                ("title", title),
            ])
            .await?;
        match res {
            Delete::Succes { .. } => log::info!("successfully deleted \"{}\"", title),
            Delete::Failure { mut errors } => {
                let error = errors.remove(0);
                log::error!(
                    "deleting \"{}\" failed. reason: {} - {}",
                    title,
                    error.code,
                    error.description
                )
            }
        };
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
