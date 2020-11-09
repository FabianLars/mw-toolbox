use crate::error::ApiError;
use crate::WikiClient;

pub async fn delete_pages<C: AsRef<WikiClient>>(
    client: C,
    titles: &[&str],
) -> Result<(), ApiError> {
    let client = client.as_ref();

    for title in titles {
        client
            .post(&[
                ("action", "delete"),
                ("summary", "automated action"),
                ("title", title),
            ])
            .await?;
        tokio::time::delay_for(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
