use crate::api::{ApiClient, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct EvaluationClient<'client> {
    client: &'client ApiClient,
}

impl<'client> EvaluationClient<'client> {
    pub fn new(client: &'client ApiClient) -> Self {
        Self { client }
    }

    pub async fn boolean(&self, eval: &EvaluateRequest) -> Result<BooleanEvaluation> {
        let path = "/evaluate/v1/boolean".to_string();

        self.client.post(&path, Some(eval)).await
    }

    pub async fn variant(&self, eval: &EvaluateRequest) -> Result<VariantEvaluation> {
        let path = "/evaluate/v1/variant".to_string();

        self.client.post(&path, Some(eval)).await
    }

    pub async fn batch(&self, batch: &BatchEvaluateRequest) -> Result<BatchEvaluation> {
        let path = "/evaluate/v1/batch".to_string();

        self.client.post(&path, Some(batch)).await
    }
}

#[derive(Debug, Default, Serialize)]
pub struct BatchEvaluateRequest {
    pub requests: Vec<EvaluateRequest>,
}

#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateRequest {
    pub context: HashMap<String, String>,
    pub entity_id: String,
    pub namespace_key: String,
    pub flag_key: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BooleanEvaluation {
    pub enabled: bool,
    pub reason: Reason,
    pub request_id: String,
    pub request_duration_millis: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariantEvaluation {
    #[serde(rename = "match")]
    pub is_match: bool,
    pub segment_keys: Vec<String>,
    pub reason: Reason,
    pub variant_key: String,
    pub variant_attachment: String,
    pub request_id: String,
    pub request_duration_millis: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEvaluation {
    pub flag_key: String,
    pub namespace_key: String,
    pub reason: ErrorEvaluationReason,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchEvaluation {
    pub request_id: String,
    pub responses: Vec<Response>,
    pub request_duration_millis: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub r#type: ResponseType,
    pub boolean_response: Option<BooleanEvaluation>,
    pub variant_response: Option<VariantEvaluation>,
    pub error_response: Option<ErrorEvaluation>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ResponseType {
    #[serde(rename = "VARIANT_EVALUATION_RESPONSE_TYPE")]
    Variant,
    #[serde(rename = "BOOLEAN_EVALUATION_RESPONSE_TYPE")]
    Boolean,
    #[serde(rename = "ERROR_EVALUATION_RESPONSE_TYPE")]
    Error,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub enum ErrorEvaluationReason {
    #[serde(rename = "UNKNOWN_ERROR_EVALUATION_REASON")]
    Unknown,
    #[serde(rename = "NOT_FOUND_ERROR_EVALUATION_REASON")]
    NotFound,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub enum Reason {
    #[serde(rename = "UNKNOWN_EVALUATION_REASON")]
    Unknown,
    #[serde(rename = "FLAG_DISABLED_EVALUATION_REASON")]
    FlagDisabled,
    #[serde(rename = "MATCH_EVALUATION_REASON")]
    Match,
    #[serde(rename = "DEFAULT_EVALUATION_REASON")]
    Default,
}
