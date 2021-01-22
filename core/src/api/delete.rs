use crate::error::ApiError;
use crate::WikiClient;

pub async fn delete_pages<C: AsRef<WikiClient>, S: AsRef<str>>(
    client: C,
    titles: &[S],
) -> Result<(), ApiError> {
    let client = client.as_ref();

    for title in titles {
        log::debug!(
            "{:?}",
            client
                .post_into_text(&[
                    ("action", "delete"),
                    ("reason", "automated action"),
                    ("title", title.as_ref()),
                ])
                .await?
        );
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
