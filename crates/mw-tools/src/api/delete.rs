use crate::response::delete::Delete;
use crate::Client;
use crate::Error;

pub async fn delete(client: &Client, titles: &[&str], reason: Option<&str>) -> Result<(), Error> {
    for title in titles {
        let res: Result<Delete, Error> = client
            .post(&[
                ("action", "delete"),
                ("reason", reason.unwrap_or("automated action")),
                ("title", title),
            ])
            .await;
        match res {
            Ok(_) => log::info!("successfully deleted \"{}\"", title),
            Err(err) => {
                if let Error::MediaWikiApi(err) = err {
                    log::error!(
                        "deleting \"{}\" failed. reason: {} - {}",
                        title,
                        err.code,
                        err.description
                    );
                } else {
                    log::error!("deleting \"{}\" failed. reason: {}", title, err.to_string());
                }
            }
        };
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
