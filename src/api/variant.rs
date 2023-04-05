use crate::api::{ApiClient, Result, DEFAULT_NAMESPACE};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct VariantClient<'client> {
    client: &'client ApiClient,
}

impl<'client> VariantClient<'client> {
    pub fn new(client: &'client ApiClient) -> Self {
        Self { client }
    }

    pub async fn create(&self, create: &VariantCreateRequest) -> Result<Variant> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags/{flag_key}/variants",
            namespace_key = create
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            flag_key = create.flag_key
        );
        self.client.post(&path, Some(create)).await
    }

    pub async fn delete(&self, delete: &VariantDeleteRequest) -> Result<VariantDeletion> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags/{flag_key}/variants/{id}",
            namespace_key = delete
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            flag_key = delete.flag_key,
            id = delete.id
        );
        self.client.delete(&path, None::<&()>).await
    }

    pub async fn update(&self, update: &VariantUpdateRequest) -> Result<Variant> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags/{flag_key}/variants/{id}",
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

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VariantCreateRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    #[serde(skip_serializing)]
    pub flag_key: String,
    pub key: String,
    pub name: String,
    pub description: String,
    pub attachment: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VariantUpdateRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    #[serde(skip_serializing)]
    pub flag_key: String,
    #[serde(skip_serializing)]
    pub id: String,
    pub key: String,
    pub name: String,
    pub description: String,
    pub attachment: String,
}

#[derive(Debug, Default)]
pub struct VariantDeleteRequest {
    pub namespace_key: Option<String>,
    pub flag_key: String,
    pub id: String,
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
