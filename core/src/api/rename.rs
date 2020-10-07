use std::error::Error;

use serde_json::Value;

use crate::{error::ApiError, PathType, WikiClient};

pub async fn move_pages<C: AsRef<WikiClient>>(
    client: C,
    path: PathType,
) -> Result<(), Box<dyn Error>> {
    let client = client.as_ref();
    let mut pages = String::new();
    let input = std::fs::read_to_string(path.file_path()?)?;
    for line in input.lines() {
        if line.starts_with("replace:") {
            continue;
        }
        match line.split(';').next() {
            Some(l) => {
                pages.push_str(l);
                pages.push_str("|");
            }
            None => (),
        }
    }
    pages.pop();

    let json: Value = client
        .get_into_json(&[
            ("action", "query"),
            ("prop", "info"),
            ("intoken", "move"),
            ("titles", &pages),
        ])
        .await?;

    let (_i, o) = json["query"]["pages"]
        .as_object()
        .ok_or(ApiError::InvalidJsonOperation(json.to_string()))?
        .into_iter()
        .next()
        .ok_or(ApiError::InvalidJsonOperation(json.to_string()))?;
    let move_token = String::from(
        o["movetoken"]
            .as_str()
            .ok_or(ApiError::InvalidJsonOperation(o.to_string()))?,
    );

    let first_line = input
        .lines()
        .next()
        .ok_or(ApiError::EmptyInput)?
        .starts_with("replace:");
    let replace: Vec<String>;
    let with: Vec<String>;
    let is_regex: bool;

    if first_line {
        let temp: Vec<String> = input
            .lines()
            .next()
            .unwrap()
            .replace("replace:", "")
            .split(';')
            .map(|x| x.to_string())
            .collect();

        replace = temp[0].split(',').map(|x| x.to_string()).collect();
        with = temp[1].split(',').map(|x| x.to_string()).collect();
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
        } else if line.contains(';') {
            let mut temp = line.split(';');
            from = temp.next().unwrap().to_string();
            dest = temp.last().unwrap().to_string();
        } else {
            panic!("Check input file or --replace array");
        }

        println!("{} => MOVED TO => {}", &from, &dest);

        client
            .post(&[
                ("action", "move"),
                ("from", &from),
                ("to", &dest),
                ("reason", "automated action"),
                ("movetalk", "1"),
                //("ignorewarnings", ""),
                ("token", &move_token),
            ])
            .await?;
        // Wenn fandom ausnahmsweise mal nen guten Tag haben sollte, wären die Abfragen zu schnell, deswegen warten wir hier vorsichtshalber eine halbe Sekunde
        std::thread::sleep(std::time::Duration::from_millis(500))
    }

    Ok(())
}
