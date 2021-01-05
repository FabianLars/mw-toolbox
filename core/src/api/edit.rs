use crate::error::ApiError;
use crate::WikiClient;

pub async fn nulledit<C: AsRef<WikiClient>>(client: C, titles: &[&str]) -> Result<(), ApiError> {
    let client = client.as_ref();

    for title in titles {
        client
            .post(&[
                ("action", "edit"),
                ("summary", "Nulledit (broken if visible in RecentChanges)"),
                ("notminor", "true"),
                ("prependtext", ""),
                ("title", title),
            ])
            .await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
