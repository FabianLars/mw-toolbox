use std::error::Error;

use crate::{error::ApiError, PathType, WikiClient};

pub async fn move_pages<C: AsRef<WikiClient>>(
    client: C,
    path: PathType,
) -> Result<(), Box<dyn Error>> {
    let client = client.as_ref();
    let input = std::fs::read_to_string(path.file_path()?)?;

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
                ("summary", "automated action"),
                ("movetalk", "1"),
                //("ignorewarnings", ""),
            ])
            .await?;
        // Wenn fandom ausnahmsweise mal nen guten Tag haben sollte, wären die Abfragen zu schnell, deswegen warten wir hier vorsichtshalber eine halbe Sekunde
        std::thread::sleep(std::time::Duration::from_millis(500))
    }

    Ok(())
}
