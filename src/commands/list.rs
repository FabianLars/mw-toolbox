use serde_json::Value;

pub async fn images(destination: std::path::PathBuf, loginname: String, loginpassword: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let mut has_next: bool = true;
    let mut continue_from = String::new();
    let mut all_images: Vec<String> = Vec::new();

    crate::helpers::wiki::wiki_login(&client, loginname, loginpassword).await?;

    while has_next {
        let res = client.get(&("https://leagueoflegends.fandom.com/de/api.php?action=query&format=json&list=allimages&ailimit=5000".to_string() + &continue_from)).send().await?.text().await?;
        let json: Value = serde_json::from_str(&res)?;
        for x in json["query"]["allimages"].as_array().unwrap().iter() {
            all_images.push(x["title"].as_str().unwrap().to_string())
        }

        match json.get("query-continue") {
            None => has_next = false,
            _ => {
                continue_from = "&aifrom=".to_string()
                    + &json["query-continue"]["allimages"]["aifrom"]
                        .as_str()
                        .unwrap()
            }
        }
    }
    ::serde_json::to_writer(&std::fs::File::create(destination)?, &all_images)?;

    Ok(())
}
