pub mod token;

use crate::error::{Error, UpstreamError};
use crate::{AuthScheme, Config};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

pub const METADATA_LABEL_NAME: &str = "io.flipt.auth.token.name";
pub const METADATA_LABEL_DESCRIPTION: &str = "io.flipt.auth.token.description";

const DEFAULT_LIMIT: usize = 100;

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug)]
pub struct AuthClient {
    client: reqwest::Client,
    auth_scheme: AuthScheme,
    endpoint: Url,
}

impl AuthClient {
    pub fn new(config: Config) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(config.user_agent)
            .build()?;

        Ok(Self {
            client,
            auth_scheme: config.auth_scheme,
            endpoint: config.endpoint,
        })
    }

    pub async fn me(&self) -> Result<Authentication> {
        self.get("/auth/v1/self", None::<&()>).await
    }

    pub fn tokens(&self) -> token::TokenClient {
        token::TokenClient::new(self)
    }

    pub(crate) async fn get<P, R>(&self, path: &str, params: Option<&P>) -> Result<R>
    where
        P: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        let url = self.build_url(path)?;
        let mut request = self.client.get(url);
        if let Some(params) = params {
            request = request.query(params);
        }
        let resp = self.send(request).await?;
        deserialize(resp).await
    }

    pub(crate) async fn post<B, R>(&self, path: &str, body: Option<&B>) -> Result<R>
    where
        B: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        let url = self.build_url(path)?;
        let mut request = self.client.post(url);
        if let Some(body) = body {
            request = request
                .header("Content-Type", "application/json")
                .json(body);
        }
        let resp = self.send(request).await?;
        deserialize(resp).await
    }

    pub(crate) async fn delete<P, R>(&self, path: &str, params: Option<&P>) -> Result<R>
    where
        P: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        let url = self.build_url(path)?;
        let mut request = self.client.delete(url);
        if let Some(params) = params {
            request = request.query(params);
        }
        let resp = self.send(request).await?;
        deserialize(resp).await
    }

    async fn send(&self, mut request: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        match self.auth_scheme {
            AuthScheme::None => {}
            AuthScheme::BearerToken(ref token) => {
                request = request.bearer_auth(token);
            }
        }
        Ok(request.send().await?)
    }

    fn build_url(&self, path: &str) -> Result<Url> {
        Ok(self.endpoint.join(path)?)
    }
}

pub async fn deserialize<T: serde::de::DeserializeOwned>(resp: reqwest::Response) -> Result<T> {
    if resp.status().is_success() {
        return Ok(resp.json::<T>().await?);
    }
    let parsed_err = resp.json::<UpstreamError>().await?;
    Err(anyhow::Error::new(Error::Upstream(parsed_err)))
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authentication {
    pub id: String,
    pub metadata: HashMap<String, String>,
    pub method: Method,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationList {
    pub authentications: Vec<Authentication>,
    pub next_page_token: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "METHOD_NONE")]
    None,
    #[serde(rename = "METHOD_TOKEN")]
    Token,
}
