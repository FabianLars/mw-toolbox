use crate::{error::ApiError, WikiClient};

pub async fn rename<C: AsRef<WikiClient>>(
    client: C,
    from: Vec<String>,
    to: Option<Destination>,
    prepend: Option<String>,
    append: Option<String>,
) -> Result<(), ApiError> {
    let client = client.as_ref();

    let mut actual_destination: Vec<String> = Vec::new();

    match to {
        Some(inner_to) => match inner_to {
            Destination::Plain(dest) => {
                if from.len() != dest.len() {
                    return Err(ApiError::InvalidInput(
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
                return Err(ApiError::InvalidInput(
                    "at least one of 'to', 'prepend' or 'append' needed".to_string(),
                ));
            }
            actual_destination = from.clone();
        }
    }

    if prepend.is_some() || append.is_some() {
        actual_destination.iter_mut().for_each(|x| {
            match &prepend {
                Some(p) => x.insert_str(0, p),
                None => (),
            }
            match &append {
                Some(a) => x.push_str(a),
                None => (),
            }
        });
    }

    for (x, y) in from.iter().zip(actual_destination.iter()) {
        // TODO: Handle response
        println!("{} => MOVED TO => {}", x, y);

        log::debug!(
            "{:?}",
            client
                .post_into_text(&[
                    ("action", "move"),
                    ("from", x),
                    ("to", y),
                    ("reason", "automated action"),
                    ("movetalk", "1"),
                    ("movesubpages", "1"),
                    //("ignorewarnings", ""),
                ])
                .await?
        );

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}

pub enum Destination {
    Plain(Vec<String>),
    Replace((String, String)),
}
