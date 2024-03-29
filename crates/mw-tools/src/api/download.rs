use std::path::PathBuf;

use futures_util::{stream, StreamExt};
use regex::Regex;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{
    response::download::{Imageinfo, Page},
    Client, Error,
};

pub async fn download(client: &Client, files: &[&str]) -> Result<(), Error> {
    let path = directories_next::UserDirs::new()
        .and_then(|p| p.download_dir().map(|p| p.to_path_buf()))
        .expect("Can't find user's download folder!");

    let len = files.len() - 1;
    let mut titles = String::new();
    let mut infos: Vec<crate::response::download::Page> = Vec::new();

    let rgxp = Regex::new(r#"[<>:"/\|?*]+"#).unwrap();

    for (i, f) in files.iter().enumerate() {
        if !titles.is_empty() {
            titles.push('|');
        }
        titles.push_str(f);

        if titles.len() >= 1500 || i >= len {
            let mut file_infos: Imageinfo = client
                .get(&[
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

    stream::iter(infos)
        .for_each_concurrent(8, |page| {
            download_and_save(client.client().clone(), path.clone(), &rgxp, page)
        })
        .await;

    Ok(())
}

async fn download_and_save(client: reqwest::Client, path: PathBuf, regex: &Regex, page: Page) {
    match inner(client, path, regex, &page).await {
        Ok(_) => log::info!(
            "successfully saved \"{}\" to your downloads folder.",
            page.title
        ),
        Err(err) => log::error!("couldn't download/save \"{}\". Error: {}", page.title, err),
    }
}

async fn inner(
    client: reqwest::Client,
    mut path: PathBuf,
    regex: &Regex,
    page: &Page,
) -> Result<(), Error> {
    let imageinfo = page
        .imageinfo
        .as_ref()
        .ok_or_else(|| Error::InvalidInput("invalid wiki response".to_string()))?;
    let file_contents = client.get(&imageinfo[0].url).send().await?.bytes().await?;

    let file_name = page.title.splitn(2, ':').last().unwrap_or_default(/*This can't happen*/);

    path.push(regex.replace_all(file_name, "").as_ref());

    let mut file = File::create(&path).await?;
    file.write_all(&file_contents).await?;

    Ok(())
}
