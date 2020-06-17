use std::{collections::HashMap, error::Error};

use serde_json::Value;

use crate::util::{config::*, wiki};

pub async fn allimages(cfg: Config) -> Result<Vec<String>, Box<dyn Error>> {
    get_from_api(cfg, "allimages", "ai").await
}

pub async fn allpages(mut cfg: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let namespaces = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
        "110", "111", "1200", "1201", "1202", "2000", "2001", "2002", "500", "501", "502", "503",
        "828", "829",
    ];

    match cfg.parameter.clone() {
        Some(param) => {
            if param == "all".to_string() {
                for ns in namespaces {
                    cfg.path = PathType::File(format!("wtools_output{}.json", ns).parse().unwrap());
                    cfg.parameter = Some(format!("&apnamespace={}", ns));
                }
            } else if namespaces.iter().any(|x| *x == param) {
                cfg.parameter = Some(format!("&apnamespace={}", param))
            } else {
                panic!("Unknown namespace given!".to_string());
            }
        }
        None => cfg.parameter = Some("0".to_string()),
    }
    get_from_api(cfg, "allpages", "ap").await
}

pub async fn alllinks(cfg: Config) -> Result<Vec<String>, Box<dyn Error>> {
    get_from_api(cfg, "alllinks", "al").await
}

pub async fn allcategories(cfg: Config) -> Result<Vec<String>, Box<dyn Error>> {
    get_from_api(cfg, "allcategories", "ac").await
}

pub async fn backlinks(mut cfg: Config) -> Result<Vec<String>, Box<dyn Error>> {
    match &cfg.parameter {
        Some(p) => {
            cfg.parameter = Some(format!("&btitle={}", p));
            get_from_api(cfg, "backlinks", "bl").await
        }
        None => panic!("Missing btitle (Title to search)"),
    }
}

pub async fn categorymembers(mut cfg: Config) -> Result<Vec<String>, Box<dyn Error>> {
    match &cfg.parameter {
        Some(p) => {
            cfg.parameter = Some(format!("&cmtitle={}", p));
            get_from_api(cfg, "categorymembers", "cm").await
        }
        None => panic!(
            "missing cmtitle (Which category to enumerate (must include 'Category:' prefix))"
        ),
    }
}

pub async fn embeddedin(mut cfg: Config) -> Result<Vec<String>, Box<dyn Error>> {
    match &cfg.parameter {
        Some(p) => {
            cfg.parameter = Some(format!("&eititle={}", p));
            get_from_api(cfg, "embeddedin", "ei").await
        }
        None => panic!("missing eititle: Title to search"),
    }
}

pub async fn imageusage(mut cfg: Config) -> Result<Vec<String>, Box<dyn Error>> {
    match &cfg.parameter {
        Some(p) => {
            cfg.parameter = Some(format!("&iutitle={}", p));
            get_from_api(cfg, "imageusage", "iu").await
        }
        None => panic!("missing iutitle: Title to search"),
    }
}

pub async fn iwbacklinks(mut cfg: Config) -> Result<Vec<String>, Box<dyn Error>> {
    match &cfg.parameter {
        Some(p) => {
            cfg.parameter = Some(format!("&iwblprefix={}", p));
            get_from_api(cfg, "iwbacklinks", "iwbl").await
        }
        None => panic!("missing iwblprefix: Prefix for the interwiki"),
    }
}

pub async fn langbacklinks(mut cfg: Config) -> Result<Vec<String>, Box<dyn Error>> {
    match &cfg.parameter {
        Some(p) => {
            cfg.parameter = Some(format!("&lbllang={}", p));
            get_from_api(cfg, "langbacklinks", "lbl").await
        }
        None => panic!("missing lbllang: Language for the language link"),
    }
}

pub async fn search(mut cfg: Config) -> Result<Vec<String>, Box<dyn Error>> {
    match &cfg.parameter {
        Some(p) => {
            cfg.parameter = Some(format!("&srsearch={}", p));
            get_from_api(cfg, "search", "sr").await
        }
        None => {
            panic!("missing srsearch: Search for all page titles (or content) that has this value")
        }
    }
}

pub async fn exturlusage(cfg: Config) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let mut has_next: bool = true;
    let mut continue_from = String::new();
    let mut results: HashMap<String, Vec<String>> = HashMap::new();

    wiki::login(&client, cfg.loginname, cfg.loginpassword).await?;

    while has_next {
        let json: Value = client.get(&("https://leagueoflegends.fandom.com/de/api.php?action=query&format=json&list=exturlusage&eulimit=5000".to_string() + &continue_from)).send().await?.json().await?;

        for x in json["query"]["exturlusage"].as_array().unwrap().iter() {
            let title = x["title"].as_str().unwrap().to_string();
            let url = x["url"].as_str().unwrap().to_string();

            if results.contains_key(&title) {
                results.get_mut(&title).unwrap().push(url);
            } else {
                results.insert(title, vec![url]);
            }
        }

        match json.get("query-continue") {
            None => has_next = false,
            Some(_) => {
                continue_from = "&euoffset=".to_string()
                    + &json["query-continue"]["exturlusage"]["euoffset"]
                        .as_i64()
                        .unwrap()
                        .to_string()
            }
        }
    }

    Ok(results)
}

pub async fn protectedtitles(cfg: Config) -> Result<Vec<String>, Box<dyn Error>> {
    get_from_api(cfg, "protectedtitles", "pt").await
}

pub async fn querypage(mut cfg: Config) -> Result<Vec<String>, Box<dyn Error>> {
    match &cfg.parameter {
        Some(p) => {
            cfg.parameter = Some(format!("&qppage={}", p));
            get_from_api(cfg, "querypage", "qp").await
        }
        None => {
            panic!("missing qppage: The name of the special page. Note, this is case sensitive")
        }
    }
}

pub async fn wkpoppages(cfg: Config) -> Result<Vec<String>, Box<dyn Error>> {
    get_from_api(cfg, "wkpoppages", "wk").await
}

pub async fn unconvertedinfoboxes(cfg: Config) -> Result<Vec<String>, Box<dyn Error>> {
    get_infobox_lists(cfg, "unconvertedinfoboxes").await
}

pub async fn allinfoboxes(cfg: Config) -> Result<Vec<String>, Box<dyn Error>> {
    get_infobox_lists(cfg, "allinfoboxes").await
}

async fn get_from_api(cfg: Config, long: &str, short: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let mut has_next: bool = true;
    let mut continue_from = String::new();
    let mut results: Vec<String> = Vec::new();
    let parameter = cfg.parameter.unwrap_or("".to_string());
    let getter = match short {
        "ac" => "*",
        _ => "title",
    };
    let from = match short {
        "eu" => "offset",
        _ => "from",
    };

    wiki::login(&client, cfg.loginname, cfg.loginpassword).await?;

    while has_next {
        let temp: String;
        let json: Value = client.get(&(format!("https://leagueoflegends.fandom.com/de/api.php?action=query&format=json&list={}&{}limit=5000{}{}", long, short, &parameter, &continue_from))).send().await?.json().await?;
        if json["query"][long].is_object() {
            for (_, x) in json["query"][long].as_object().unwrap().iter() {
                results.push(x[getter].as_str().unwrap().to_string())
            }
        } else if json["query"][long].is_array() {
            for x in json["query"][long].as_array().unwrap().iter() {
                results.push(x[getter].as_str().unwrap().to_string())
            }
        }

        match json.get("query-continue") {
            None => has_next = false,
            Some(_) => {
                temp = match json["query-continue"][long][format!("{}{}", short, from)].as_str() {
                    Some(x) => x.to_owned(),
                    None => json["query-continue"][long][format!("{}{}", short, from)]
                        .as_i64()
                        .unwrap()
                        .to_string(),
                };
                continue_from = format!("&{}{}=", short, from).to_string() + &temp
            }
        }
    }

    Ok(results)
}

async fn get_infobox_lists(cfg: Config, typ: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let mut results: Vec<String> = Vec::new();

    wiki::login(&client, cfg.loginname, cfg.loginpassword).await?;

    let json: Value = client
        .get(
            &(format!(
                "https://leagueoflegends.fandom.com/de/api.php?action=query&format=json&list={}",
                typ
            )
            .to_string()),
        )
        .send()
        .await?
        .json()
        .await?;

    for x in json["query"][typ].as_array().unwrap().iter() {
        results.push(x["title"].as_str().unwrap().to_string())
    }

    Ok(results)
}
