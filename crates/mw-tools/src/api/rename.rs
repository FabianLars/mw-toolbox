use crate::{response::rename::Rename, Client, Error};

pub async fn rename(
    client: &Client,
    from: Vec<String>,
    to: Option<Destination>,
    prepend: Option<&str>,
    append: Option<&str>,
) -> Result<(), Error> {
    let mut actual_destination: Vec<String> = Vec::new();

    if let Some(to) = to {
        match to {
            Destination::Plain(dest) => {
                if from.len() != dest.len() {
                    return Err(Error::InvalidInput(
                        "amount of from/to pages is not the same".to_string(),
                    ));
                }
                actual_destination = dest;
            }
            Destination::Replace((replace, with)) => {
                for x in &from {
                    actual_destination.push(x.replace(&replace, &with));
                }
            }
        }
    } else {
        if prepend.is_none() && append.is_none() {
            return Err(Error::InvalidInput(
                "at least one of 'to', 'prepend' or 'append' needed".to_string(),
            ));
        }
        actual_destination = from.clone();
    }

    if prepend.is_some() || append.is_some() {
        for x in &mut actual_destination {
            if let Some(p) = &prepend {
                x.insert_str(0, p);
            }
            if let Some(a) = &prepend {
                x.push_str(a);
            }
        }
    }

    for (x, y) in from.iter().zip(actual_destination.iter()) {
        let response: Result<Rename, Error> = client
            .post(&[
                ("action", "move"),
                ("from", x),
                ("to", y),
                ("reason", "automated action"),
                ("movetalk", ""),
                ("movesubpages", ""),
                ("ignorewarnings", ""),
            ])
            .await;

        log::debug!("{:?}", response);

        match response {
            Ok(m) => println!("{} => MOVED TO => {}", m.rename.from, m.rename.to),
            Err(err) => println!(
                "Error moving {} to {}: {}\nProceeding with next pages...",
                x,
                y,
                match err {
                    Error::MediaWikiApi(err) => format!("{} - {}", err.code, err.description),
                    _ => err.code().to_string(),
                }
            ),
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}

#[derive(Debug)]
pub enum Destination {
    Plain(Vec<String>),
    Replace((String, String)),
}
