use crate::error::ApiError;
use crate::WikiClient;

pub async fn delete_pages<C: AsRef<WikiClient>>(
    client: C,
    titles: &[&str],
) -> Result<(), ApiError> {
    let client = client.as_ref();

    for title in titles {
        log::debug!(
            "{:?}",
            client
                .post_into_json(&[
                    ("action", "delete"),
                    ("reason", "automated action"),
                    ("title", title),
                ])
                .await?
        );
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
