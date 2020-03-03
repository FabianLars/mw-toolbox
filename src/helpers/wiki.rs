use serde_json::Value;

pub async fn wiki_login(client: &reqwest::Client, botname: String, botpw: String) -> Result<(), Box<dyn std::error::Error>> {
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
