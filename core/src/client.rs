use reqwest::{Client, Response};
use serde_json::Value;

use crate::error::ClientError;

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
    pub fn new() -> Result<Self, ClientError> {
        Ok(Self {
            client: Client::builder()
                .cookie_store(true)
                .user_agent("wtools by FabianLars (https://github.com/FabianLars/wtools)")
                .build()
                .map_err(|source| ClientError::BuildFailed { source })?,
            ..Self::default()
        })
    }

    pub async fn new_logged_in<S: Into<String>>(
        url: S,
        loginname: S,
        password: S,
    ) -> Result<Self, ClientError> {
        let mut client = Self {
            client: Client::builder()
                .cookie_store(true)
                .user_agent("wtools by FabianLars (https://github.com/FabianLars/wtools)")
                .build()
                .map_err(|source| ClientError::BuildFailed { source })?,
            url: url.into(),
            loginname: loginname.into(),
            password: password.into(),
        };
        client.login().await?;
        Ok(client)
    }

    pub fn from<S: Into<String>>(url: S) -> Result<Self, ClientError> {
        Ok(Self {
            client: Client::builder()
                .cookie_store(true)
                .user_agent("wtools by FabianLars (https://github.com/FabianLars/wtools)")
                .build()
                .map_err(|source| ClientError::BuildFailed { source })?,
            url: url.into(),
            ..Self::default()
        })
    }

    pub fn url<S: Into<String>>(&mut self, url: S) {
        self.url = url.into();
    }

    pub fn credentials<S: Into<String>>(&mut self, loginname: S, password: S) {
        self.loginname = loginname.into();
        self.password = password.into();
    }

    pub async fn login(&mut self) -> Result<(), ClientError> {
        let json: Value = self
            .get_into_json(&[("action", "query"), ("meta", "tokens"), ("type", "login")])
            .await?;

        println!("{:?}", &json);

        let token = match json["query"]["tokens"]["logintoken"].as_str() {
            Some(s) => s.to_string(),
            _ => return Err(ClientError::TokenNotFound(json.to_string())),
        };

        println!(
            "{:?}",
            self.post(&[
                ("action", "login"),
                ("lgname", &self.loginname),
                ("lgpassword", &self.password),
                ("lgtoken", &token),
                /* ("action", "login"),
                ("format", "json"),
                ("lgname", &self.loginname),
                ("lgpassword", &self.password),
                ("lgtoken", &token), */
            ])
            .await?
            .text()
            .await?
        );

        Ok(())
    }

    pub async fn get(&self, parameters: &[(&str, &str)]) -> Result<Response, ClientError> {
        self.client
            .get(&self.url)
            .query(&[("format", "json")])
            .query(parameters)
            .send()
            .await
            .map_err(|source| ClientError::RequestFailed { source })
    }

    pub async fn get_into_text(&self, parameters: &[(&str, &str)]) -> Result<String, ClientError> {
        self.get(parameters)
            .await?
            .text()
            .await
            .map_err(|source| ClientError::TextConversionFailed { source })
    }

    pub async fn get_into_json(&self, parameters: &[(&str, &str)]) -> Result<Value, ClientError> {
        self.get(parameters)
            .await?
            .json()
            .await
            .map_err(|source| ClientError::JsonConversionFailed { source })
    }

    pub async fn post(&self, parameters: &[(&str, &str)]) -> Result<Response, ClientError> {
        self.client
            .post(&self.url)
            .query(&[("format", "json")])
            .form(parameters)
            .send()
            .await
            .map_err(|source| ClientError::RequestFailed { source })
    }

    pub async fn post_into_text(&self, parameters: &[(&str, &str)]) -> Result<String, ClientError> {
        self.post(parameters)
            .await?
            .text()
            .await
            .map_err(|source| ClientError::TextConversionFailed { source })
    }

    pub async fn post_into_json(&self, parameters: &[(&str, &str)]) -> Result<Value, ClientError> {
        self.post(parameters)
            .await?
            .json()
            .await
            .map_err(|source| ClientError::JsonConversionFailed { source })
    }

    pub async fn send_multipart(
        &self,
        paramters: &[(&str, &str)],
        form: reqwest::multipart::Form,
    ) -> Result<Response, ClientError> {
        self.client
            .post(&self.url)
            .query(paramters)
            .multipart(form)
            .send()
            .await
            .map_err(|source| ClientError::RequestFailed { source })
    }

    pub async fn get_external(&self, url: &str) -> Result<Response, ClientError> {
        self.client
            .get(url)
            .send()
            .await
            .map_err(|source| ClientError::RequestFailed { source })
    }

    pub async fn get_external_json(&self, url: &str) -> Result<Value, ClientError> {
        self.get_external(url)
            .await?
            .json()
            .await
            .map_err(|source| ClientError::JsonConversionFailed { source })
    }

    pub async fn get_external_text(&self, url: &str) -> Result<String, ClientError> {
        self.get_external(url)
            .await?
            .text()
            .await
            .map_err(|source| ClientError::TextConversionFailed { source })
    }
}
