use serde_json::Value;

pub async fn wiki_login(client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
    let botname = std::env::var("FANDOM_BOT_NAME")?;
    let botpw = std::env::var("FANDOM_BOT_PASSWORD")?;
    let wiki_api_url = "https://leagueoflegends.fandom.com/de/api.php";

    let res = client
        .post(wiki_api_url)
        .form(&[
            ("action", "login"),
            ("format", "json"),
            ("lgname", &botname),
            ("lgpassword", &botpw),
        ])
        .send()
        .await?
        .text()
        .await?;

    let json: Value = serde_json::from_str(&res)?;
    let token: String = String::from(json["login"]["token"].as_str().unwrap());

    client
        .post(wiki_api_url)
        .form(&[
            ("action", "login"),
            ("format", "json"),
            ("lgname", &botname),
            ("lgpassword", &botpw),
            ("lgtoken", &token),
        ])
        .send()
        .await?;

    Ok(())
}
