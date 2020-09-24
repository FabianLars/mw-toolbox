use serde_json::Value;

use crate::WikiClient;

pub async fn delete_pages<C: AsRef<WikiClient>>(client: C, titles: &[&str]) -> anyhow::Result<()> {
    let client = client.as_ref();
    let json: Value = client
        .get_into_json(&[
            ("action", "query"),
            ("format", "json"),
            ("prop", "info"),
            ("intoken", "delete"),
            ("titles", &titles.join("|")),
        ])
        .await?;

    let (_i, o) = json["query"]["pages"]
        .as_object()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let delete_token = String::from(o["deletetoken"].as_str().unwrap());

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
