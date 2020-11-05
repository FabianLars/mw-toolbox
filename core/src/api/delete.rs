use crate::error::ApiError;
use crate::WikiClient;

pub async fn delete_pages<C: AsRef<WikiClient>>(
    client: C,
    titles: &[&str],
) -> Result<(), ApiError> {
    let client = client.as_ref();
    let delete_token = client.get_csrf_token();

    for title in titles {
        client
            .post(&[
                ("action", "delete"),
                ("summary", "automated action"),
                ("title", title),
                ("token", &delete_token),
            ])
            .await?;
        std::thread::sleep(std::time::Duration::from_millis(500))
    }

    Ok(())
}
