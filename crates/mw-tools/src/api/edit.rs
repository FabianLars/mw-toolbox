use crate::{response::edit::Edit, response::Ignore, Client, Error};

pub async fn nulledit(client: &Client, titles: &[&str]) -> Result<(), Error> {
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
    client: &Client,
    title: &str,
    content: &str,
    summary: Option<&str>,
) -> Result<String, Error> {
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
