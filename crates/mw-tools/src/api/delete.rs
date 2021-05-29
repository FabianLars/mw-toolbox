use crate::error::ToolsError;
use crate::WikiClient;

pub async fn delete(
    client: &WikiClient,
    titles: &[&str],
    reason: Option<&str>,
) -> Result<(), ToolsError> {
    for title in titles {
        let res = client
            .post_into_text(&[
                ("action", "delete"),
                ("reason", reason.unwrap_or("automated action")),
                ("title", title),
            ])
            .await?;
        log::debug!("{:?}", res);
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
