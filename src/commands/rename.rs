use std::error::Error;

use serde_json::Value;

use crate::util::{props::Props, wiki};

pub async fn move_pages(props: Props) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let wiki_api_url = "https://leagueoflegends.fandom.com/de/api.php";

    let mut pages = "".to_owned();
    let input = std::fs::read_to_string(props.path.file_path())?;
    for line in input.lines() {
        if line.starts_with("replace:") {
            continue;
        }
        pages.push_str(line.split(";").nth(0).unwrap());
        pages.push_str("|");
    }
    pages.pop();

    wiki::wiki_login(&client, props.loginname, props.loginpassword).await?;

    let json: Value = client
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
        .json()
        .await?;

    let (_i, o) = json["query"]["pages"]
        .as_object()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let move_token = String::from(o["movetoken"].as_str().unwrap());

    let first_line = input.lines().nth(0).unwrap().starts_with("replace:");
    let replace: Vec<String>;
    let with: Vec<String>;
    let is_regex: bool;

    if first_line {
        let temp: Vec<String> = input
            .lines()
            .nth(0)
            .unwrap()
            .replace("replace:", "")
            .split(";")
            .map(|x| x.to_string())
            .collect();

        replace = temp[0].split(",").map(|x| x.to_string()).collect();
        with = temp[1].split(",").map(|x| x.to_string()).collect();
        if replace.len() != with.len() {
            panic!("Check 'replace:' line in input file");
        }
        is_regex = true;
    } else {
        replace = Vec::new();
        with = Vec::new();
        is_regex = false;
    }

    for line in input.lines() {
        if line.starts_with("replace:") {
            continue;
        }
        let from;
        let mut dest;

        if is_regex {
            from = line.to_string();
            dest = line.to_string();
            for (from, to) in replace.iter().zip(with.iter()) {
                dest = dest.replace(from, to);
            }
        } else if line.contains(";") {
            let mut temp = line.split(";");
            from = temp.nth(0).unwrap().to_string();
            dest = temp.last().unwrap().to_string();
        } else {
            panic!("Check input file or --replace array");
        }

        println!("{} => MOVED TO => {}", &from, &dest);

        client
            .post(wiki_api_url)
            .form(&[
                ("action", "move"),
                ("from", &from),
                ("to", &dest),
                ("format", "json"),
                ("reason", "automated action"),
                ("movetalk", "1"),
                //("ignorewarnings", ""),
                ("token", &move_token),
            ])
            .send()
            .await?;
        // Wenn fandom ausnahmsweise mal nen guten Tag haben sollte, wären die Abfragen zu schnell, deswegen warten wir hier vorsichtshalber eine halbe Sekunde
        std::thread::sleep(std::time::Duration::from_millis(500))
    }

    Ok(())
}
