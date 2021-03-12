use reqwest::{Client, Response};
use serde::de::DeserializeOwned;

use crate::{
    error::ToolsError,
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
    pub fn new() -> Result<Self, ToolsError> {
        Ok(Self {
            client: Client::builder()
                .cookie_store(true)
                .user_agent(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION"),
                    " using rust/reqwest",
                ))
                .build()?,
            ..Self::default()
        })
    }

    pub async fn new_logged_in<S: Into<String>>(
        url: S,
        loginname: S,
        password: S,
    ) -> Result<Self, ToolsError> {
        let mut client = Self {
            client: Client::builder()
                .cookie_store(true)
                .user_agent(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION"),
                    " using rust/reqwest",
                ))
                .build()?,
            url: url.into(),
            loginname: loginname.into(),
            password: password.into(),
            ..Self::default()
        };
        client.login().await?;
        Ok(client)
    }

    // TODO: Validate URL
    pub fn from<S: Into<String>>(url: S) -> Result<Self, ToolsError> {
        Ok(Self {
            client: Client::builder()
                .cookie_store(true)
                .user_agent(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION"),
                    " using rust/reqwest",
                ))
                .build()?,
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

    pub async fn login(&mut self) -> Result<(), ToolsError> {
        let json: Token = self
            .get_into_json(&[("action", "query"), ("meta", "tokens"), ("type", "login")])
            .await?;

        log::debug!("this should contain the requested login token: {:?}", &json);

        let token = match json.query.tokens.logintoken {
            Some(t) => t,
            None => return Err(ToolsError::TokenNotFound(format!("{:?}", json))),
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
            Login::Error { error } => {
                return Err(ToolsError::LoginFailed(error.reason.description))
            }
            Login::ErrorUnreachable { mut errors } => {
                return Err(ToolsError::LoginFailed(errors.remove(0).description))
            }
            Login::WarningsUnreachable { mut warnings } => {
                return Err(ToolsError::LoginFailed(warnings.remove(0).description))
            }
        }

        self.request_csrf_token().await
    }

    pub async fn logout(&mut self) -> Result<(), ToolsError> {
        log::debug!(
            "logout request completed: {:?}",
            self.post_into_text(&[("action", "logout"), ("token", &self.csrf_token),])
                .await?
        );

        self.csrf_token = "".to_string();

        Ok(())
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn logged_in(&self) -> bool {
        !self.csrf_token.is_empty()
    }

    pub async fn get(&self, parameters: &[(&str, &str)]) -> Result<Response, ToolsError> {
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
            .map_err(|source| ToolsError::RequestFailed { source })
    }

    pub async fn get_into_text(&self, parameters: &[(&str, &str)]) -> Result<String, ToolsError> {
        self.get(parameters)
            .await?
            .text()
            .await
            .map_err(|source| ToolsError::TextConversionFailed { source })
    }

    pub async fn get_into_json<T: DeserializeOwned>(
        &self,
        parameters: &[(&str, &str)],
    ) -> Result<T, ToolsError> {
        self.get(parameters)
            .await?
            .json()
            .await
            .map_err(|source| ToolsError::JsonConversionFailed { source })
    }

    pub async fn post(&self, parameters: &[(&str, &str)]) -> Result<Response, ToolsError> {
        let parameters = if parameters
            .iter()
            .any(|(x, y)| *x == "action" && ["delete", "edit", "move", "upload"].contains(y))
        {
            [parameters, &[("token", self.csrf_token.as_str())]].concat()
        } else {
            parameters.to_vec()
        };

        //is_send(self.client.post("https://google.com/").send());

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
            .map_err(|source| ToolsError::RequestFailed { source })
    }

    pub async fn post_into_text(&self, parameters: &[(&str, &str)]) -> Result<String, ToolsError> {
        self.post(parameters)
            .await?
            .text()
            .await
            .map_err(|source| ToolsError::TextConversionFailed { source })
    }

    pub async fn post_into_json<T: DeserializeOwned>(
        &self,
        parameters: &[(&str, &str)],
    ) -> Result<T, ToolsError> {
        self.post(parameters)
            .await?
            .json()
            .await
            .map_err(|source| ToolsError::JsonConversionFailed { source })
    }

    async fn request_csrf_token(&mut self) -> Result<(), ToolsError> {
        let res: Token = self
            .get_into_json(&[("action", "query"), ("meta", "tokens"), ("type", "csrf")])
            .await?;

        log::debug!("this should contain the requested csrf token: {:?}", &res);

        if let Some(token) = res.query.tokens.csrftoken {
            if token.as_str() == "+\\\\" {
                return Err(ToolsError::TokenNotFound(
                    "token was '+\\\\' aka empty".to_string(),
                ));
            }
            self.csrf_token = token;
        } else {
            return Err(ToolsError::TokenNotFound(format!("{:?}", res)));
        }

        Ok(())
    }

    pub async fn send_multipart(
        &self,
        parameters: &[(&str, &str)],
        file_part: reqwest::multipart::Part,
    ) -> Result<Response, ToolsError> {
        let mut form = reqwest::multipart::Form::new().part("file", file_part);
        let parameters = [
            parameters,
            &[
                ("format", "json"),
                ("formatversion", "2"),
                ("errorformat", "plaintext"),
                ("token", &self.csrf_token),
            ],
        ]
        .concat();
        for (k, v) in parameters {
            form = form.text(k.to_string(), v.to_string());
        }
        self.client
            .post(&self.url)
            .multipart(form)
            .send()
            .await
            .map_err(|source| ToolsError::RequestFailed { source })
    }
}
