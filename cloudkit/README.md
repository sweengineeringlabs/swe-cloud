# CloudKit - Multi-Cloud SDK for Rust

**Unified Cloud SDK** - Interact with AWS, Azure, and GCP through a single, type-safe Rust API using the Stratified Encapsulation Architecture (SEA).

## Features

- ✅ **Unified Interface**: Same code for S3, Blob Storage, GCS, and ZeroStore.
- ✅ **Type-Safe**: Compile-time safety for all cloud operations.
- ✅ **Async-First**: Built on the Tokio runtime for high performance.
- ✅ **SEA Architecture**: Clean, layered design for high maintainability.
- ✅ **ZeroCloud Support**: Native support for high-performance functional private cloud.

## Quick Start

```rust
use cloudkit::prelude::*;

#[tokio::main]
async fn main() -> Result<(), CloudError> {
    // Initialize for ZeroCloud
    let cloud = CloudKit::zero()
        .endpoint("http://localhost:8080")
        .build()
        .await?;

    // Use unified API (works for any provider)
    cloud.storage()
        .put_object("my-bucket", "hello.txt", b"Hello from Zero!")
        .await?;

    Ok(())
}
```

## Examples

Check the [examples](./crates/cloudkit_facade/examples) directory for complete, runnable code:

*   **Platform**
    *   [Getting Started](./crates/cloudkit_facade/examples/01_aws_s3_getting_started.rs)
    *   [Local Development (CloudEmu)](./crates/cloudkit_facade/examples/04_local_development.rs)
    *   [Multi-Cloud Storage](./crates/cloudkit_facade/examples/03_multi_cloud_storage.rs)
    *   [Error Handling](./crates/cloudkit_facade/examples/02_error_handling.rs)
    *   [ZeroCloud Integration](./crates/cloudkit_facade/tests/zero_integration.rs)
*   **Services**
    *   [DynamoDB (Database)](./crates/cloudkit_facade/examples/07_database_dynamodb.rs)
    *   [SQS (Messaging)](./crates/cloudkit_facade/examples/08_messaging_sqs.rs)
    *   [SNS (PubSub)](./crates/cloudkit_facade/examples/09_pubsub_sns.rs)
    *   [Lambda (Compute)](./crates/cloudkit_facade/examples/10_serverless_lambda.rs)

## Documentation

Full documentation is available in the **[Documentation Hub](./docs/overview.md)**.

- **[Architecture Specification](./docs/3-design/architecture.md)**
- **[Developer Guide](./docs/4-development/developer-guide.md)**
- **[WASM Support](./docs/wasm.md)**

## Crates

CloudKit is composed of several specialized crates:

- `cloudkit_facade`: Public API surface.
- `cloudkit_core`: Orchestration and provider logic.
- `cloudkit_api`: Service contracts (traits).
- `cloudkit_spi`: Foundational types and errors.

## License

MIT - See [LICENSE](LICENSE) for details.
