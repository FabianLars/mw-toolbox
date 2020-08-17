use anyhow::{anyhow, Result};
use serde_json::Value;
use surf::{Client, Response};

use async_std::path::Path;
use surf::http::mime;
pub use util::PathType;
//pub use surf::Client::*;

pub mod api;
mod util;

#[derive(Clone, Debug)]
pub struct WikiClient {
    client: Client,
    url: String,
    loginname: String,
    password: String,
    //TODO: cookies: Vec<Cookie oder String>,
}

impl AsRef<WikiClient> for WikiClient {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Default for WikiClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
            ..Self::default()
        }
    }
}

impl WikiClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn new_logged_in<S: Into<String>>(url: S, loginname: S, password: S) -> Result<Self> {
        let mut client = Self {
            url: url.into(),
            loginname: loginname.into(),
            password: password.into(),
            ..Self::default()
        };
        client.login().await?;
        Ok(client)
    }

    pub fn from<S: Into<String>>(url: S) -> Self {
        Self {
            url: url.into(),
            ..Self::default()
        }
    }

    pub fn url<S: Into<String>>(&mut self, url: S) {
        self.url = url.into();
    }

    pub fn credentials<S: Into<String>>(&mut self, loginname: S, password: S) {
        self.loginname = loginname.into();
        self.password = password.into();
    }

    pub async fn login(&mut self) -> Result<()> {
        let json: Value = self
            .request_json(&[
                ("action", "login"),
                ("format", "json"),
                ("lgname", &self.loginname),
                ("lgpassword", &self.password),
            ])
            .await?;

        println!("{:?}", &json);

        let token: String = String::from(json["login"]["token"].as_str().unwrap());

        println!(
            "{:?}",
            self.request_text(&[
                ("action", "login"),
                ("format", "json"),
                ("lgname", &self.loginname),
                ("lgpassword", &self.password),
                ("lgtoken", &token),
            ])
            .await?
        );

        Ok(())
    }

    pub async fn request(&self, parameters: &[(&str, &str)]) -> Result<Response> {
        let res = self
            .client
            .send(
                surf::post(&self.url).content_type(mime::FORM).body(
                    parameters
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect::<Vec<String>>()
                        .join("&"),
                ),
            )
            .await
            .map_err(|e| anyhow!("Error requesting. Surf error: {}", e));
        dbg!(&res);
        res
    }

    pub async fn request_text(&self, parameters: &[(&str, &str)]) -> Result<String> {
        self.request(parameters)
            .await?
            .body_string()
            .await
            .map_err(|e| anyhow!("Error getting text from Response. Surf error: {}", e))
    }

    pub async fn request_json(&self, parameters: &[(&str, &str)]) -> Result<Value> {
        self.request(parameters)
            .await?
            .body_json()
            .await
            .map_err(|e| anyhow!("Error getting json from Response. Surf error: {}", e))
    }

    pub async fn upload_file(&self, parameters: &[(&str, &str)], file: &Path) -> Result<Response> {
        let mut req = surf::post(&self.url).build();
        req.set_query(&parameters)?;
        req.body_file(file).await?;
        self.client
            .send(req)
            .await
            .map_err(|e| anyhow!("Error sending multipart form data. Surf error: {}", e))
    }

    pub async fn get_external(&self, url: &str) -> Result<Response> {
        self.client
            .send(surf::get(url))
            .await
            .map_err(|e| anyhow::anyhow!("Error sending external get request. Surf error: {}", e))
    }

    pub async fn get_external_json(&self, url: &str) -> Result<Value> {
        self.get_external(url)
            .await?
            .body_json()
            .await
            .map_err(|e| anyhow!("Error getting json from Response. Surf error: {}", e))
    }

    pub async fn get_external_text(&self, url: &str) -> Result<String> {
        self.get_external(url)
            .await?
            .body_string()
            .await
            .map_err(|e| anyhow!("Error getting text from Response. Surf error: {}", e))
    }
}
