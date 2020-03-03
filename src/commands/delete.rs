use serde_json::Value;

pub async fn delete_pages(props: super::super::DeleteProps) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let wiki_api_url = "https://leagueoflegends.fandom.com/de/api.php";

    let mut pages = "".to_owned();
    for line in props.input.lines() {
        pages.push_str(line);
        pages.push_str("|");
    }
    pages.pop();

    crate::helpers::wiki::wiki_login(&client, props.loginname, props.loginpassword).await?;

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
        .await?
        .text()
        .await?;

    let json: Value = serde_json::from_str(&res)?;
    let (_i, o) = json["query"]["pages"]
        .as_object()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let delete_token = String::from(o["deletetoken"].as_str().unwrap());

    for line in props.input.lines() {
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
