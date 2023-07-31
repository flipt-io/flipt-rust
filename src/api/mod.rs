pub mod constraint;
pub mod distribution;
pub mod evaluation;
pub mod flag;
pub mod namespace;
pub mod rollout;
pub mod rule;
pub mod segment;
pub mod variant;

use crate::error::{Error, UpstreamError};
use crate::{AuthScheme, Config};
use url::Url;

const DEFAULT_LIMIT: usize = 100;
const DEFAULT_NAMESPACE: &str = "default";

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug)]
pub struct ApiClient {
    client: reqwest::Client,
    auth_scheme: AuthScheme,
    endpoint: Url,
}

impl ApiClient {
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

    pub fn flags(&self) -> flag::FlagClient {
        flag::FlagClient::new(self)
    }

    pub fn variants(&self) -> variant::VariantClient {
        variant::VariantClient::new(self)
    }

    pub fn segments(&self) -> segment::SegmentClient {
        segment::SegmentClient::new(self)
    }

    pub fn constraints(&self) -> constraint::ConstraintClient {
        constraint::ConstraintClient::new(self)
    }

    pub fn distributions(&self) -> distribution::DistributionClient {
        distribution::DistributionClient::new(self)
    }

    pub fn rollouts(&self) -> rollout::RolloutClient {
        rollout::RolloutClient::new(self)
    }

    pub fn rules(&self) -> rule::RuleClient {
        rule::RuleClient::new(self)
    }

    pub fn evaluation(&self) -> evaluation::EvaluationClient {
        evaluation::EvaluationClient::new(self)
    }

    pub fn namespaces(&self) -> namespace::NamespaceClient {
        namespace::NamespaceClient::new(self)
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

    pub(crate) async fn put<B, R>(&self, path: &str, body: Option<&B>) -> Result<R>
    where
        B: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        let url = self.build_url(path)?;
        let mut request = self.client.put(url);
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
