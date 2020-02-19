use serde_json::Value;

pub async fn wiki_login(client: &reqwest::Client) {
    let botname = std::env::var("FANDOM_BOT_NAME").unwrap();
    let botpw = std::env::var("FANDOM_BOT_PASSWORD").unwrap();
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
        .await
        .expect("Can't get login token")
        .text()
        .await
        .expect("Can't get login token from response body");

    let json: Value = serde_json::from_str(&res).unwrap();
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
        .await
        .expect("Can't login with token");
}
