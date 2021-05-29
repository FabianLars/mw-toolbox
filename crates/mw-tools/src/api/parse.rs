use crate::WikiClient;
use crate::{error::ToolsError, response::parse::Parse};

pub async fn get_page_content(client: &WikiClient, page: &str) -> Result<String, ToolsError> {
    let res: Parse = client
        .get_into_json(&[("action", "parse"), ("prop", "wikitext"), ("page", page)])
        .await?;

    match res {
        Parse::Succes { parse } => Ok(parse.wikitext),
        Parse::Failure { mut errors } => Err(ToolsError::MediaWikiApi(errors.remove(0))),
    }
}
