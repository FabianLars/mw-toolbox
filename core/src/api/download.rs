use std::{fs::File, io::Write};

use crate::{error::ApiError, response::download::Imageinfo, WikiClient};

pub async fn download<C: AsRef<WikiClient>, S: AsRef<str>>(
    client: C,
    files: &[S],
) -> Result<(), ApiError> {
    let client = client.as_ref();

    let mut path = directories::UserDirs::new()
        .expect("Couldn't get home dir from system")
        .download_dir()
        .expect("Couldn't get download dir from system")
        .to_path_buf();

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

        let file_contents = client
            .client()
            .get(&file_info.query.pages[0].imageinfo.as_ref().unwrap()[0].url)
            .send()
            .await?
            .bytes()
            .await?;

        path.push(f.split_at(f.find(':').unwrap_or_else(|| 0) + 1).1);

        let mut file = File::create(&path).expect("Couldn't create file");
        file.write_all(&file_contents)?;

        path.pop();
    }

    Ok(())
}
