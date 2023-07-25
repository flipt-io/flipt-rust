use crate::api::{ApiClient, Result, DEFAULT_NAMESPACE};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct RolloutClient<'client> {
    client: &'client ApiClient,
}

impl<'client> RolloutClient<'client> {
    pub fn new(client: &'client ApiClient) -> Self {
        Self { client }
    }

    pub async fn get(&self, get: &RolloutGetRequest) -> Result<Rollout> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags/{flag_key}/rollouts/{id}",
            namespace_key = get
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            flag_key = get.flag_key,
            id = get.id
        );

        self.client.get(&path, None::<&()>).await
    }

    pub async fn create(&self, create: &RolloutCreateRequest) -> Result<Rollout> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags/{flag_key}/rollouts",
            namespace_key = create
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            flag_key = create.flag_key
        );

        self.client.post(&path, Some(create)).await
    }

    pub async fn delete(&self, delete: &RolloutDeleteRequest) -> Result<RolloutDeletion> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags/{flag_key}/rollouts/{id}",
            namespace_key = delete
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            flag_key = delete.flag_key,
            id = delete.id
        );

        self.client.delete(&path, None::<&()>).await
    }

    pub async fn update(&self, update: &RolloutUpdateRequest) -> Result<Rollout> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags/{flag_key}/rollouts/{id}",
            namespace_key = update
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            flag_key = update.flag_key,
            id = update.id
        );

        self.client.put(&path, Some(update)).await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RolloutDeletion {}

#[derive(Debug, Default, Serialize)]
pub struct RolloutGetRequest {
    pub id: String,
    pub namespace_key: Option<String>,
    pub flag_key: String,
}

#[derive(Debug, Default, Serialize)]
pub struct RolloutCreateRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    #[serde(skip_serializing)]
    pub flag_key: String,
    pub rank: usize,
    pub description: String,
    pub threshold: Option<RolloutThreshold>,
    pub segment: Option<RolloutSegment>,
}

#[derive(Debug, Default, Serialize)]
pub struct RolloutUpdateRequest {
    #[serde(skip_serializing)]
    pub id: String,
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    #[serde(skip_serializing)]
    pub flag_key: String,
    pub rank: u32,
    pub description: String,
}

#[derive(Debug, Default, Serialize)]
pub struct RolloutDeleteRequest {
    pub namespace_key: Option<String>,
    pub flag_key: String,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum RolloutType {
    #[serde(rename = "UNKNOWN_ROLLOUT_TYPE")]
    Unknown,
    #[serde(rename = "SEGMENT_ROLLOUT_TYPE")]
    Segment,
    #[serde(rename = "THRESHOLD_ROLLOUT_TYPE")]
    Threshold,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rollout {
    pub id: String,
    pub rank: u32,
    #[serde(rename = "type")]
    pub rollout_type: RolloutType,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<RolloutThreshold>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<RolloutSegment>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RolloutThreshold {
    pub percentage: f32,
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RolloutSegment {
    pub segment_key: String,
    pub value: bool,
}
