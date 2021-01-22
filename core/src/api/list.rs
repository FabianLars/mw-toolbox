use std::collections::HashMap;

use crate::{
    error::ApiError,
    response::list::{List, Namespaces, Querypage},
    WikiClient,
};

type Result<T, E = ApiError> = core::result::Result<T, E>;

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
                match num {
                    Ok(i) => {
                        if i < 0 {
                            continue;
                        }
                    }
                    Err(_) => {}
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
    if parameter.is_none() {
        return Err(ApiError::InvalidInput(
            "Missing bltitle: Title to search".to_string(),
        ));
    }
    get_from_api(client.as_ref(), "backlinks", "bl", parameter).await
}

pub async fn categorymembers<C: AsRef<WikiClient>>(
    client: C,
    parameter: Option<&str>,
) -> Result<Vec<String>> {
    if parameter.is_none() {
        return Err(ApiError::InvalidInput(
            "Missing cmtitle (Which category to enumerate (must include 'Category:' prefix))"
                .to_string(),
        ));
    }
    get_from_api(client.as_ref(), "categorymembers", "cm", parameter).await
}

// TODO: (fix this) Returns Error if no page embedds page. Do other functions do the same?
pub async fn embeddedin<C: AsRef<WikiClient>>(
    client: C,
    parameter: Option<&str>,
) -> Result<Vec<String>> {
    if parameter.is_none() {
        return Err(ApiError::InvalidInput(
            "Missing eititle: Title to search".to_string(),
        ));
    }
    get_from_api(client.as_ref(), "embeddedin", "ei", parameter).await
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
    if parameter.is_none() {
        return Err(ApiError::InvalidInput(
            "Missing iutitle: Title to search".to_string(),
        ));
    }
    get_from_api(client.as_ref(), "imageusage", "iu", parameter).await
}

pub async fn protectedtitles<C: AsRef<WikiClient>>(client: C) -> Result<Vec<String>> {
    get_from_api(client.as_ref(), "protectedtitles", "pt", None).await
}

pub async fn querypage<C: AsRef<WikiClient>>(
    client: C,
    parameter: Option<&str>,
) -> Result<Vec<String>> {
    if parameter.is_none() {
        return Err(ApiError::InvalidInput(
            "Missing qppage: The name of the special page. Note, this is case sensitive"
                .to_string(),
        ));
    }
    get_from_api(client.as_ref(), "querypage", "qp", parameter).await
}

pub async fn search<C: AsRef<WikiClient>>(
    client: C,
    parameter: Option<&str>,
) -> Result<Vec<String>> {
    if parameter.is_none() {
        return Err(ApiError::InvalidInput(
            "Missing srsearch: Search for all page titles (or content) that has this value"
                .to_string(),
        ));
    }
    get_from_api(client.as_ref(), "search", "sr", parameter).await
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

                for page in json.query.pages {
                    match short {
                        "eu" => {
                            results.push(format!("{}~URL~{}", page.title, page.url.unwrap()));
                        }
                        _ => results.push(page.title),
                    }
                }

                if let Some(c) = json.querycontinue {
                    continue_from = c.from
                } else {
                    has_next = false
                };
            }
        }
    }

    Ok(results)
}
