use crate::error::ApiError;
use crate::WikiClient;

pub async fn nulledit<C: AsRef<WikiClient>, S: AsRef<str>>(
    client: C,
    titles: &[S],
) -> Result<(), ApiError> {
    let client = client.as_ref();

    for title in titles {
        client
            .post(&[
                ("action", "edit"),
                ("summary", "Nulledit (broken if visible in RecentChanges)"),
                ("notminor", "true"),
                ("prependtext", ""),
                ("title", title.as_ref()),
            ])
            .await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
