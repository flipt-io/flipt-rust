// FLIPT_ENDPOINT=http://localhost:8080 cargo run --example list_flags

use anyhow::Result;
use flipt::api::flag::{Flag, FlagClient, FlagListRequest};
use flipt::api::ApiClient;
use flipt::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new_from_env()?;
    let client = ApiClient::new(config)?;

    for f in all_flags(&client.flags()).await? {
        println!("{:?}", f);
    }
    Ok(())
}

async fn all_flags(client: &FlagClient<'_>) -> Result<Vec<Flag>> {
    let mut all = Vec::new();
    let mut req = FlagListRequest::default();
    loop {
        let mut res = client.list(&req).await?;
        all.append(&mut res.flags);
        if all.len() >= res.total_count || res.next_page_token == "" {
            return Ok(all);
        }
        req.page_token = res.next_page_token;
    }
}
