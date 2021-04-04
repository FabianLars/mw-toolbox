use std::{fs::File, io::Write};

use crate::{error::ToolsError, response::download::Imageinfo, WikiClient};

// TODO: download_multiple() with concurrent downloads. download() for single file

pub async fn download<C: AsRef<WikiClient>, S: AsRef<str>>(
    client: C,
    files: &[S],
) -> Result<(), ToolsError> {
    let client = client.as_ref();

    let mut path = directories_next::UserDirs::new()
        .and_then(|p| p.download_dir().map(|p| p.to_path_buf()))
        .expect("Can't find user's download folder!");

    for f in files {
        let f = f.as_ref();
        let file_info: Imageinfo = client
            .get_into_json(&[
                ("action", "query"),
                ("prop", "imageinfo"),
                ("iiprop", "url"),
                ("titles", f),
            ])
            .await?;

        if let Some(imageinfo) = file_info.query.pages[0].imageinfo.as_ref() {
            let file_contents = client
                .client()
                .get(&imageinfo[0].url)
                .send()
                .await?
                .bytes()
                .await?;

            if let Some(pos) = f.find(':') {
                path.push(f.split_at(pos + 1).1);
            } else {
                path.push(f);
            }

            let mut file = File::create(&path)?;
            file.write_all(&file_contents)?;

            path.pop();
        }
    }

    Ok(())
}
