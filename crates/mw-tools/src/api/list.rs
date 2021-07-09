use std::collections::HashMap;

use crate::{
    error::ToolsError,
    response::list::{List, Namespaces, Querypage},
    WikiClient,
};

type Result<T, E = ToolsError> = core::result::Result<T, E>;

pub async fn allcategories(client: &WikiClient) -> Result<Vec<String>> {
    get_from_api(client, "allcategories", "ac", None).await
}

pub async fn allimages(client: &WikiClient) -> Result<Vec<String>> {
    get_from_api(client, "allimages", "ai", None).await
}

pub async fn allinfoboxes(client: &WikiClient) -> Result<Vec<String>> {
    get_from_api(client, "allinfoboxes", "", None).await
}

pub async fn alllinks(client: &WikiClient) -> Result<Vec<String>> {
    get_from_api(client, "alllinks", "al", None).await
}

pub async fn allpages(client: &WikiClient, parameter: Option<&str>) -> Result<Vec<String>> {
    if let Some(param) = parameter {
        if param == "all" {
            let mut temp: Vec<String> = Vec::new();
            let ns_res: Namespaces = client
                .get(&[
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

pub async fn backlinks(client: &WikiClient, parameter: &str) -> Result<Vec<String>> {
    get_from_api(
        client,
        "backlinks",
        "bl",
        Some(&format!("bltitle={}", parameter)),
    )
    .await
}

pub async fn categorymembers(client: &WikiClient, parameter: &str) -> Result<Vec<String>> {
    get_from_api(
        client,
        "categorymembers",
        "cm",
        Some(&format!("cmtitle={}", parameter)),
    )
    .await
}

pub async fn embeddedin(client: &WikiClient, parameter: &str) -> Result<Vec<String>> {
    get_from_api(
        client,
        "embeddedin",
        "ei",
        Some(&format!("eititle={}", parameter)),
    )
    .await
}

pub async fn exturlusage(client: &WikiClient) -> Result<HashMap<String, Vec<String>>> {
    let api_res = get_from_api(client, "exturlusage", "eu", None).await?;
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

pub async fn imageusage(client: &WikiClient, parameter: &str) -> Result<Vec<String>> {
    get_from_api(
        client,
        "imageusage",
        "iu",
        Some(&format!("iutitle={}", parameter)),
    )
    .await
}

pub async fn protectedtitles(client: &WikiClient) -> Result<Vec<String>> {
    get_from_api(client, "protectedtitles", "pt", None).await
}

pub async fn querypage(client: &WikiClient, parameter: &str) -> Result<Vec<String>> {
    get_from_api(
        client,
        "querypage",
        "qp",
        Some(&format!("qppage={}", parameter)),
    )
    .await
}

pub async fn search(client: &WikiClient, parameter: &str) -> Result<Vec<String>> {
    get_from_api(
        client,
        "search",
        "sr",
        Some(&format!("srsearch={}", parameter)),
    )
    .await
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
                    .get(&[
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
                    .get(&[
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
                                        page.url.unwrap_or_default()
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
                        return Err(ToolsError::MediaWikiApi(errors.remove(0)));
                    }
                }
            }
        }
    }

    Ok(results)
}
