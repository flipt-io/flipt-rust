use crate::api::variant::Variant;
use crate::api::{ApiClient, Result, DEFAULT_LIMIT};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct FlagClient<'client> {
    client: &'client ApiClient,
}

impl<'client> FlagClient<'client> {
    pub fn new(client: &'client ApiClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, list: &FlagListRequest) -> Result<FlagList> {
        self.client.get("/api/v1/flags", Some(list)).await
    }

    pub async fn get(&self, key: &str) -> Result<Flag> {
        let path = format!("/api/v1/flags/{key}", key = key);
        self.client.get(&path, None::<&()>).await
    }

    pub async fn create(&self, create: &FlagCreateRequest) -> Result<Flag> {
        self.client.post("/api/v1/flags", Some(create)).await
    }

    pub async fn delete(&self, key: &str) -> Result<FlagDeletion> {
        let path = format!("/api/v1/flags/{key}", key = key);
        self.client.delete(&path, None::<&()>).await
    }

    pub async fn update(&self, key: &str, update: &FlagUpdateRequest) -> Result<Flag> {
        let path = format!("/api/v1/flags/{key}", key = key);
        self.client.put(&path, Some(update)).await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FlagDeletion {}

#[derive(Debug, Default, Serialize)]
pub struct FlagCreateRequest {
    pub key: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize)]
pub struct FlagUpdateRequest {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlagListRequest {
    pub offset: usize,
    pub limit: usize,
    pub page_token: String,
}

impl Default for FlagListRequest {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: DEFAULT_LIMIT,
            page_token: "".to_owned(),
        }
    }
}

impl FlagListRequest {
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
pub struct Flag {
    pub key: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlagList {
    pub flags: Vec<Flag>,
    pub next_page_token: String,
    pub total_count: usize,
}
