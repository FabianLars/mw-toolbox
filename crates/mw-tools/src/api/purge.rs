use crate::{response::Ignore, Client, Error};

pub async fn purge(client: &Client, titles: &[&str], recursive: bool) -> Result<(), Error> {
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
