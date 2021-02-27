use std::collections::HashMap;

use crate::{
    error::ToolsError,
    response::list::{List, Namespaces, Querypage},
    WikiClient,
};

type Result<T, E = ToolsError> = core::result::Result<T, E>;

pub async fn allcategories<C: AsRef<WikiClient>>(client: C) -> Result<Vec<String>> {
    get_from_api(client.as_ref(), "allcategories", "ac", None).await
}

pub async fn allimages<C: AsRef<WikiClient>>(client: C) -> Result<Vec<String>> {
    get_from_api(client.as_ref(), "allimages", "ai", None).await
}

pub async fn allinfoboxes<C: AsRef<WikiClient>>(client: C) -> Result<Vec<String>> {
    get_from_api(client.as_ref(), "allinfoboxes", "", None).await
}

pub async fn alllinks<C: AsRef<WikiClient>>(client: C) -> Result<Vec<String>> {
    get_from_api(client.as_ref(), "alllinks", "al", None).await
}

pub async fn allpages<C: AsRef<WikiClient>>(
    client: C,
    parameter: Option<&str>,
) -> Result<Vec<String>> {
    let client = client.as_ref();

    if let Some(param) = parameter {
        if param == "all" {
            let mut temp: Vec<String> = Vec::new();
            let ns_res: Namespaces = client
                .get_into_json(&[
                    ("action", "query"),
                    ("meta", "siteinfo"),
                    ("siprop", "namespaces"),
                ])
                .await?;
            for ns in ns_res.query.namespaces.keys() {
                let num = ns.parse::<i32>();
                if let Ok(i) = num {
                    if i < 0 {
                        continue;
                    }
                }
                temp.append(
                    &mut get_from_api(
                        client,
                        "allpages",
                        "ap",
                        Some(&format!("apnamespace={}", ns)),
                    )
                    .await?,
                );
            }
            return Ok(temp);
        } else {
            return get_from_api(
                client,
                "allpages",
                "ap",
                Some(&format!("apnamespace={}", param)),
            )
            .await;
        }
    }
    get_from_api(client, "allpages", "ap", None).await
}

pub async fn backlinks<C: AsRef<WikiClient>>(
    client: C,
    parameter: Option<&str>,
) -> Result<Vec<String>> {
    if let Some(p) = parameter {
        get_from_api(
            client.as_ref(),
            "backlinks",
            "bl",
            Some(&format!("bltitle={}", p)),
        )
        .await
    } else {
        Err(ToolsError::InvalidInput(
            "Missing bltitle: Title to search".to_string(),
        ))
    }
}

pub async fn categorymembers<C: AsRef<WikiClient>>(
    client: C,
    parameter: Option<&str>,
) -> Result<Vec<String>> {
    if let Some(p) = parameter {
        get_from_api(
            client.as_ref(),
            "categorymembers",
            "cm",
            Some(&format!("cmtitle={}", p)),
        )
        .await
    } else {
        Err(ToolsError::InvalidInput(
            "Missing cmtitle (Which category to enumerate (must include 'Category:' prefix))"
                .to_string(),
        ))
    }
}

pub async fn embeddedin<C: AsRef<WikiClient>>(
    client: C,
    parameter: Option<&str>,
) -> Result<Vec<String>> {
    if let Some(p) = parameter {
        get_from_api(
            client.as_ref(),
            "embeddedin",
            "ei",
            Some(&format!("eititle={}", p)),
        )
        .await
    } else {
        Err(ToolsError::InvalidInput(
            "Missing eititle: Title to search".to_string(),
        ))
    }
}

pub async fn exturlusage<C: AsRef<WikiClient>>(client: C) -> Result<HashMap<String, Vec<String>>> {
    let api_res = get_from_api(client.as_ref(), "exturlusage", "eu", None).await?;
    let mut results: HashMap<String, Vec<String>> = HashMap::new();

    for v in api_res {
        let split: Vec<&str> = v.split("~URL~").collect();
        results
            .entry(split[0].to_string())
            .or_insert_with(Vec::new)
            .push(split[1].to_string())
    }

    Ok(results)
}

pub async fn imageusage<C: AsRef<WikiClient>>(
    client: C,
    parameter: Option<&str>,
) -> Result<Vec<String>> {
    if let Some(p) = parameter {
        get_from_api(
            client.as_ref(),
            "imageusage",
            "iu",
            Some(&format!("iutitle={}", p)),
        )
        .await
    } else {
        Err(ToolsError::InvalidInput(
            "Missing iutitle: Title to search".to_string(),
        ))
    }
}

pub async fn protectedtitles<C: AsRef<WikiClient>>(client: C) -> Result<Vec<String>> {
    get_from_api(client.as_ref(), "protectedtitles", "pt", None).await
}

pub async fn querypage<C: AsRef<WikiClient>>(
    client: C,
    parameter: Option<&str>,
) -> Result<Vec<String>> {
    if let Some(p) = parameter {
        get_from_api(
            client.as_ref(),
            "querypage",
            "qp",
            Some(&format!("qppage={}", p)),
        )
        .await
    } else {
        Err(ToolsError::InvalidInput(
            "Missing qppage: The name of the special page. Note, this is case sensitive"
                .to_string(),
        ))
    }
}

pub async fn search<C: AsRef<WikiClient>>(
    client: C,
    parameter: Option<&str>,
) -> Result<Vec<String>> {
    if let Some(p) = parameter {
        get_from_api(
            client.as_ref(),
            "search",
            "sr",
            Some(&format!("srsearch={}", p)),
        )
        .await
    } else {
        Err(ToolsError::InvalidInput(
            "Missing srsearch: Search for all page titles (or content) that has this value"
                .to_string(),
        ))
    }
}

async fn get_from_api(
    api: &WikiClient,
    long: &str,
    short: &str,
    parameter: Option<&str>,
) -> Result<Vec<String>> {
    let mut has_next: bool = true;
    let mut continue_from = String::new();
    let mut results: Vec<String> = Vec::new();
    let cont = match short {
        "qp" | "sr" => "offset",
        _ => "continue",
    };
    let param = match parameter {
        Some(p) => {
            let temp: Vec<&str> = p.split('=').collect();
            (temp[0], temp[1])
        }
        None => ("", ""),
    };

    match short {
        "qp" => {
            while has_next {
                let json: Querypage = api
                    .get_into_json(&[
                        ("action", "query"),
                        ("list", long),
                        ("qplimit", "500"),
                        ("qpoffset", &continue_from),
                        param,
                    ])
                    .await?;

                for page in json.query.querypage.results {
                    results.push(page.title);
                }

                if let Some(c) = json.querycontinue {
                    continue_from = c.from
                } else {
                    has_next = false
                };
            }
        }
        _ => {
            while has_next {
                let json: List = api
                    .get_into_json(&[
                        ("action", "query"),
                        ("list", long),
                        (&format!("{}limit", short), "5000"),
                        (
                            &format!(
                                "{}{}",
                                if continue_from.is_empty() { "" } else { short },
                                cont
                            ),
                            &continue_from,
                        ),
                        param,
                    ])
                    .await?;

                match json {
                    List::Succes {
                        querycontinue,
                        query,
                    } => {
                        for page in query.pages {
                            match short {
                                "eu" => {
                                    results.push(format!(
                                        "{}~URL~{}",
                                        page.title,
                                        page.url.unwrap()
                                    ));
                                }
                                _ => results.push(page.title),
                            }
                        }

                        if let Some(c) = querycontinue {
                            continue_from = c.from
                        } else {
                            has_next = false
                        };
                    }
                    List::Failure { mut errors } => {
                        return Err(ToolsError::MediaWikiError(errors.remove(0)))
                    }
                }
            }
        }
    }

    Ok(results)
}
