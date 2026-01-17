# ZeroCloud Rust SDK

Official SDK for interacting with ZeroCloud private cloud services.

## Services Supported

- **ZeroStore** (S3 Parity)
- **ZeroDB** (DynamoDB Parity)
- **ZeroFunc** (Lambda/AppRunner Parity)
- **ZeroQueue** (SQS Parity)
- **ZeroID** (IAM Parity)

## Usage

```rust
use zero_sdk::ZeroClient;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize client (defaults to http://localhost:8080 or ZERO_URL env)
    let client = ZeroClient::from_env();

    // 1. Create a bucket
    client.store().create_bucket("my-assets").await?;

    // 2. Insert data into a table
    client.db().put_item("users", "user123", json!({
        "username": "zero_dev",
        "email": "dev@zero.local"
    })).await?;

    // 3. Send a message to a queue
    client.queue().send_message("orders", "{\"id\": \"order-001\"}").await?;

    Ok(())
}
```
