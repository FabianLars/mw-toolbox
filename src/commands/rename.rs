use std::error::Error;

use serde_json::Value;

use crate::util::{props::Props, wiki};
use crate::util::error::WtoolsError;

pub async fn move_pages(props: Props) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let wiki_api_url = "https://leagueoflegends.fandom.com/de/api.php";

    let mut pages = "".to_owned();
    let input = std::fs::read_to_string(props.path.file_path()).unwrap();
    for line in input.lines() {
        if line.starts_with("replace:") {
            continue;
        }
        pages.push_str(line.split(";").nth(0).unwrap());
        pages.push_str("|");
    }
    pages.pop();

    wiki::wiki_login(&client, props.loginname, props.loginpassword).await?;

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
            return Err(Box::new(WtoolsError::RenameError("Check 'replace:' line in input file".to_string())));
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
            return Err(Box::new(WtoolsError::RenameError("Check input file or --replace array".to_string())));
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
            .await
            .expect(&format!("Can't move Page: {}", line));
        // Wenn fandom ausnahmsweise mal nen guten Tag haben sollte, wären die Abfragen zu schnell, deswegen warten wir hier vorsichtshalber eine halbe Sekunde
        std::thread::sleep(std::time::Duration::from_millis(500))
    }

    Ok(())
}
