use crate::{error::ApiError, WikiClient};

pub async fn purge<C: AsRef<WikiClient>, S: AsRef<str>>(
    client: C,
    titles: &[S],
    recursive: bool,
) -> Result<(), ApiError> {
    let client = client.as_ref();
    let titles: Vec<&str> = titles.iter().map(|s| s.as_ref()).collect();

    for chunk in titles.chunks(50) {
        client
            .post(&[
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
