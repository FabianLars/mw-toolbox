use crate::WikiClient;
use crate::{error::ToolsError, response::parse::Parse};

pub async fn get_page_content<C: AsRef<WikiClient>, S: AsRef<str>>(
    client: C,
    page: S,
) -> Result<String, ToolsError> {
    let client = client.as_ref();

    let res: Parse = client
        .get_into_json(&[
            ("action", "parse"),
            ("prop", "wikitext"),
            ("page", page.as_ref()),
        ])
        .await?;

    match res {
        Parse::Succes { parse } => Ok(parse.wikitext),
        Parse::Failure { errors } => Err(ToolsError::MediaWikiError(errors[0].code.clone())),
    }
}
