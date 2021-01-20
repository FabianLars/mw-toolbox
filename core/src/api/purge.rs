use crate::{error::ApiError, WikiClient};

pub async fn purge<C: AsRef<WikiClient>, S: AsRef<str>>(
    client: C,
    titles: &[S],
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
                ("titles", title.as_ref()),
            ])
            .await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    Ok(())
}
