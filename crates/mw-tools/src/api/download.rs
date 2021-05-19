use std::{fs::File, io::Write, path::PathBuf};

use futures::StreamExt;
use regex::Regex;

use crate::{
    error::ToolsError,
    response::download::{Imageinfo, Page},
    WikiClient,
};

pub async fn download<C: AsRef<WikiClient>, S: AsRef<str>>(
    client: C,
    files: &[S],
) -> Result<(), ToolsError> {
    let client = client.as_ref();

    let path = directories_next::UserDirs::new()
        .and_then(|p| p.download_dir().map(|p| p.to_path_buf()))
        .expect("Can't find user's download folder!");

    let mut titles = String::new();
    let mut infos: Vec<crate::response::download::Page> = Vec::new();

    let rgxp = Regex::new(r#"[<>:"/\|?*]+"#).unwrap();

    for f in files {
        let f = f.as_ref();

        if !titles.is_empty() {
            titles.push('|');
        }
        titles.push_str(f);

        if titles.len() >= 1500 {
            let mut file_infos: Imageinfo = client
                .get_into_json(&[
                    ("action", "query"),
                    ("prop", "imageinfo"),
                    ("iiprop", "url"),
                    ("titles", &titles),
                ])
                .await?;

            infos.append(&mut file_infos.query.pages);

            titles.clear();
        }
    }

    futures::stream::iter(infos)
        .map(|page| download_and_save(client.client().clone(), path.clone(), rgxp.clone(), page))
        .buffer_unordered(8)
        .for_each(|entry| async move {
            if let Ok(title) = entry {
                log::info!("successfully saved {} to your downloads folder", title)
            } else {
                log::error!("{:?}", entry.unwrap_err())
            }
        })
        .await;

    Ok(())
}

async fn download_and_save(
    client: reqwest::Client,
    mut path: PathBuf,
    regex: Regex,
    page: Page,
) -> Result<String, ToolsError> {
    if let Some(imageinfo) = page.imageinfo {
        let file_contents = client.get(&imageinfo[0].url).send().await?.bytes().await?;

        let file_name = if let Some(pos) = page.title.find(':') {
            page.title.split_at(pos + 1).1
        } else {
            &page.title
        };

        path.push(regex.replace_all(file_name, "").as_ref());

        let mut file = File::create(&path)?;
        file.write_all(&file_contents)?;

        Ok(page.title)
    } else {
        Err(ToolsError::InvalidInput(page.title))
    }
}
