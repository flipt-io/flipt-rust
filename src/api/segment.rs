use crate::api::constraint::Constraint;
use crate::api::{ApiClient, Result, DEFAULT_LIMIT, DEFAULT_NAMESPACE};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct SegmentClient<'client> {
    client: &'client ApiClient,
}

impl<'client> SegmentClient<'client> {
    pub fn new(client: &'client ApiClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, list: &SegmentListRequest) -> Result<SegmentList> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/segments",
            namespace_key = list
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string())
        );
        self.client.get(&path, Some(list)).await
    }

    pub async fn get(&self, get: &SegmentGetRequest) -> Result<Segment> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/segments/{key}",
            namespace_key = get
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            key = get.key
        );
        self.client.get(&path, None::<&()>).await
    }

    pub async fn create(&self, create: &SegmentCreateRequest) -> Result<Segment> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/segments",
            namespace_key = create
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
        );
        self.client.post(&path, Some(create)).await
    }

    pub async fn delete(&self, delete: &SegmentDeleteRequest) -> Result<SegmentDeletion> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/segments/{key}",
            namespace_key = delete
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            key = delete.key
        );
        self.client.delete(&path, None::<&()>).await
    }

    pub async fn update(&self, update: &SegmentUpdateRequest) -> Result<Segment> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/segments/{key}",
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
pub struct SegmentDeletion {}

#[derive(Debug, Default)]
pub struct SegmentGetRequest {
    pub namespace_key: Option<String>,
    pub key: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentCreateRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    pub key: String,
    pub match_type: Match,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentUpdateRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    #[serde(skip_serializing)]
    pub key: String,
    pub match_type: Match,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Default)]
pub struct SegmentDeleteRequest {
    pub namespace_key: Option<String>,
    pub key: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentListRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    pub offset: usize,
    pub limit: usize,
    pub page_token: String,
}

impl Default for SegmentListRequest {
    fn default() -> Self {
        Self {
            namespace_key: None,
            offset: 0,
            limit: DEFAULT_LIMIT,
            page_token: "".to_owned(),
        }
    }
}

impl SegmentListRequest {
    pub fn new() -> Self {
        Self {
            namespace_key: None,
            offset: 0,
            limit: DEFAULT_LIMIT,
            page_token: "".to_owned(),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub enum Match {
    #[default]
    #[serde(rename = "ALL_MATCH_TYPE")]
    All,
    #[serde(rename = "ANY_MATCH_TYPE")]
    Any,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub namespace_key: String,
    pub key: String,
    pub match_type: Match,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentList {
    pub flags: Vec<Segment>,
    pub next_page_token: String,
    pub total_count: u32,
}
