use reqwest::{Client, Response};
use serde::de::DeserializeOwned;

use crate::{
    error::ToolsError,
    response::{login::Login, token::Token, Ignore},
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

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum ApiResponse<T> {
    Success(T),
    Failure { errors: Vec<crate::response::Error> },
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
            .get(&[("action", "query"), ("meta", "tokens"), ("type", "login")])
            .await?;

        log::debug!("this should contain the requested login token: {:?}", &json);

        let token = match json.query.tokens.logintoken {
            Some(t) => t,
            None => return Err(ToolsError::TokenNotFound(format!("{:?}", json))),
        };

        let res: Login = self
            .post(&[
                ("action", "login"),
                ("lgname", &self.loginname),
                ("lgpassword", &self.password),
                ("lgtoken", &token),
            ])
            .await?;

        log::debug!("login request completed: {:?}", res);

        if let Some(r) = res.login.reason {
            return Err(ToolsError::LoginFailed(r.description));
        };

        self.request_csrf_token().await
    }

    pub async fn logout(&mut self) -> Result<(), ToolsError> {
        self.post::<Ignore>(&[("action", "logout"), ("token", &self.csrf_token)])
            .await?;

        log::debug!("logout successful");

        self.csrf_token = "".to_string();

        Ok(())
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn logged_in(&self) -> bool {
        !self.csrf_token.is_empty()
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        parameters: &[(&str, &str)],
    ) -> Result<T, ToolsError> {
        loop {
            // It's fine to create a new request every iteration, because we almost never need a second one
            let res: ApiResponse<T> = self
                .client
                .get(&self.url)
                .query(&[
                    ("format", "json"),
                    ("formatversion", "2"),
                    ("errorformat", "plaintext"),
                ])
                .query(parameters)
                .send()
                .await
                .and_then(|res| res.error_for_status())
                .map_err(ToolsError::from)?
                .json()
                .await
                .map_err(ToolsError::from)?;
            match res {
                ApiResponse::Success(r) => return Ok(r),
                ApiResponse::Failure { mut errors } => {
                    let err = errors.remove(0);
                    if &err.code == "ratelimited" {
                        log::warn!("Rate limited! Retrying after 15 seconds...");
                        tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
                        continue;
                    } else {
                        return Err(ToolsError::MediaWikiApi(err));
                    }
                }
            };
        }
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        parameters: &[(&str, &str)],
    ) -> Result<T, ToolsError> {
        let parameters = if parameters
            .iter()
            .any(|(x, y)| *x == "action" && ["delete", "edit", "move", "upload"].contains(y))
        {
            [parameters, &[("token", self.csrf_token.as_str())]].concat()
        } else {
            parameters.to_vec()
        };

        loop {
            // It's fine to create a new request every iteration, because we almost never need a second one
            let res: ApiResponse<T> = self
                .client
                .post(&self.url)
                .query(&[
                    ("format", "json"),
                    ("formatversion", "2"),
                    ("errorformat", "plaintext"),
                ])
                .form(&parameters)
                .send()
                .await
                .and_then(|res| res.error_for_status())
                .map_err(ToolsError::from)?
                .json()
                .await
                .map_err(ToolsError::from)?;
            match res {
                ApiResponse::Success(r) => return Ok(r),
                ApiResponse::Failure { mut errors } => {
                    let err = errors.remove(0);
                    if &err.code == "ratelimited" {
                        log::warn!("Rate limited! Retrying after 15 seconds...");
                        tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
                        continue;
                    } else {
                        return Err(ToolsError::MediaWikiApi(err));
                    }
                }
            };
        }
    }

    async fn request_csrf_token(&mut self) -> Result<(), ToolsError> {
        let res: Token = self
            .get(&[("action", "query"), ("meta", "tokens"), ("type", "csrf")])
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
            .map_err(Into::into)
    }
}
