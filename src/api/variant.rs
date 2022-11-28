use crate::api::{ApiClient, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct VariantClient<'client> {
    client: &'client ApiClient,
}

impl<'client> VariantClient<'client> {
    pub fn new(client: &'client ApiClient) -> Self {
        Self { client }
    }

    pub async fn create(&self, flag_key: &str, create: &VariantCreateRequest) -> Result<Variant> {
        let path = format!("/api/v1/flags/{flag_key}/variants", flag_key = flag_key);
        self.client.post(&path, Some(create)).await
    }

    pub async fn delete(&self, flag_key: &str, id: &str) -> Result<VariantDeletion> {
        let path = format!(
            "/api/v1/flags/{flag_key}/variants/{id}",
            flag_key = flag_key,
            id = id
        );
        self.client.delete(&path, None::<&()>).await
    }

    pub async fn update(
        &self,
        flag_key: &str,
        id: &str,
        update: &VariantUpdateRequest,
    ) -> Result<Variant> {
        let path = format!(
            "/api/v1/flags/{flag_key}/variants/{id}",
            flag_key = flag_key,
            id = id,
        );
        self.client.put(&path, Some(update)).await
    }
}

#[derive(Debug, Default, Serialize)]
pub struct VariantCreateRequest {
    pub key: String,
    pub name: String,
    pub description: String,
    pub attachment: String,
}

#[derive(Debug, Serialize)]
pub struct VariantUpdateRequest {
    pub key: String,
    pub name: String,
    pub description: String,
    pub attachment: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VariantDeletion {}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variant {
    pub id: String,
    pub key: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub attachment: String,
}
