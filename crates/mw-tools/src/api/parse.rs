use crate::{response::parse::Parse, Client, Error};

pub async fn get_page_content(client: &Client, page: &str) -> Result<String, Error> {
    let res: Parse = client
        .get(&[("action", "parse"), ("prop", "wikitext"), ("page", page)])
        .await?;

    Ok(res.parse.wikitext)
}
