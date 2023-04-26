pub mod info;

use reqwest::Url;

use crate::{
    error::{Error, UpstreamError},
    AuthScheme, Config,
};

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug)]
pub struct MetaClient {
    client: reqwest::Client,
    auth_scheme: AuthScheme,
    endpoint: Url,
}

impl MetaClient {
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
    pub fn info(&self) -> info::InfoClient {
        info::InfoClient::new(self)
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
