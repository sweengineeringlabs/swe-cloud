# CloudEmu User Manual (Operations Guide)

**Audience**: DevOps Engineers, Developers, and QA.

This manual covers the **operations** of CloudEmu: running, configuring, and maintaining the emulator in local and CI environments.

## 1. Quick Start

The fastest way to start CloudEmu is using Cargo from the source repository:

```bash
# From the root of the repository
cargo run -p cloudemu-server
```

This will launch the **Unified Multi-Cloud Server** which orchestrates emulators for AWS, Azure, and GCP.

**Default Ports**:
- **AWS**: `http://localhost:4566`
- **Azure**: `http://localhost:4567`
- **GCP**: `http://localhost:4568`

## 2. Configuration

CloudEmu is configured primarily through environment variables.

| Variable | Default | Description |
| :--- | :--- | :--- |
| `CLOUDEMU_AWS_PORT` | `4566` | Port for the AWS emulator |
| `CLOUDEMU_AZURE_PORT` | `4567` | Port for the Azure emulator |
| `CLOUDEMU_GCP_PORT` | `4568` | Port for the GCP emulator |
| `CLOUDEMU_DATA_DIR` | `.cloudemu` | Directory for persistent storage |
| `RUST_LOG` | `info` | Log level (error, warn, info, debug, trace) |
| `CLOUDEMU_HOST` | `127.0.0.1` | Bind address (use `0.0.0.0` for Docker) |

### Example: Running with Custom Configuration

```bash
export CLOUDEMU_AWS_PORT=5000
export CLOUDEMU_DATA_DIR=/tmp/cloudemu-data
export RUST_LOG=debug

cargo run -p cloudemu-server
```

## 3. Connecting Clients

### AWS CLI

Configure the AWS CLI to point to localhost:

```bash
# Method 1: Per-command
aws --endpoint-url=http://localhost:4566 s3 ls

# Method 2: Environment variables
export AWS_ENDPOINT_URL=http://localhost:4566
aws s3 mb s3://test-bucket
```

### Azure CLI (Blob Storage)

Use the storage emulator connection string or explicit endpoints:

```bash
# List containers (using curl for direct API access as Azure CLI requires special setup for local emulators)
curl "http://localhost:4567/devstoreaccount1/?comp=list"
```

### Terraform (IAC)

See the **[IAC Integration Guide](../../../iac/doc/7-operations/user-manual.md)** for detailed Terraform configuration. Setup endpoints in your provider configuration:

```hcl
provider "aws" {
  endpoints {
    s3 = "http://localhost:4566"
    # ... other services
  }
}
```

### CloudKit (Rust SDK)

CloudKit includes built-in support for CloudEmu through the `.cloudemu()` builder method.

```rust
use cloudkit::CloudKit;
use cloudkit_spi::ProviderType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Facade: Automatically configures local endpoints (e.g., http://localhost:4566 for AWS)
    let client = CloudKit::cloudemu(ProviderType::Aws)
        .build()
        .await?;

    // 2. Direct Provider: Manually set the endpoint
    let aws = cloudkit_aws::AwsBuilder::new()
        .endpoint("http://localhost:4566")
        .build()
        .await?;

    Ok(())
}
```

## 4. Data Persistence & Reset

CloudEmu persists resource metadata and data to the `CLOUDEMU_DATA_DIR` (default: `.cloudemu`).

### Where is my data?
- **AWS Data**: `.cloudemu/aws/`
- **Azure Data**: `.cloudemu/azure/`
- **GCP Data**: `.cloudemu/gcp/`

### How to Reset State?

To wipe all data and start fresh (factory reset):

1. Stop the server (`Ctrl+C`).
2. Delete the data directory:
   ```bash
   rm -rf .cloudemu
   ```
3. Restart the server.

## 5. Troubleshooting / FAQ

### "Connection Refused"
- **Cause**: The server is not running or running on a different port.
- **Fix**: Check terminal output for listening ports. Verfy `localhost` vs `127.0.0.1`.

### "Address already in use"
- **Cause**: Another process (or a zombie CloudEmu instance) is using port 4566/4567/4568.
- **Fix**:
  ```bash
  # Find process (Linux/Mac)
  lsof -i :4566
  
  # Kill process
  kill -9 <PID>
  ```

### "Access Denied" (AWS)
- **Cause**: While CloudEmu allows any credentials, some SDKs/CLIs require *valid-looking* credentials.
- **Fix**: Ensure `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` are set to any non-empty string (e.g., "test").

### Logs & Debugging
Enable detailed logs to debug request handling:

```bash
RUST_LOG=cloudemu=debug,tower_http=debug cargo run -p cloudemu-server
```

## 6. Maintenance

### Upgrading
Pull the latest changes from the repository and recompile:

```bash
git pull
cargo build --release -p cloudemu-server
```

### Health Checks
The server exposes a health endpoint (per provider):

```bash
curl http://localhost:4566/health
```

---

**Related Documentation**:
- [Installation Guide](../6-deployment/installation.md)
- [Architecture Overview](../3-design/architecture.md)
