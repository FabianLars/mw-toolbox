use serde_json::Value;
use std::collections::HashMap;

pub async fn allimages(props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    get_from_api(props, "allimages", "ai").await?;
    Ok(())
}

pub async fn allpages(props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    get_from_api(props, "allpages", "ap").await?;
    Ok(())
}

pub async fn alllinks(props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    get_from_api(props, "alllinks", "al").await?;
    Ok(())
}

pub async fn allcategories(props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    get_from_api(props, "allcategories", "ac").await?;
    Ok(())
}

pub async fn backlinks(mut props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    if props.parameter == "" { panic!("missing bltitle: Title to search"); }
    props.parameter = format!("&btitle={}", props.parameter);
    get_from_api(props, "backlinks", "bl").await?;
    Ok(())
}

pub async fn categorymembers(mut props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    if props.parameter == "" { panic!("missing cmtitle: Which category to enumerate (must include 'Category:' prefix"); }
    props.parameter = format!("&cmtitle={}", props.parameter);
    get_from_api(props, "categorymembers", "cm").await?;
    Ok(())
}

pub async fn embeddedin(mut props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    if props.parameter == "" { panic!("missing eititle: Title to search"); }
    props.parameter = format!("&eititle={}", props.parameter);
    get_from_api(props, "embeddedin", "ei").await?;
    Ok(())
}

pub async fn imageusage(mut props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    if props.parameter == "" { panic!("missing iutitle: Title to search"); }
    props.parameter = format!("&iutitle={}", props.parameter);
    get_from_api(props, "imageusage", "iu").await?;
    Ok(())
}

pub async fn iwbacklinks(mut props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    if props.parameter == "" { panic!("missing iwblprefix: Prefix for the interwiki"); }
    props.parameter = format!("&iwblprefix={}", props.parameter);
    get_from_api(props, "iwbacklinks", "iwbl").await?;
    Ok(())
}

pub async fn langbacklinks(mut props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    if props.parameter == "" { panic!("missing lbllang: Language for the language link"); }
    props.parameter = format!("&lbllang={}", props.parameter);
    get_from_api(props, "langbacklinks", "lbl").await?;
    Ok(())
}

pub async fn search(mut props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    if props.parameter == "" { panic!("missing srsearch: Search for all page titles (or content) that has this value"); }
    props.parameter = format!("&srsearch={}", props.parameter);
    get_from_api(props, "search", "sr").await?;
    Ok(())
}

pub async fn exturlusage(props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    /* get_from_api(props, "exturlusage", "eu").await?;
    Ok(()) */
    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let mut has_next: bool = true;
    let mut continue_from = String::new();
    let mut results: HashMap<String, Vec<String>> = HashMap::new();

    crate::helpers::wiki::wiki_login(&client, props.loginname, props.loginpassword).await?;

    while has_next {
        let json: Value = serde_json::from_str(&client.get(&("https://leagueoflegends.fandom.com/de/api.php?action=query&format=json&list=exturlusage&eulimit=5000".to_string() + &continue_from)).send().await?.text().await?)?;

        for x in json["query"]["exturlusage"].as_array().expect("as_array").iter() {
            let title = x["title"].as_str().unwrap_or("unwrap error").to_string();
            let url = x["url"].as_str().unwrap_or("unwrap error").to_string();

            if results.contains_key(&title) {
                results.get_mut(&title).unwrap().push(url);
            } else {
                results.insert(title, vec![url]);
            }
        }

        match json.get("query-continue") {
            None => has_next = false,
            Some(_) => {
                continue_from = "&euoffset=".to_string() + &json["query-continue"]["exturlusage"]["euoffset"].as_i64().expect("as_str").to_string()
            }
        }
    }


    ::serde_json::to_writer(&std::fs::File::create(props.output)?, &results)?;

    Ok(())
}

pub async fn protectedtitles(props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    get_from_api(props, "protectedtitles", "pt").await?;
    Ok(())
}

pub async fn querypage(mut props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    if props.parameter == "" { panic!("missing qppage: The name of the special page. Note, this is case sensitive"); }
    props.parameter = format!("&qppage={}", props.parameter);
    get_from_api(props, "querypage", "qp").await?;
    Ok(())
}

pub async fn wkpoppages(props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    get_from_api(props, "wkpoppages", "wk").await?;
    Ok(())
}

pub async fn unconvertedinfoboxes(props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    get_infobox_lists(props, "unconvertedinfoboxes").await?;
    Ok(())
}

pub async fn allinfoboxes(props: super::super::ListProps) -> Result<(), Box<dyn std::error::Error>> {
    get_infobox_lists(props, "allinfoboxes").await?;
    Ok(())
}

async fn get_from_api(props: super::super::ListProps, long: &str, short: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let mut has_next: bool = true;
    let mut continue_from = String::new();
    let mut results: Vec<String> = Vec::new();
    let getter = match short {
        "ac" => "*",
        _ => "title",
    };
    let from = match short {
        "eu" => "offset",
        _ => "from"
    };


    crate::helpers::wiki::wiki_login(&client, props.loginname, props.loginpassword).await?;

    while has_next {
        let temp: String;
        let json: Value = serde_json::from_str(&client.get(&(format!("https://leagueoflegends.fandom.com/de/api.php?action=query&format=json&list={}&{}limit=5000{}", long, short, props.parameter).to_string() + &continue_from)).send().await?.text().await?)?;
        if json["query"][long].is_object() {
            for (_, x) in json["query"][long].as_object().expect("as_object").iter() {
                results.push(x[getter].as_str().expect("as_str(object)").to_string())
            }
        } else if json["query"][long].is_array() {
            for x in json["query"][long].as_array().expect("as_array").iter() {
                results.push(x[getter].as_str().expect("as_str(array)").to_string())
            }
        }

        match json.get("query-continue") {
            None => has_next = false,
            Some(_) => {
                temp = match json["query-continue"][long][format!("{}{}", short, from)].as_str() {
                    Some(x) => x.to_owned(),
                    None => json["query-continue"][long][format!("{}{}", short, from)].as_i64().expect("as_i64(query-continue)").to_string()
                };
                continue_from = format!("&{}{}=", short, from).to_string() + &temp
                }
            }
        }
    ::serde_json::to_writer(&std::fs::File::create(props.output)?, &results)?;
    Ok(())
}

async fn get_infobox_lists(props: super::super::ListProps, typ: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let mut results: Vec<String> = Vec::new();

    crate::helpers::wiki::wiki_login(&client, props.loginname, props.loginpassword).await?;

    let res = client.get(&(format!("https://leagueoflegends.fandom.com/de/api.php?action=query&format=json&list={}", typ).to_string())).send().await?.text().await?;
    let json: Value = serde_json::from_str(&res)?;
    for x in json["query"][typ].as_array().unwrap().iter() {
        results.push(x["title"].as_str().unwrap().to_string())
    }

    ::serde_json::to_writer(&std::fs::File::create(props.output)?, &results)?;
    Ok(())
}