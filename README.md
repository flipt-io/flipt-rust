# Flipt Rust Library (Deprecated)

![deprecated](https://img.shields.io/badge/status-deprecated-red)

> [!WARNING]
> This SDK is deprecated and will not be maintained going forward. Please use the new [Flipt Server Rust SDK](https://github.com/flipt-io/flipt-server-sdks/tree/main/flipt-rust) instead.

## Documentation

API documentation is available at <https://www.flipt.io/docs/reference/overview>.

SDK documentation is available at <https://docs.rs/crate/flipt/latest>.

## Status

This SDK is considered in 'hardening' status. We will try to minimize breaking changes, however there still may be breaking changes between versions without a major version update. 

We recommend pinning the package version to a specific version in your Cargo.toml file. This way, you can install the same version each time without breaking changes unless you are intentionally looking for the latest version.

## Install

```toml
# Cargo.toml
[dependencies]
flipt = "0.x.0"
```
## Usage

```rust
use flipt::api::flag::{Flag, FlagClient, FlagGetRequest};
use flipt::api::ApiClient;
use flipt::Config;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::new_from_env().expect("config");
    let client = ApiClient::new(config).expect("build client");

    let flag = client.flags().get(&FlagGetRequest{
      namespace_key: None,
      key: "flag-a",
    }).await
}
```

## Contributing

TODO
