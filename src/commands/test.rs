pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    crate::helpers::wiki::wiki_login(&reqwest::Client::builder().cookie_store(true).build()?)
        .await?;

    Ok(())
}
