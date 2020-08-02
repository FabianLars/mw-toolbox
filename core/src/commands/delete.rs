use serde_json::Value;

use crate::util::{config::Config, wiki};

pub async fn delete_pages(cfg: Config) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let wiki_api_url = "https://leagueoflegends.fandom.com/de/api.php";

    let mut pages = "".to_owned();
    let input = std::fs::read_to_string(cfg.path.file_path())?;
    for line in input.lines() {
        pages.push_str(line);
        pages.push_str("|");
    }
    pages.pop();

    wiki::login(&client, &cfg.loginname, &cfg.loginpassword).await?;

    let json: Value = client
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
        .json()
        .await?;

    let (_i, o) = json["query"]["pages"]
        .as_object()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let delete_token = String::from(o["deletetoken"].as_str().unwrap());

    for line in input.lines() {
        client
            .post(wiki_api_url)
            .form(&[
                ("action", "delete"),
                ("reason", "automated action"),
                ("title", line),
                ("token", &delete_token),
            ])
            .send()
            .await?;
        std::thread::sleep(std::time::Duration::from_millis(500))
    }

    Ok(())
}
