use crate::{error::ApiError, WikiClient};

pub async fn purge<C: AsRef<WikiClient>>(
    client: C,
    titles: &[&str],
    recursive: bool,
) -> Result<(), ApiError> {
    let client = client.as_ref();

    // loop instead of multiple/all at once because fandom tends to timeout if amount of pages is > 10
    for title in titles {
        client
            .post(&[
                ("action", "purge"),
                ("forcelinkupdate", "true"),
                ("forcerecursivelinkupdate", &recursive.to_string()),
                ("titles", title),
            ])
            .await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    Ok(())
}
