use crate::auth::{AuthClient, Authentication, AuthenticationList, Result, DEFAULT_LIMIT};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct TokenClient<'client> {
    client: &'client AuthClient,
}

impl<'client> TokenClient<'client> {
    pub fn new(client: &'client AuthClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, list: &TokenListRequest) -> Result<AuthenticationList> {
        self.client.get("/auth/v1/tokens", Some(&list)).await
    }

    pub async fn get(&self, id: &str) -> Result<Authentication> {
        let path = format!("/auth/v1/tokens/{id}");
        self.client.get(&path, None::<&()>).await
    }

    pub async fn create(&self, create: &TokenCreateRequest) -> Result<TokenCreation> {
        self.client
            .post("/auth/v1/method/token", Some(&create))
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<AuthenticationDeletion> {
        let path = format!("/auth/v1/tokens/{id}");
        self.client.delete(&path, None::<&()>).await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthenticationDeletion {}

#[derive(Debug, Serialize)]
pub struct TokenCreateRequest {
    pub name: String,
    pub description: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub namespace_key: String,
}

impl Default for TokenCreateRequest {
    fn default() -> Self {
        Self {
            name: "".into(),
            description: "".into(),
            expires_at: None,
            namespace_key: "".into(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenListRequest {
    pub offset: usize,
    pub limit: usize,
    pub page_token: String,
}

impl Default for TokenListRequest {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: DEFAULT_LIMIT,
            page_token: "".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenCreation {
    pub client_token: String,
    pub authentication: Authentication,
}
