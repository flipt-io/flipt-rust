use crate::api::distribution::Distribution;
use crate::api::{ApiClient, Result, DEFAULT_LIMIT};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct RuleClient<'client> {
    client: &'client ApiClient,
}

impl<'client> RuleClient<'client> {
    pub fn new(client: &'client ApiClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, flag_key: &str, list: &RuleListRequest) -> Result<RuleList> {
        let path = format!("/api/v1/flags/{flag_key}/rules", flag_key = flag_key);
        self.client.get(&path, Some(list)).await
    }

    pub async fn create(&self, flag_key: &str, create: &RuleCreateRequest) -> Result<Rule> {
        let path = format!("/api/v1/flags/{flag_key}/rules", flag_key = flag_key);
        self.client.post(&path, Some(create)).await
    }

    pub async fn delete(&self, flag_key: &str, id: &str) -> Result<RuleDeletion> {
        let path = format!(
            "/api/v1/flags/{flag_key}/rules/{id}",
            flag_key = flag_key,
            id = id
        );
        self.client.delete(&path, None::<&()>).await
    }

    pub async fn get(&self, flag_key: &str, id: &str) -> Result<Rule> {
        let path = format!(
            "/api/v1/flags/{flag_key}/rules/{id}",
            flag_key = flag_key,
            id = id
        );
        self.client.get(&path, None::<&()>).await
    }

    pub async fn update(
        &self,
        flag_key: &str,
        id: &str,
        update: &RuleUpdateRequest,
    ) -> Result<Rule> {
        let path = format!(
            "/api/v1/flags/{flag_key}/rules/{id}",
            flag_key = flag_key,
            id = id,
        );
        self.client.put(&path, Some(update)).await
    }
}

#[derive(Debug, Default, Serialize)]
pub struct RuleCreateRequest {
    pub rank: usize,
    pub segment_key: String,
}

#[derive(Debug, Serialize)]
pub struct RuleUpdateRequest {
    pub rank: u32,
    pub segment_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RuleDeletion {}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleListRequest {
    pub offset: usize,
    pub limit: usize,
    pub page_token: String,
}

impl Default for RuleListRequest {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: DEFAULT_LIMIT,
            page_token: "".to_owned(),
        }
    }
}

impl RuleListRequest {
    pub fn new() -> Self {
        Self {
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
    pub segment_key: String,
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
