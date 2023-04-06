use crate::api::{ApiClient, Result, DEFAULT_NAMESPACE};
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

    pub async fn evaluate(&self, eval: &EvaluateRequest) -> Result<Evaluation> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/evaluate",
            namespace_key = eval
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string())
        );
        self.client.post(&path, Some(eval)).await
    }

    pub async fn evaluate_batch(&self, batch: &BatchEvaluateRequest) -> Result<Evaluation> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/batch-evaluate",
            namespace_key = batch
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string())
        );
        self.client.post(&path, Some(batch)).await
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchEvaluateRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    pub requests: Vec<EvaluateRequest>,
    pub exclude_not_found: bool,
    pub request_id: String,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateRequest {
    pub context: HashMap<String, String>,
    pub entity_id: String,
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    pub flag_key: String,
    pub request_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchEvaluation {
    pub response: Vec<Evaluation>,
    pub request_duration_millis: f64,
    pub request_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Evaluation {
    pub attachment: String,
    pub entity_id: String,
    pub namespace_key: String,
    pub flag_key: String,
    #[serde(rename = "match")]
    pub is_match: bool,
    pub reason: Reason,
    pub request_context: HashMap<String, String>,
    pub request_duration_millis: f64,
    pub request_id: String,
    pub segment_key: String,
    pub timestamp: DateTime<Utc>,
    pub value: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub enum Reason {
    #[serde(rename = "UNKNOWN_EVALUATION_REASON")]
    Unknown,
    #[serde(rename = "FLAG_DISABLED_EVALUATION_REASON")]
    FlagDisabled,
    #[serde(rename = "FLAG_NOT_FOUND_EVALUATION_REASON")]
    FlagNotFound,
    #[serde(rename = "MATCH_EVALUATION_REASON")]
    Match,
    #[serde(rename = "ERROR_EVALUATION_REASON")]
    Error,
}
