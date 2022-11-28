use crate::api::constraint::{ComparisonType, Operator};
use crate::api::{ApiClient, Result, DEFAULT_LIMIT};
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
        self.client.get("/api/v1/segments", Some(list)).await
    }

    pub async fn get(&self, key: &str) -> Result<Segment> {
        let path = format!("/api/v1/segments/{key}", key = key);
        self.client.get(&path, None::<&()>).await
    }

    pub async fn create(&self, create: &SegmentCreateRequest) -> Result<Segment> {
        self.client.post("/api/v1/segments", Some(create)).await
    }

    pub async fn delete(&self, key: &str) -> Result<SegmentDeletion> {
        let path = format!("/api/v1/segments/{key}", key = key);
        self.client.delete(&path, None::<&()>).await
    }

    pub async fn update(&self, key: &str, update: &SegmentUpdateRequest) -> Result<Segment> {
        let path = format!("/api/v1/segments/{key}", key = key);
        self.client.put(&path, Some(update)).await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SegmentDeletion {}

#[derive(Debug, Serialize)]
pub struct SegmentCreateRequest {
    pub key: String,
    pub match_type: Match,
    pub name: String,
    pub description: String,
}

impl Default for SegmentCreateRequest {
    fn default() -> Self {
        Self {
            key: "".into(),
            name: "".into(),
            description: "".into(),
            match_type: Match::Any,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SegmentUpdateRequest {
    pub match_type: Match,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentListRequest {
    offset: usize,
    limit: usize,
    page_token: String,
}

impl Default for SegmentListRequest {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: DEFAULT_LIMIT,
            page_token: "".to_owned(),
        }
    }
}

impl SegmentListRequest {
    pub fn new() -> Self {
        Self {
            offset: 0,
            limit: DEFAULT_LIMIT,
            page_token: "".to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub enum Match {
    #[serde(rename = "ALL_MATCH_TYPE")]
    All,
    #[serde(rename = "ANY_MATCH_TYPE")]
    Any,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub key: String,
    pub match_type: Match,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub constraints: Vec<SegmentConstraint>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentConstraint {
    pub segment_key: String,
    pub id: String,
    #[serde(rename = "type")]
    pub comparison_type: ComparisonType,
    pub property: String,
    pub operator: Operator,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentList {
    pub flags: Vec<Segment>,
    pub next_page_token: String,
    pub total_count: u32,
}
