use crate::{error::ToolsError, response::rename::Rename, WikiClient};

pub async fn rename(
    client: &WikiClient,
    from: Vec<String>,
    to: Option<Destination>,
    prepend: Option<&str>,
    append: Option<&str>,
) -> Result<(), ToolsError> {
    let mut actual_destination: Vec<String> = Vec::new();

    match to {
        Some(inner_to) => match inner_to {
            Destination::Plain(dest) => {
                if from.len() != dest.len() {
                    return Err(ToolsError::InvalidInput(
                        "amount of from/to pages is not the same".to_string(),
                    ));
                }
                actual_destination = dest;
            }
            Destination::Replace((replace, with)) => {
                from.iter().for_each(|x| {
                    actual_destination.push(x.replace(&replace, &with));
                });
            }
        },
        None => {
            if prepend.is_none() && append.is_none() {
                return Err(ToolsError::InvalidInput(
                    "at least one of 'to', 'prepend' or 'append' needed".to_string(),
                ));
            }
            actual_destination = from.clone();
        }
    }

    if prepend.is_some() || append.is_some() {
        actual_destination.iter_mut().for_each(|x| {
            if let Some(p) = &prepend {
                x.insert_str(0, p)
            }
            if let Some(a) = &prepend {
                x.push_str(a)
            }
        });
    }

    for (x, y) in from.iter().zip(actual_destination.iter()) {
        let response: Rename = client
            .post_into_json(&[
                ("action", "move"),
                ("from", x),
                ("to", y),
                ("reason", "automated action"),
                ("movetalk", ""),
                ("movesubpages", ""),
                ("ignorewarnings", ""),
            ])
            .await?;

        log::debug!("{:?}", response);

        match response {
            Rename::Succes { moved } => {
                println!("{} => MOVED TO => {}", moved.from, moved.to);
            }
            Rename::Failure { errors } => {
                println!(
                    "Error moving {} to {}: {}\nProceeding with next pages...",
                    x, y, errors[0].code
                );
            }
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
