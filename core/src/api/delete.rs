use serde_json::Value;

use crate::error::ApiError;
use crate::WikiClient;

pub async fn delete_pages<C: AsRef<WikiClient>>(
    client: C,
    titles: &[&str],
) -> Result<(), ApiError> {
    let client = client.as_ref();
    let json: Value = client
        .get_into_json(&[
            ("action", "query"),
            ("prop", "info"),
            ("intoken", "delete"),
            ("titles", &titles.join("|")),
        ])
        .await?;

    let (_i, o) = json["query"]["pages"]
        .as_object()
        .ok_or(ApiError::InvalidJsonOperation(json.to_string()))?
        .into_iter()
        .next()
        .ok_or(ApiError::InvalidJsonOperation(json.to_string()))?;
    let delete_token = String::from(
        o["deletetoken"]
            .as_str()
            .ok_or(ApiError::InvalidJsonOperation(json.to_string()))?,
    );

    for title in titles {
        client
            .post(&[
                ("action", "delete"),
                ("reason", "automated action"),
                ("title", title),
                ("token", &delete_token),
            ])
            .await?;
        std::thread::sleep(std::time::Duration::from_millis(500))
    }

    Ok(())
}
