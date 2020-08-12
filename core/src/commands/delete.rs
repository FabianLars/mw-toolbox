use crate::Api;
use serde_json::Value;

impl Api {
    pub async fn delete_pages(&self, titles: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        let json: Value = self
            .request_json(&[
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
            self.request(&[
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
}
