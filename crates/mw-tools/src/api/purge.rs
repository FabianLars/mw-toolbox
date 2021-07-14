use crate::{error::ToolsError, response::Ignore, WikiClient};

pub async fn purge(
    client: &WikiClient,
    titles: &[&str],
    recursive: bool,
) -> Result<(), ToolsError> {
    for chunk in titles.chunks(50) {
        client
            .post::<Ignore>(&[
                ("action", "purge"),
                ("forcelinkupdate", "true"),
                ("forcerecursivelinkupdate", &recursive.to_string()),
                ("titles", &chunk.join("|")),
            ])
            .await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
