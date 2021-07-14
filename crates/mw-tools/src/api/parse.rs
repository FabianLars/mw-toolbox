use crate::{error::ToolsError, response::parse::Parse, WikiClient};

pub async fn get_page_content(client: &WikiClient, page: &str) -> Result<String, ToolsError> {
    let res: Parse = client
        .get(&[("action", "parse"), ("prop", "wikitext"), ("page", page)])
        .await?;

    Ok(res.parse.wikitext)
}
