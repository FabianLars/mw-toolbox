use crate::{error::ToolsError, response::edit::Edit, response::Ignore, WikiClient};

pub async fn nulledit(client: &WikiClient, titles: &[&str]) -> Result<(), ToolsError> {
    for title in titles {
        client
            .post::<Ignore>(&[
                ("action", "edit"),
                ("summary", "Nulledit (broken if visible in RecentChanges)"),
                ("notminor", "true"),
                ("prependtext", ""),
                ("nocreate", ""),
                ("title", title),
            ])
            .await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}

pub async fn edit(
    client: &WikiClient,
    title: &str,
    content: &str,
    summary: Option<&str>,
) -> Result<String, ToolsError> {
    let res: Edit = client
        .post(&[
            ("action", "edit"),
            ("bot", ""),
            ("summary", summary.unwrap_or("")),
            ("title", title),
            ("text", content),
        ])
        .await?;

    Ok(res.edit.result)
}
