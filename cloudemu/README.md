# CloudEmu

**A Universal, Multi-Cloud Local Emulator**

CloudEmu is a lightweight, unified emulator for AWS, Azure, and Google Cloud Platform (GCP). It enables developers to test multi-cloud applications locally without needing valid cloud credentials or incurring costs.

## 🌟 Key Features

- **Multi-Cloud Support**: Emulates core services from AWS, Azure, GCP, and Oracle.
- **Unified Architecture**: Uses a single "Storage Engine Architecture" (SEA) to back all four clouds.
- **Persistence**: All data is persisted locally via SQLite and filesystem.
- **Lightweight**: Written in Rust for high performance and low resource usage.
- **Standalone**: Runs as a single binary (or library) handling requests on different ports.

## 🚀 Supported Services

| Service Type | AWS (Port 4566) | Azure (Port 10000) | GCP (Port 4567) | Oracle (Port 4568) |
|--------------|-----------------|-------------------|-----------------|--------------------|
| **Object Storage** | S3 | Blob Storage | Cloud Storage | *Obj Storage* |
| **NoSQL Database** | DynamoDB | Cosmos DB | Firestore | - |
| **Message Queue** | SQS | Service Bus | *Pub/Sub* | - |
| **Pub/Sub** | SNS | - | Pub/Sub | - |
| **Functions** | Lambda | Azure Functions | Cloud Functions | - |
| **Secrets** | Secrets Manager | Key Vault | Secret Manager | - |
| **FinOps** | Price List | Retail Prices | Billing | Metering |

*Plus AWS extras: KMS, EventBridge, CloudWatch, Cognito, Step Functions.*

## 🛠️ Getting Started

### Prerequisites
- Rust (latest stable)
- SQLite (bundled)

### Running the Emulator

```bash
# Clone the repository
git clone https://github.com/sweengineeringlabs/cloudemu.git
cd cloudemu

# Run the server
cargo run --bin cloudemu-server
```

The server will start listening on:
- **AWS**: `http://localhost:4566`
- **Azure**: `http://localhost:10000`
- **GCP**: `http://localhost:4567`
- **Oracle**: `http://localhost:4568`

### Configuration

You can configure CloudEmu via environment variables:

```bash
export CLOUDEMU_DATA_DIR="./.cloudemu_data"
export CLOUDEMU_LOGGING="true"
```

## 📐 Architecture

CloudEmu uses a **Facade Pattern** where each cloud provider's API is translated into a common set of storage primitives.

- **SPI Layer**: Defines common interfaces for Cloud Providers.
- **Control Plane**: Parses AWS/Azure/GCP specific HTTP requests.
- **Data Plane**: A shared storage engine (SQLite + FS) that persists all resources.

See [Documentation](./doc/3-design/storage-engine-architecture.md) for details.

## 🧪 Testing

Integration tests are available for each provider:

```bash
# Run Azure tests
cargo test -p azure-control-core --test integration

# Run GCP tests
cargo test -p gcp-control-core --test integration
```

## 📜 License

MIT License.
