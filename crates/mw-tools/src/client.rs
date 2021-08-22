use reqwest::{Client, Response};
use serde::de::DeserializeOwned;

use crate::{
    error::ToolsError,
    response::{login::Login, token::Token, Ignore},
};

/// A wrapper around [reqwest::Client] to interact with the mediawiki API.
///
/// Intended to use with functions provided in the [api](crate::api) module.
#[derive(Clone, Debug)]
pub struct WikiClient {
    client: Client,
    url: String,
    username: String,
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
    /// Construct a new `WikiClient`.
    ///
    /// Just like [reqwest::ClientBuilder], this method fails if a TLS backend cannot be initialized, or the resolver cannot load the system configuration.
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
            url: String::new(),
            username: String::new(),
            password: String::new(),
            csrf_token: String::new(),
        })
    }

    /// Set the URL to the mediawiki API (pointing to api.php, including the scheme).
    pub fn with_url<S: Into<String>>(mut self, url: S) -> Self {
        self.url = url.into();
        self
    }

    /// Set username and password, created via Special:BotPasswords.
    pub fn with_credentials<S: Into<String>>(mut self, username: S, password: S) -> Self {
        self.username = username.into();
        self.password = password.into();
        self
    }

    /// Set the URL to the mediawiki API (pointing to api.php, including the scheme).
    pub fn set_url<S: Into<String>>(&mut self, url: S) {
        self.url = url.into();
    }

    /// Set username and password, created via Special:BotPasswords.
    pub fn set_credentials<S: Into<String>>(&mut self, username: S, password: S) {
        self.username = username.into();
        self.password = password.into();
    }

    /// log into the mediawiki API.
    ///
    /// If successful, this method also requests an edit token which is needed for some endpoints.
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
                ("lgname", &self.username),
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

    /// Log out of the mediawiki API.
    ///
    /// Note: This doesn't remove the stored values for `url`, `username` and `password`.
    ///
    /// You generally don't need to call this, except if you want to switch the wiki or user without creating a new Client.
    pub async fn logout(&mut self) -> Result<(), ToolsError> {
        self.post::<Ignore>(&[("action", "logout"), ("token", &self.csrf_token)])
            .await?;

        log::debug!("logout successful");

        self.csrf_token = "".to_string();

        Ok(())
    }

    /// Get a reference to the inner [reqwest::Client].
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Check if the client is logged into the API.
    pub fn is_online(&self) -> bool {
        !self.csrf_token.is_empty()
    }

    /// Send a GET request with query parameters.
    /// You need to specify a response type which implements [serde::de::DeserializeOwned].
    ///
    /// If you don't care about the response, use [serde::de::IgnoredAny].
    /// # Example
    /// ```no_run
    /// # async fn test_get() -> Result<(), mw_tools::error::ToolsError> {
    /// # let client = mw_tools::WikiClient::new()?;
    /// // request the page content in wikitext form.
    ///let res: serde_json::Value = client
    ///    .get(&[("action", "parse"), ("prop", "wikitext"), ("page", "Page Title")])
    ///    .await?;
    /// # Ok(())}
    /// ```
    pub async fn get<T: DeserializeOwned>(
        &self,
        parameters: &[(&str, &str)],
    ) -> Result<T, ToolsError> {
        loop {
            // It's fine to create a new request every iteration, because we almost never need a second one.
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

    /// Send a POST request with parameters added as a form body.
    ///
    /// You need to specify a response type which implements [serde::de::DeserializeOwned].
    ///
    /// If you don't care about the response, use [serde::de::IgnoredAny].
    /// # Example
    /// ```no_run
    /// # async fn test_get() -> Result<(), mw_tools::error::ToolsError> {
    /// # use serde::de::IgnoredAny;
    /// # let client = mw_tools::WikiClient::new()?;
    /// // Purge the server cache for a page and ignore the result.
    ///client.post::<IgnoredAny>(&[
    ///    ("action", "purge"),
    ///    ("titles", "Page Title"),
    ///]).await?;
    /// # Ok(())}
    /// ```
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
            // It's fine to create a new request every iteration, because we almost never need a second try
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

    // get a new edit token
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

    // upload a file via a multipart/form-data request
    pub(crate) async fn send_multipart(
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
