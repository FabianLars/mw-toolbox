pub async fn login(cfg: crate::util::config::Config) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;

    crate::util::wiki::login_persistent(&client, Some(cfg.loginname), Some(cfg.loginpassword)).await.unwrap();

    let client = reqwest::Client::builder().cookie_store(true).build()?;

    crate::util::wiki::login_persistent(&client, None, None).await.unwrap();

    let res = client
        .post("https://leagueoflegends.fandom.com/de/api.php")
        .form(&[
            ("action", "query"),
            ("format", "json"),
            ("prop", "info"),
            ("intoken", "move"),
            ("titles", "Gold"),
        ])
        .send()
        .await?.text().await?;

    println!("{:?}", res);

    Ok(())
}