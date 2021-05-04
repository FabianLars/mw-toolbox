use crate::WikiClient;
use crate::{error::ToolsError, response::edit::Edit};

pub async fn nulledit<C: AsRef<WikiClient>, S: AsRef<str>>(
    client: C,
    titles: &[S],
) -> Result<(), ToolsError> {
    let client = client.as_ref();

    for title in titles {
        client
            .post(&[
                ("action", "edit"),
                ("summary", "Nulledit (broken if visible in RecentChanges)"),
                ("notminor", "true"),
                ("prependtext", ""),
                ("nocreate", ""),
                ("title", title.as_ref()),
            ])
            .await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}

pub async fn edit<C: AsRef<WikiClient>, S: AsRef<str>, R: Into<String>>(
    client: C,
    title: S,
    content: S,
    summary: Option<R>,
) -> Result<String, ToolsError> {
    let client = client.as_ref();
    let summary = summary.map_or_else(|| "".to_string(), |v| v.into());

    let res: Edit = client
        .post_into_json(&[
            ("action", "edit"),
            ("bot", ""),
            ("summary", &summary),
            ("title", title.as_ref()),
            ("text", content.as_ref()),
        ])
        .await?;

    match res {
        Edit::Succes { edit } => Ok(edit.result),
        Edit::Failure { mut errors } => Err(ToolsError::MediaWikiError(errors.remove(0))),
    }
}
