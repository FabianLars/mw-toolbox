use anyhow::{anyhow, Result};
use reqwest::{multipart::Form, Client, Response};
use serde_json::Value;

pub use util::PathType;

mod commands;
mod util;

#[derive(Clone, Debug, Default)]
pub struct Api {
    client: Client,
    url: String,
    loginname: String,
    password: String,
    wiki_headers: Option<reqwest::header::HeaderMap>,
}

impl Api {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .cookie_store(true)
                .user_agent("wtools by FabianLars (https://github.com/FabianLars/wtools)")
                .build()
                .unwrap(),
            ..Self::default()
        }
    }

    pub fn from<S: Into<String>>(url: S) -> Self {
        Self {
            client: Client::builder()
                .cookie_store(true)
                .user_agent("wtools by FabianLars (https://github.com/FabianLars/wtools)")
                .build()
                .unwrap(),
            url: url.into(),
            ..Self::default()
        }
    }

    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        self.url = url.into();
        self
    }

    pub fn credentials<S: Into<String>>(mut self, loginname: S, password: S) -> Self {
        self.loginname = loginname.into();
        self.password = password.into();
        self
    }

    pub async fn login(self) -> Result<Self> {
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
            self.request(&[
                ("action", "login"),
                ("format", "json"),
                ("lgname", &self.loginname),
                ("lgpassword", &self.password),
                ("lgtoken", &token),
            ])
            .await?
            .text()
            .await
        );

        Ok(self)
    }

    pub async fn request(&self, parameters: &[(&str, &str)]) -> Result<Response> {
        self.client
            .post(&self.url)
            .form(parameters)
            .send()
            .await
            .map_err(|e| anyhow!("Error requesting. Reqwest error: {}", e))
    }

    pub async fn request_text(&self, parameters: &[(&str, &str)]) -> Result<String> {
        self.request(parameters)
            .await?
            .text()
            .await
            .map_err(|e| anyhow!("Error getting text from Response. Reqwest error: {}", e))
    }

    pub async fn request_json(&self, parameters: &[(&str, &str)]) -> Result<Value> {
        self.request(parameters)
            .await?
            .json()
            .await
            .map_err(|e| anyhow!("Error getting json from Response. Reqwest error: {}", e))
    }

    pub async fn send_multipart(
        &self,
        parameters: &[(&str, &str)],
        form: Form,
    ) -> Result<Response> {
        self.client
            .post(&self.url)
            .query(parameters)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow!("Error sending multipart form data. Reqwest error: {}", e))
    }

    pub async fn get_external(&self, url: &str) -> Result<Response> {
        self.client
            .get(url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Error sending get request: Reqwest error: {}", e))
    }

    pub async fn get_external_json(&self, url: &str) -> Result<Value> {
        self.get_external(url)
            .await?
            .json()
            .await
            .map_err(|e| anyhow!("Error getting json from Response. Reqwest error: {}", e))
    }

    pub async fn get_external_text(&self, url: &str) -> Result<String> {
        self.get_external(url)
            .await?
            .text()
            .await
            .map_err(|e| anyhow!("Error getting text from Response. Reqwest error: {}", e))
    }
}
