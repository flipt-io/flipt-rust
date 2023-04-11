use crate::api::{ApiClient, Result, DEFAULT_NAMESPACE};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct DistributionClient<'client> {
    client: &'client ApiClient,
}

impl<'client> DistributionClient<'client> {
    pub fn new(client: &'client ApiClient) -> Self {
        Self { client }
    }

    pub async fn create(&self, create: &DistributionCreateRequest) -> Result<Distribution> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags/{flag_key}/rules/{rule_id}/distributions",
            namespace_key = create
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            flag_key = create.flag_key,
            rule_id = create.rule_id
        );
        self.client.post(&path, Some(create)).await
    }

    pub async fn delete(&self, delete: &DistributionDeleteRequest) -> Result<DistributionDeletion> {
        let path =
            format!(
            "/api/v1/namespace/{namespace_key}/flags/{flag_key}/rules/{rule_id}/distributions/{id}",
            namespace_key = delete.namespace_key.as_ref().unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            flag_key = delete.flag_key,
            rule_id = delete.rule_id,
            id = delete.id
        );
        self.client.delete(&path, None::<&()>).await
    }

    pub async fn update(&self, update: &DistributionUpdateRequest) -> Result<Distribution> {
        let path = format!("/api/v1/namespaces/{namespace_key}/flags/{flag_key}/rules/{rule_id}/distributions/{id}",
            namespace_key = update.namespace_key.as_ref().unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            flag_key = update.flag_key,
            rule_id = update.rule_id,
            id = update.id);
        self.client.put(&path, Some(update)).await
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DistributionCreateRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    #[serde(skip_serializing)]
    pub flag_key: String,
    #[serde(skip_serializing)]
    pub rule_id: String,
    pub rollout: f32,
    pub variant_id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DistributionUpdateRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    #[serde(skip_serializing)]
    pub flag_key: String,
    #[serde(skip_serializing)]
    pub rule_id: String,
    #[serde(skip_serializing)]
    pub id: String,
    pub rollout: f32,
    pub variant_id: String,
}

#[derive(Debug, Default)]
pub struct DistributionDeleteRequest {
    pub namespace_key: Option<String>,
    pub flag_key: String,
    pub rule_id: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DistributionDeletion {}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Distribution {
    pub id: String,
    pub rule_id: String,
    pub variant_id: String,
    pub rollout: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
