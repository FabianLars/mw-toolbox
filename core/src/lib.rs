use anyhow::{anyhow, Result};
use reqwest::{Client, Response};
use serde_json::Value;

pub use util::PathType;

pub mod api;
mod util;

#[derive(Clone, Debug, Default)]
pub struct WikiClient {
    client: Client,
    url: String,
    loginname: String,
    password: String,
}

impl AsRef<WikiClient> for WikiClient {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl WikiClient {
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

    pub async fn new_logged_in<S: Into<String>>(url: S, loginname: S, password: S) -> Result<Self> {
        let mut client = Self {
            client: Client::builder()
                .cookie_store(true)
                .user_agent("wtools by FabianLars (https://github.com/FabianLars/wtools)")
                .build()
                .unwrap(),
            url: url.into(),
            loginname: loginname.into(),
            password: password.into(),
        };
        client.login().await?;
        Ok(client)
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

    pub fn url<S: Into<String>>(&mut self, url: S) {
        self.url = url.into();
    }

    pub fn credentials<S: Into<String>>(&mut self, loginname: S, password: S) {
        self.loginname = loginname.into();
        self.password = password.into();
    }

    pub async fn login(&mut self) -> Result<()> {
        let json: Value = self
            .request_json(&[("action", "query"), ("meta", "tokens"), ("type", "login")])
            .await?;

        println!("{:?}", &json);

        let token: String = String::from(json["query"]["tokens"]["logintoken"].as_str().unwrap());

        println!(
            "{:?}",
            self.request_text(&[
                ("action", "clientlogin"),
                ("format", "json"),
                ("username", &self.loginname),
                ("password", &self.password),
                ("loginreturnurl", "http://example.com"),
                ("rememberMe", "1"),
                ("logintoken", &token),
            ])
            .await?
        );

        Ok(())
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
        paramters: &[(&str, &str)],
        form: reqwest::multipart::Form,
    ) -> Result<Response> {
        self.client
            .post(&self.url)
            .query(paramters)
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
