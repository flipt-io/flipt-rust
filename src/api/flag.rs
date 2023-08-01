use crate::api::variant::Variant;
use crate::api::{ApiClient, Result, DEFAULT_LIMIT, DEFAULT_NAMESPACE};
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
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags",
            namespace_key = list
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string())
        );
        self.client.get(&path, Some(list)).await
    }

    pub async fn get(&self, get: &FlagGetRequest) -> Result<Flag> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags/{key}",
            namespace_key = get
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            key = get.key
        );
        self.client.get(&path, None::<&()>).await
    }

    pub async fn create(&self, create: &FlagCreateRequest) -> Result<Flag> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags",
            namespace_key = create
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
        );
        self.client.post(&path, Some(create)).await
    }

    pub async fn delete(&self, delete: &FlagDeleteRequest) -> Result<FlagDeletion> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags/{key}",
            namespace_key = delete
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            key = delete.key
        );
        self.client.delete(&path, None::<&()>).await
    }

    pub async fn update(&self, update: &FlagUpdateRequest) -> Result<Flag> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags/{key}",
            namespace_key = update
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            key = update.key
        );
        self.client.put(&path, Some(update)).await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FlagDeletion {}

#[derive(Debug, Default)]
pub struct FlagGetRequest {
    pub namespace_key: Option<String>,
    pub key: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlagCreateRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    pub key: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub r#type: Option<FlagType>,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FlagUpdateRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    #[serde(skip_serializing)]
    pub key: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Debug, Default)]
pub struct FlagDeleteRequest {
    pub namespace_key: Option<String>,
    pub key: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlagListRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    pub offset: usize,
    pub limit: usize,
    pub page_token: String,
}

impl Default for FlagListRequest {
    fn default() -> Self {
        Self {
            namespace_key: None,
            offset: 0,
            limit: DEFAULT_LIMIT,
            page_token: "".to_owned(),
        }
    }
}

impl FlagListRequest {
    pub fn new() -> Self {
        Self {
            namespace_key: None,
            offset: 0,
            limit: DEFAULT_LIMIT,
            page_token: "".to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flag {
    pub namespace_key: String,
    pub key: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub r#type: Option<FlagType>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlagList {
    pub flags: Vec<Flag>,
    pub next_page_token: String,
    pub total_count: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FlagType {
    #[default]
    #[serde(rename = "VARIANT_FLAG_TYPE")]
    Variant,
    #[serde(rename = "BOOLEAN_FLAG_TYPE")]
    Boolean,
}
