use crate::api::{ApiClient, Result, DEFAULT_LIMIT};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct NamespaceClient<'client> {
    client: &'client ApiClient,
}

impl<'client> NamespaceClient<'client> {
    pub fn new(client: &'client ApiClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, list: &NamespaceListRequest) -> Result<NamespaceList> {
        self.client.get("/api/v1/namespaces", Some(list)).await
    }

    pub async fn get(&self, get: &NamespaceGetRequest) -> Result<Namespace> {
        let path = format!("/api/v1/namespaces/{key}", key = get.key);
        self.client.get(&path, None::<&()>).await
    }

    pub async fn create(&self, create: &NamespaceCreateRequest) -> Result<Namespace> {
        self.client.post("/api/v1/namespaces", Some(create)).await
    }

    pub async fn delete(&self, delete: &NamespaceDeleteRequest) -> Result<NamespaceDeletion> {
        let path = format!("/api/v1/namespaces/{key}", key = delete.key);
        self.client.delete(&path, None::<&()>).await
    }

    pub async fn update(&self, update: &NamespaceUpdateRequest) -> Result<Namespace> {
        let path = format!("/api/v1/namespaces/{key}", key = update.key);
        self.client.put(&path, Some(update)).await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct NamespaceDeletion {}

#[derive(Debug, Default)]
pub struct NamespaceGetRequest {
    pub key: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceCreateRequest {
    pub key: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceUpdateRequest {
    #[serde(skip_serializing)]
    pub key: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Default)]
pub struct NamespaceDeleteRequest {
    pub key: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceListRequest {
    pub offset: usize,
    pub limit: usize,
    pub page_token: String,
}

impl Default for NamespaceListRequest {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: DEFAULT_LIMIT,
            page_token: "".to_owned(),
        }
    }
}

impl NamespaceListRequest {
    pub fn new() -> Self {
        Self {
            offset: 0,
            limit: DEFAULT_LIMIT,
            page_token: "".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Namespace {
    pub key: String,
    pub name: String,
    pub description: String,
    pub protected: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceList {
    pub namespaces: Vec<Namespace>,
    pub next_page_token: String,
    pub total_count: usize,
}
