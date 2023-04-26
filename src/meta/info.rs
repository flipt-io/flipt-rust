use crate::meta::{MetaClient, Result};

pub struct InfoClient<'client> {
    client: &'client MetaClient,
}

impl<'client> InfoClient<'client> {
    pub fn new(client: &'client MetaClient) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<String> {
        self.client.get("/meta/info", None::<&()>).await
    }
}
