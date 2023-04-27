pub mod api;
pub mod auth;
pub mod error;
pub mod meta;

use anyhow::Result;
use std::env;
use url::Url;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const DEFAULT_USER_AGENT: &str = "flipt-rust";

#[derive(Debug, Clone)]
pub struct Config {
    endpoint: Url,
    auth_scheme: AuthScheme,
    user_agent: String,
}

impl Config {
    pub fn new_from_env() -> Result<Self> {
        Ok(Self {
            endpoint: endpoint_from_env()?,
            auth_scheme: auth_scheme_from_env(),
            user_agent: user_agent_from_env(),
        })
    }

    pub fn new(endpoint: Url, auth_scheme: AuthScheme) -> Self {
        Self {
            endpoint,
            auth_scheme,
            user_agent: format!("{}/{}", DEFAULT_USER_AGENT, VERSION.unwrap_or("unknown")),
        }
    }

    pub fn set_user_agent(mut self, v: &str) -> Self {
        self.user_agent = v.into();
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(
            Url::parse("http://localhost:8080").unwrap(),
            AuthScheme::None,
        )
    }
}

fn endpoint_from_env() -> Result<Url, url::ParseError> {
    let endpoint = env::var("FLIPT_ENDPOINT").unwrap_or_default();
    Url::parse(&endpoint)
}

fn user_agent_from_env() -> String {
    env::var("FLIPT_USER_AGENT")
        .unwrap_or_else(|_| format!("{}/{}", DEFAULT_USER_AGENT, VERSION.unwrap_or("unknown")))
}

fn auth_scheme_from_env() -> AuthScheme {
    let token = env::var("FLIPT_AUTH_TOKEN").unwrap_or_default();
    if token.is_empty() {
        AuthScheme::None
    } else {
        AuthScheme::BearerToken(token)
    }
}

#[derive(Debug, Clone)]
pub enum AuthScheme {
    None,
    BearerToken(String),
}

impl Default for AuthScheme {
    fn default() -> Self {
        Self::None
    }
}
