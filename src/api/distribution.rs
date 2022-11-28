use crate::api::{ApiClient, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct DistributionClient<'client> {
    client: &'client ApiClient,
}

impl<'client> DistributionClient<'client> {
    pub fn new(client: &'client ApiClient) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        flag_key: &str,
        rule_id: &str,
        create: &DistributionCreateRequest,
    ) -> Result<Distribution> {
        let path = format!(
            "/api/v1/flags/{flag_key}/rules/{rule_id}/distributions",
            flag_key = flag_key,
            rule_id = rule_id,
        );
        self.client.post(&path, Some(create)).await
    }

    pub async fn delete(
        &self,
        flag_key: &str,
        rule_id: &str,
        id: &str,
    ) -> Result<DistributionDeletion> {
        let path = format!(
            "/api/v1/flags/{flag_key}/rules/{rule_id}/distributions/{id}",
            flag_key = flag_key,
            rule_id = rule_id,
            id = id
        );
        self.client.delete(&path, None::<&()>).await
    }

    pub async fn update(
        &self,
        flag_key: &str,
        rule_id: &str,
        id: &str,
        update: &DistributionUpdateRequest,
    ) -> Result<Distribution> {
        let path = format!(
            "/api/v1/flags/{flag_key}/rules/{rule_id}/distributions/{id}",
            flag_key = flag_key,
            rule_id = rule_id,
            id = id,
        );
        self.client.put(&path, Some(update)).await
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DistributionCreateRequest {
    pub rollout: f32,
    pub variant_id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DistributionUpdateRequest {
    pub rollout: f32,
    pub variant_id: String,
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
