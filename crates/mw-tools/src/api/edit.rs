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
                ("title", title.as_ref()),
            ])
            .await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}

pub async fn edit<C: AsRef<WikiClient>, S: AsRef<str>>(
    client: C,
    title: S,
    content: S,
    summary: Option<S>,
) -> Result<String, ToolsError> {
    let client = client.as_ref();
    let summary = match summary {
        Some(s) => s.as_ref().to_string(),
        None => "automated action".to_string(),
    };

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
