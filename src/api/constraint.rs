use crate::api::{ApiClient, Result, DEFAULT_NAMESPACE};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct ConstraintClient<'client> {
    client: &'client ApiClient,
}

impl<'client> ConstraintClient<'client> {
    pub fn new(client: &'client ApiClient) -> Self {
        Self { client }
    }

    pub async fn create(&self, create: &ConstraintCreateRequest) -> Result<Constraint> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/segments/{segment_key}/constraints",
            namespace_key = create
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            segment_key = create.segment_key
        );
        self.client.post(&path, Some(create)).await
    }

    pub async fn delete(&self, delete: &ConstraintDeleteRequest) -> Result<ConstraintDeletion> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/segments/{segment_key}/constraints/{id}",
            namespace_key = delete
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            segment_key = delete.segment_key,
            id = delete.id
        );
        self.client.delete(&path, None::<&()>).await
    }

    pub async fn update(&self, update: &ConstraintUpdateRequest) -> Result<Constraint> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/segments/{segment_key}/constraints/{id}",
            namespace_key = update
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            segment_key = update.segment_key,
            id = update.id
        );
        self.client.put(&path, Some(update)).await
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ConstraintCreateRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    #[serde(skip_serializing)]
    pub segment_key: String,
    pub operator: Operator,
    pub property: String,
    #[serde(rename = "type")]
    pub comparison_type: ComparisonType,
    pub value: String,
}

#[derive(Debug, Default, Serialize)]
pub struct ConstraintUpdateRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    #[serde(skip_serializing)]
    pub segment_key: String,
    #[serde(skip_serializing)]
    pub id: String,
    pub operator: Operator,
    pub property: String,
    #[serde(rename = "type")]
    pub comparison_type: ComparisonType,
    pub value: String,
}

#[derive(Debug, Default)]
pub struct ConstraintDeleteRequest {
    pub namespace_key: Option<String>,
    pub segment_key: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConstraintDeletion {}

#[derive(Debug, Default, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub enum ComparisonType {
    #[default]
    #[serde(rename = "UNKNOWN_COMPARISON_TYPE")]
    Unknown,
    #[serde(rename = "STRING_COMPARISON_TYPE")]
    String,
    #[serde(rename = "NUMBER_COMPARISON_TYPE")]
    Number,
    #[serde(rename = "BOOLEAN_COMPARISON_TYPE")]
    Boolean,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub enum Operator {
    #[default]
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "neq")]
    NotEq,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lte")]
    Lte,
    #[serde(rename = "gt")]
    Gt,
    #[serde(rename = "gte")]
    Gte,
    #[serde(rename = "empty")]
    Empty,
    #[serde(rename = "notempty")]
    NotEmpty,
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "present")]
    Present,
    #[serde(rename = "notpresent")]
    NotPresent,
    #[serde(rename = "prefix")]
    Prefix,
    #[serde(rename = "suffix")]
    Suffix,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constraint {
    pub id: String,
    pub operator: Operator,
    pub property: String,
    #[serde(rename = "type")]
    pub comparison_type: ComparisonType,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
