use reqwest::{Client, Response};
use serde::de::DeserializeOwned;

use crate::{
    error::ClientError,
    response::{login::Login, token::Token},
};

#[derive(Clone, Debug, Default)]
pub struct WikiClient {
    client: Client,
    url: String,
    loginname: String,
    password: String,
    csrf_token: String,
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
                .user_agent(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION"),
                    " using rust/reqwest",
                ))
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
                .user_agent(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION"),
                    " using rust/reqwest",
                ))
                .build()
                .map_err(|source| ClientError::BuildFailed { source })?,
            url: url.into(),
            loginname: loginname.into(),
            password: password.into(),
            ..Self::default()
        };
        client.login().await?;
        Ok(client)
    }

    // TODO: Validate URL
    pub fn from<S: Into<String>>(url: S) -> Result<Self, ClientError> {
        Ok(Self {
            client: Client::builder()
                .cookie_store(true)
                .user_agent(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION"),
                    " using rust/reqwest",
                ))
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
        let json: Token = self
            .get_into_json(&[("action", "query"), ("meta", "tokens"), ("type", "login")])
            .await?;

        log::debug!("this should contain the requested login token: {:?}", &json);

        let token = match json.query.tokens.logintoken {
            Some(t) => t,
            None => return Err(ClientError::TokenNotFound(format!("{:?}", json))),
        };

        let res: Login = self
            .post_into_json(&[
                ("action", "login"),
                ("lgname", &self.loginname),
                ("lgpassword", &self.password),
                ("lgtoken", &token),
            ])
            .await?;

        log::debug!("login request completed: {:?}", res);

        match res {
            Login::Login { .. } => {}
            Login::LoginError { error } => return Err(ClientError::LoginFailed(error.reason)),
            Login::Error { errors } => {
                return Err(ClientError::LoginFailed(format!("{:?}", errors)))
            }
            Login::Warnings { warnings } => {
                return Err(ClientError::LoginFailed(format!("{:?}", warnings)))
            }
        }

        self.request_csrf_token().await
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn logged_in(&self) -> bool {
        !self.csrf_token.is_empty()
    }

    pub async fn get(&self, parameters: &[(&str, &str)]) -> Result<Response, ClientError> {
        self.client
            .get(&self.url)
            .query(&[
                ("format", "json"),
                ("formatversion", "2"),
                ("errorformat", "plaintext"),
            ])
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

    pub async fn get_into_json<T: DeserializeOwned>(
        &self,
        parameters: &[(&str, &str)],
    ) -> Result<T, ClientError> {
        self.get(parameters)
            .await?
            .json()
            .await
            .map_err(|source| ClientError::JsonConversionFailed { source })
    }

    pub async fn post(&self, parameters: &[(&str, &str)]) -> Result<Response, ClientError> {
        let parameters = if parameters
            .iter()
            .any(|(x, y)| *x == "action" && ["delete", "edit", "move", "upload"].contains(y))
        {
            [parameters, &[("token", self.csrf_token.as_str())]].concat()
        } else {
            parameters.to_vec()
        };

        self.client
            .post(&self.url)
            .query(&[
                ("format", "json"),
                ("formatversion", "2"),
                ("errorformat", "plaintext"),
            ])
            .form(&parameters)
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

    pub async fn post_into_json<T: DeserializeOwned>(
        &self,
        parameters: &[(&str, &str)],
    ) -> Result<T, ClientError> {
        self.post(parameters)
            .await?
            .json()
            .await
            .map_err(|source| ClientError::JsonConversionFailed { source })
    }

    async fn request_csrf_token(&mut self) -> Result<(), ClientError> {
        let res: Token = self
            .get_into_json(&[("action", "query"), ("meta", "tokens"), ("type", "csrf")])
            .await?;

        log::debug!("this should contain the requested csrf token: {:?}", &res);

        if let Some(token) = res.query.tokens.csrftoken {
            if token.as_str() == "+\\\\" {
                return Err(ClientError::TokenNotFound(
                    "token was '+\\\\' aka empty".to_string(),
                ));
            }
            self.csrf_token = token.to_string();
        } else {
            return Err(ClientError::TokenNotFound(format!("{:?}", res)));
        }

        Ok(())
    }

    pub async fn send_multipart(
        &self,
        paramters: &[(&str, &str)],
        form: reqwest::multipart::Form,
    ) -> Result<Response, ClientError> {
        self.client
            .post(&self.url)
            .query(&[
                ("format", "json"),
                ("formatversion", "2"),
                ("errorformat", "plaintext"),
                ("token", &self.csrf_token),
            ])
            .query(paramters)
            .multipart(form)
            .send()
            .await
            .map_err(|source| ClientError::RequestFailed { source })
    }
}
