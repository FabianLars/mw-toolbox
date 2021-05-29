use crate::error::ToolsError;
use crate::WikiClient;

pub async fn delete<C: AsRef<WikiClient>, S: AsRef<str>>(
    client: C,
    titles: &[S],
    reason: Option<&str>,
) -> Result<(), ToolsError> {
    let client = client.as_ref();

    for title in titles {
        let res = client
            .post_into_text(&[
                ("action", "delete"),
                ("reason", reason.unwrap_or("automated action")),
                ("title", title.as_ref()),
            ])
            .await?;
        log::debug!("{:?}", res);
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
