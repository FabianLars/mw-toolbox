use serde_json::Value;

pub async fn move_pages(props: super::super::MoveProps) -> Result<(), Box<dyn std::error::Error>> {
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
            ("intoken", "move"),
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
    let move_token = String::from(o["movetoken"].as_str().unwrap());

    for line in props.input.lines() {
        // Needs to be edited before use
        let dest = line.replace("Vorlage:Data ", "Vorlage:Rune data/");
        let dest = dest.replace(" (Rune)", "");

        println!("{:?}xxx{:?}", line, dest);

        client
            .post(wiki_api_url)
            .form(&[
                ("action", "move"),
                ("from", line),
                ("to", &dest),
                ("format", "json"),
                ("reason", "automated action"),
                ("movetalk", "1"),
                ("token", &move_token),
            ])
            .send()
            .await
            .expect(&format!("Can't move Page: {}", line));
        std::thread::sleep(std::time::Duration::from_millis(500))
    }

    Ok(())
}