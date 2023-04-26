use crate::meta::{MetaClient, Result};

pub struct InfoClient<'client> {
    client: &'client MetaClient,
}

impl<'client> InfoClient<'client> {
    pub fn new(client: &'client MetaClient) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<String> {
        let result: Result<serde_json::Value> = self.client.get("/meta/info", None::<&()>).await;

        match result {
            Ok(value) => Ok(value.to_string()),
            Err(e) => Err(e),
        }
    }
}
