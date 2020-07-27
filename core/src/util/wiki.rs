use anyhow::{anyhow, Result};
use serde_json::Value;

use crate::util::storage;

pub async fn login(client: &reqwest::Client, botname: String, botpw: String) -> Result<()> {
    let wiki_api_url = "https://leagueoflegends.fandom.com/de/api.php";

    let json: Value = client
        .post(wiki_api_url)
        .form(&[
            ("action", "login"),
            ("format", "json"),
            ("lgname", &botname),
            ("lgpassword", &botpw),
        ])
        .send()
        .await?
        .json()
        .await?;

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

pub async fn login_persistent(
    client: &reqwest::Client,
    botname: Option<String>,
    botpw: Option<String>,
) -> Result<()> {
    let wiki_api_url = "https://leagueoflegends.fandom.com/de/api.php";

    let name;
    let pw;

    if botname.is_some() {
        name = botname.unwrap();
        storage::insert_secure(&base64::encode("lgname"), &name, "lgnamead").await?;
    } else {
        name = match storage::get_secure(&base64::encode("lgname"), "lgnamead").await {
            Ok(n) => n,
            Err(e) => panic!("Missing loginname - {}", e)
        }
    }

    if botpw.is_some() {
        pw = botpw.unwrap();
        storage::insert_secure(&base64::encode("wk_botpw"), &pw, &name).await?;
    } else {
        pw = match storage::get_secure(&base64::encode("wk_botpw"), &name).await {
            Ok(p) => p,
            Err(e) => panic!("Missing loginname - {}", e)
        }
    }

    let json: Value = client
        .post(wiki_api_url)
        .form(&[
            ("action", "login"),
            ("format", "json"),
            ("lgname", &name),
            ("lgpassword", &pw),
        ])
        .send()
        .await?
        .json()
        .await?;

    let token: String = String::from(json["login"]["token"].as_str().unwrap());

    client
        .post(wiki_api_url)
        .form(&[
            ("action", "login"),
            ("format", "json"),
            ("lgname", &name),
            ("lgpassword", &pw),
            ("lgtoken", &token),
        ])
        .send()
        .await?;

    Ok(())
}
