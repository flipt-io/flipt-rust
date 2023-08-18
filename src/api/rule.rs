use crate::api::distribution::Distribution;
use crate::api::{ApiClient, Result, DEFAULT_LIMIT, DEFAULT_NAMESPACE};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct RuleClient<'client> {
    client: &'client ApiClient,
}

impl<'client> RuleClient<'client> {
    pub fn new(client: &'client ApiClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, list: &RuleListRequest) -> Result<RuleList> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags/{flag_key}/rules",
            namespace_key = list
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            flag_key = list.flag_key
        );
        self.client.get(&path, Some(list)).await
    }

    pub async fn create(&self, create: &RuleCreateRequest) -> Result<Rule> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags/{flag_key}/rules",
            namespace_key = create
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            flag_key = create.flag_key
        );
        self.client.post(&path, Some(create)).await
    }

    pub async fn delete(&self, delete: &RuleDeleteRequest) -> Result<RuleDeletion> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags/{flag_key}/rules/{id}",
            namespace_key = delete
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            flag_key = delete.flag_key,
            id = delete.id
        );
        self.client.delete(&path, None::<&()>).await
    }

    pub async fn get(&self, get: &RuleGetRequest) -> Result<Rule> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags/{flag_key}/rules/{id}",
            namespace_key = get
                .namespace_key
                .as_ref()
                .unwrap_or(&DEFAULT_NAMESPACE.to_string()),
            flag_key = get.flag_key,
            id = get.id
        );
        self.client.get(&path, None::<&()>).await
    }

    pub async fn update(&self, update: &RuleUpdateRequest) -> Result<Rule> {
        let path = format!(
            "/api/v1/namespaces/{namespace_key}/flags/{flag_key}/rules/{id}",
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

#[derive(Debug, Default)]
pub struct RuleGetRequest {
    pub namespace_key: Option<String>,
    pub flag_key: String,
    pub id: String,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleCreateRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    #[serde(skip_serializing)]
    pub flag_key: String,
    pub segment_key: Option<String>,
    pub segment_keys: Option<Vec<String>>,
    pub segment_operator: Option<SegmentOperator>,
    pub rank: usize,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub enum SegmentOperator {
    #[default]
    #[serde(rename = "OR_SEGMENT_OPERATOR")]
    Or,
    #[serde(rename = "AND_SEGMENT_OPERATOR")]
    And,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleUpdateRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    #[serde(skip_serializing)]
    pub flag_key: String,
    #[serde(skip_serializing)]
    pub id: String,
    pub segment_key: Option<String>,
    pub segment_keys: Option<Vec<String>>,
    pub segment_operator: Option<SegmentOperator>,
    pub rank: u32,
}

#[derive(Debug, Default)]
pub struct RuleDeleteRequest {
    pub namespace_key: Option<String>,
    pub flag_key: String,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RuleDeletion {}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleListRequest {
    #[serde(skip_serializing)]
    pub namespace_key: Option<String>,
    pub flag_key: String,
    pub offset: usize,
    pub limit: usize,
    pub page_token: String,
}

impl Default for RuleListRequest {
    fn default() -> Self {
        Self {
            namespace_key: None,
            flag_key: "".to_owned(),
            offset: 0,
            limit: DEFAULT_LIMIT,
            page_token: "".to_owned(),
        }
    }
}

impl RuleListRequest {
    pub fn new() -> Self {
        Self {
            namespace_key: None,
            flag_key: "".to_owned(),
            offset: 0,
            limit: DEFAULT_LIMIT,
            page_token: "".to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub id: String,
    pub rank: u32,
    pub distributions: Vec<Distribution>,
    pub segment_key: Option<String>,
    pub segment_keys: Option<Vec<String>>,
    pub segment_operator: Option<SegmentOperator>,
    pub flag_key: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleList {
    pub rules: Vec<Rule>,
    pub next_page_token: String,
    pub total_count: u32,
}
