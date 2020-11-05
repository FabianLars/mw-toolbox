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
        std::thread::sleep(std::time::Duration::from_millis(500))
    }

    Ok(())
}
