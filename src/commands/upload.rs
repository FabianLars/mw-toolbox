use serde_json::Value;

#[cfg(feature = "gui")]
pub async fn from_gui(props: crate::util::props::Props) -> Result<(), crate::gui::app::ExecuteError> {
    println!("from gui");
    match upload(props).await {
        Ok(()) => Ok(()),
        Err(_) => Err(crate::gui::app::ExecuteError::Upload),
    }
}

pub async fn upload(props: crate::util::props::Props) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let wiki_api_url = "https://leagueoflegends.fandom.com/de/api.php";

    let mut pages = "".to_owned();
    let mut files: Vec<std::path::PathBuf> = Vec::new();

    crate::util::wiki::wiki_login(&client, props.loginname, props.loginpassword).await?;

    match props.path {
        super::super::PathType::File(x) => {
            files.push(x.clone());
        },
        super::super::PathType::Files(v) => files = v,
        super::super::PathType::Folder(x) => {
            let mut entries = tokio::fs::read_dir(x).await?;
            while let Some(entry) = entries.next_entry().await? {
                files.push(entry.path());
            }
        }
    }

    for f in &files {
        pages.push_str(&format!("Datei:{}|", f.file_name().expect("file_name()").to_os_string().to_str().expect("path->os_string->str")))
    }
    pages.pop();

    let res = client
        .post(wiki_api_url)
        .form(&[
            ("action", "query"),
            ("format", "json"),
            ("prop", "info"),
            ("intoken", "edit"),
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
    let edit_token = String::from(o["edittoken"].as_str().unwrap());

    for f in files {
        let file_name = f.file_name().expect("file_name()").to_os_string().to_str().expect("path->os_string->str").to_string();
        let contents = tokio::fs::read(f).await?;
        let part = reqwest::multipart::Part::bytes(contents).file_name(file_name.clone()).mime_str("multipart/form-data")?;
        let form = reqwest::multipart::Form::new()
            .part("file", part);

        println!("{:?}", client.post(wiki_api_url)
        .query(&[
                ("action", "upload"),
                ("text", "{{Dateienkategorisierung}}"),
                ("format", "json"),
                ("filename", &file_name),
                ("ignorewarnings", "1"),
                ("token", &edit_token),
            ])
            .multipart(form)
            .send().await?.text().await?);
    }

    Ok(())
}