use serde_json::{Result, Value};

pub async fn delete_pages(content: String) -> Result<()> {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    let wiki_api_url = "https://leagueoflegends.fandom.com/de/api.php";

    let mut pages = "".to_owned();
    for line in content.lines() {
        pages.push_str(line);
        pages.push_str("|");
    }
    pages.pop();

    crate::helpers::wiki::wiki_login(&client).await;

    let res = client
        .post(wiki_api_url)
        .form(&[
            ("action", "query"),
            ("format", "json"),
            ("prop", "info"),
            ("intoken", "delete"),
            ("titles", &pages),
        ])
        .send()
        .await
        .expect("Can't get delete token")
        .text()
        .await
        .expect("Can't get delete token from response body");

    let json: Value = serde_json::from_str(&res).unwrap();
    let (_i, o) = json["query"]["pages"]
        .as_object()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let delete_token = String::from(o["deletetoken"].as_str().unwrap());

    for line in content.lines() {
        client
            .post(wiki_api_url)
            .form(&[
                ("action", "delete"),
                ("reason", "semi-automated action"),
                ("title", line),
                ("token", &delete_token),
            ])
            .send()
            .await
            .expect(&format!("Can't delete Page: {}", line));
        std::thread::sleep(std::time::Duration::from_millis(500))
    }

    Ok(())
}
