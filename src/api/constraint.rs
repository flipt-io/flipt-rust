use crate::api::{ApiClient, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct ConstraintClient<'client> {
    client: &'client ApiClient,
}

impl<'client> ConstraintClient<'client> {
    pub fn new(client: &'client ApiClient) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        segment_key: &str,
        create: &ConstraintCreateRequest,
    ) -> Result<Constraint> {
        let path = format!(
            "/api/v1/segments/{segment_key}/constraints",
            segment_key = segment_key
        );
        self.client.post(&path, Some(create)).await
    }

    pub async fn delete(&self, segment_key: &str, id: &str) -> Result<ConstraintDeletion> {
        let path = format!(
            "/api/v1/segments/{segment_key}/constraints/{id}",
            segment_key = segment_key,
            id = id
        );
        self.client.delete(&path, None::<&()>).await
    }

    pub async fn update(
        &self,
        segment_key: &str,
        id: &str,
        update: &ConstraintUpdateRequest,
    ) -> Result<Constraint> {
        let path = format!(
            "/api/v1/segments/{segment_key}/constraints/{id}",
            segment_key = segment_key,
            id = id
        );
        self.client.put(&path, Some(update)).await
    }
}

#[derive(Debug, Serialize)]
pub struct ConstraintCreateRequest {
    pub operator: Operator,
    pub property: String,
    #[serde(rename = "type")]
    pub comparison_type: ComparisonType,
    pub value: String,
}

impl Default for ConstraintCreateRequest {
    fn default() -> Self {
        Self {
            property: "".into(),
            value: "".into(),
            operator: Operator::Eq,
            comparison_type: ComparisonType::Unknown,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ConstraintUpdateRequest {
    pub operator: Operator,
    pub property: String,
    #[serde(rename = "type")]
    pub comparison_type: ComparisonType,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConstraintDeletion {}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub enum ComparisonType {
    #[serde(rename = "UNKNOWN_COMPARISON_TYPE")]
    Unknown,
    #[serde(rename = "STRING_COMPARISON_TYPE")]
    String,
    #[serde(rename = "NUMBER_COMPARISON_TYPE")]
    Number,
    #[serde(rename = "BOOLEAN_COMPARISON_TYPE")]
    Boolean,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub enum Operator {
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
