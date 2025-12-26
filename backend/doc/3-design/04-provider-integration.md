# 04 - Provider Integration

## Document Information

| Field | Value |
|-------|-------|
| **Version** | 1.0.0 |
| **Status** | Design Complete |
| **Last Updated** | 2025-12-26 |

---

## 1. SDK vs CLI Architecture

CloudKit uses **native SDKs** (Software Development Kits) for cloud integration, not command-line tools.

### Comparison

```
┌─────────────────────────────────────────────────────────────────┐
│                      SDK Approach (Used)                         │
│                                                                  │
│   ┌─────────────────┐                                           │
│   │  Application    │                                           │
│   └────────┬────────┘                                           │
│            │ Function call                                       │
│            ▼                                                     │
│   ┌─────────────────┐                                           │
│   │  CloudKit       │                                           │
│   └────────┬────────┘                                           │
│            │ Method call                                         │
│            ▼                                                     │
│   ┌─────────────────┐                                           │
│   │  Provider SDK   │  (aws-sdk-s3, azure_storage, etc.)        │
│   └────────┬────────┘                                           │
│            │ HTTPS                                               │
│            ▼                                                     │
│   ┌─────────────────┐                                           │
│   │  Cloud API      │                                           │
│   └─────────────────┘                                           │
│                                                                  │
│   ✅ Fast (direct API calls)                                    │
│   ✅ Type-safe                                                   │
│   ✅ Async-native                                                │
│   ✅ No external dependencies                                    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                    CLI Approach (NOT Used)                       │
│                                                                  │
│   ┌─────────────────┐                                           │
│   │  Application    │                                           │
│   └────────┬────────┘                                           │
│            │ std::process::Command                               │
│            ▼                                                     │
│   ┌─────────────────┐                                           │
│   │  Shell Process  │  (aws, az, gcloud, oci)                   │
│   └────────┬────────┘                                           │
│            │ stdout/stderr                                       │
│            ▼                                                     │
│   ┌─────────────────┐                                           │
│   │  Parse Output   │                                           │
│   └────────┬────────┘                                           │
│            │                                                     │
│            ▼                                                     │
│   ┌─────────────────┐                                           │
│   │  Cloud API      │                                           │
│   └─────────────────┘                                           │
│                                                                  │
│   ❌ Slow (process spawn overhead)                              │
│   ❌ Text parsing for errors                                    │
│   ❌ Blocking I/O                                                │
│   ❌ Requires CLI installed                                      │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Performance Comparison

| Aspect | SDK | CLI |
|--------|-----|-----|
| **Latency** | ~5ms overhead | ~50-200ms overhead |
| **Memory** | In-process | New process per call |
| **Error Handling** | Typed errors | String parsing |
| **Async** | Native async/await | Blocking |
| **Dependencies** | Compiled in | External binary |

---

## 2. Provider SDK Details

### AWS SDK for Rust

```
┌─────────────────────────────────────────────────────────────────┐
│                      AWS SDK for Rust                            │
│                                                                  │
│   Maintainer: Amazon Web Services                                │
│   Repository: github.com/awslabs/aws-sdk-rust                   │
│   Status:     GA (Generally Available)                           │
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                    Crate Structure                       │   │
│   │                                                          │   │
│   │   aws-config      │  Configuration loading               │   │
│   │   aws-sdk-s3      │  S3 operations                       │   │
│   │   aws-sdk-dynamodb│  DynamoDB operations                 │   │
│   │   aws-sdk-sqs     │  SQS operations                      │   │
│   │   aws-sdk-sns     │  SNS operations                      │   │
│   │   aws-sdk-lambda  │  Lambda invocation                    │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
│   Authentication:                                                │
│   • Environment variables (AWS_ACCESS_KEY_ID)                   │
│   • Shared credentials file (~/.aws/credentials)                │
│   • IAM roles (EC2, ECS, Lambda)                                │
│   • STS AssumeRole                                               │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

**Cargo Dependencies**:
```toml
aws-config = "1.5"
aws-sdk-s3 = "1.65"
aws-sdk-dynamodb = "1.56"
aws-sdk-sqs = "1.51"
aws-sdk-sns = "1.53"
aws-sdk-lambda = "1.60"
```

### Azure SDK for Rust

```
┌─────────────────────────────────────────────────────────────────┐
│                     Azure SDK for Rust                           │
│                                                                  │
│   Maintainer: Microsoft                                          │
│   Repository: github.com/Azure/azure-sdk-for-rust               │
│   Status:     GA (Storage), Preview (others)                    │
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                    Crate Structure                       │   │
│   │                                                          │   │
│   │   azure_core        │  Core types and traits             │   │
│   │   azure_storage     │  Storage account client            │   │
│   │   azure_storage_blobs  │  Blob storage operations        │   │
│   │   azure_cosmos      │  Cosmos DB (coming)                │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
│   Authentication:                                                │
│   • Storage account key                                          │
│   • Connection string                                            │
│   • Azure AD (DefaultAzureCredential)                           │
│   • Managed Identity                                             │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

**Cargo Dependencies**:
```toml
azure_core = "0.21"
azure_storage = "0.21"
azure_storage_blobs = "0.21"
```

### GCP SDK (Community)

```
┌─────────────────────────────────────────────────────────────────┐
│                   Google Cloud Rust (Community)                  │
│                                                                  │
│   Maintainer: Community (not official Google)                   │
│   Repository: github.com/yoshidan/google-cloud-rust             │
│   Status:     Community maintained, actively developed          │
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                    Crate Structure                       │   │
│   │                                                          │   │
│   │   google-cloud-storage   │  Cloud Storage operations    │   │
│   │   google-cloud-pubsub    │  Pub/Sub messaging           │   │
│   │   google-cloud-spanner   │  Spanner database            │   │
│   │   google-cloud-bigquery  │  BigQuery (analytics)        │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
│   Authentication:                                                │
│   • Service account JSON file                                    │
│   • GOOGLE_APPLICATION_CREDENTIALS env var                      │
│   • Workload Identity (GKE)                                      │
│                                                                  │
│   ⚠️ Note: Not official Google SDK                              │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

**Cargo Dependencies**:
```toml
google-cloud-storage = "0.22"
google-cloud-pubsub = "0.29"
```

### Oracle Cloud (REST API)

```
┌─────────────────────────────────────────────────────────────────┐
│                   Oracle Cloud Infrastructure                    │
│                                                                  │
│   Maintainer: N/A (no official Rust SDK)                        │
│   Approach:   Direct REST API via HTTP client                   │
│   Status:     Custom implementation required                    │
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                  Implementation Approach                 │   │
│   │                                                          │   │
│   │   reqwest           │  HTTP client                       │   │
│   │   OCI Signature     │  Request signing (custom code)     │   │
│   │   serde_json        │  JSON serialization                │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
│   Authentication:                                                │
│   • API key signing (RSA)                                       │
│   • OCI config file (~/.oci/config)                             │
│   • Instance principal (on OCI compute)                         │
│                                                                  │
│   API Endpoints:                                                 │
│   • objectstorage.{region}.oraclecloud.com                      │
│   • nosql.{region}.oraclecloud.com                              │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

**Cargo Dependencies**:
```toml
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
# Plus custom signing implementation
```

---

## 3. SDK Maturity Matrix

```
┌─────────────────────────────────────────────────────────────────┐
│                    SDK Maturity Matrix                           │
│                                                                  │
│              AWS     Azure    GCP      Oracle                   │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │                                                          │  │
│   │ Official    ████████  ████████  ░░░░░░░░  ░░░░░░░░      │  │
│   │                                                          │  │
│   │ GA Status   ████████  ██████░░  ░░░░░░░░  ░░░░░░░░      │  │
│   │                                                          │  │
│   │ Async       ████████  ████████  ████████  ████████      │  │
│   │                                                          │  │
│   │ Complete    ████████  ██████░░  ████░░░░  ██░░░░░░      │  │
│   │                                                          │  │
│   └──────────────────────────────────────────────────────────┘  │
│                                                                  │
│   Legend:  ████ = Full   ██░░ = Partial   ░░░░ = Missing       │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

| Provider | Official | GA Status | Async | Completeness |
|----------|----------|-----------|-------|--------------|
| **AWS** | ✅ Yes | ✅ GA | ✅ Full | ✅ All services |
| **Azure** | ✅ Yes | ⚠️ Storage GA | ✅ Full | ⚠️ Storage, some services |
| **GCP** | ❌ Community | ⚠️ Community | ✅ Full | ⚠️ Core services |
| **Oracle** | ❌ None | ❌ N/A | ✅ REST | ⚠️ Manual impl |

---

## 4. Authentication Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    Authentication Flow                           │
│                                                                  │
│                         CloudKit                                 │
│                             │                                    │
│                             ▼                                    │
│                    ┌─────────────────┐                          │
│                    │  AuthProvider   │ (SPI)                    │
│                    │     trait       │                          │
│                    └────────┬────────┘                          │
│                             │                                    │
│         ┌───────────────────┼───────────────────┐               │
│         ▼                   ▼                   ▼                │
│   ┌───────────┐      ┌───────────┐      ┌───────────┐          │
│   │ Env Vars  │      │  Vault    │      │  Custom   │          │
│   │           │      │           │      │           │          │
│   │ AWS_*     │      │ HashiCorp │      │ Your impl │          │
│   │ AZURE_*   │      │ Vault     │      │           │          │
│   │ GOOGLE_*  │      │           │      │           │          │
│   └───────────┘      └───────────┘      └───────────┘          │
│         │                   │                   │                │
│         └───────────────────┼───────────────────┘               │
│                             ▼                                    │
│                    ┌─────────────────┐                          │
│                    │   Credentials   │                          │
│                    │                 │                          │
│                    │ • access_key    │                          │
│                    │ • secret_key    │                          │
│                    │ • session_token │                          │
│                    └─────────────────┘                          │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 5. Request Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                      Complete Request Flow                       │
│                                                                  │
│   Application                                                    │
│       │                                                          │
│       │  storage.put_object("bucket", "key", data)              │
│       ▼                                                          │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                    CloudKit                              │   │
│   │                                                          │   │
│   │   1. Validate parameters                                 │   │
│   │   2. Get credentials (AuthProvider)                     │   │
│   │   3. Create operation context                           │   │
│   │                                                          │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│                            ▼                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │               OperationExecutor                          │   │
│   │                                                          │   │
│   │   4. Start metrics timer                                 │   │
│   │   5. Execute operation                                   │   │
│   │   6. Handle retry on failure                            │   │
│   │   7. Record metrics                                      │   │
│   │                                                          │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│                            ▼                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │              Provider Implementation                     │   │
│   │                                                          │   │
│   │   8. Map CloudKit types to SDK types                    │   │
│   │   9. Call SDK method                                     │   │
│   │   10. Map SDK response to CloudKit types                │   │
│   │                                                          │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│                            ▼                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                 Provider SDK                             │   │
│   │                                                          │   │
│   │   11. Serialize request                                  │   │
│   │   12. Sign request                                       │   │
│   │   13. HTTPS to cloud API                                │   │
│   │   14. Deserialize response                               │   │
│   │                                                          │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│                            ▼                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                  Cloud Provider                          │   │
│   │                                                          │   │
│   │   15. Authenticate request                               │   │
│   │   16. Execute operation                                  │   │
│   │   17. Return response                                    │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 6. Service Mapping Table

| CloudKit Service | AWS | Azure | GCP | Oracle |
|------------------|-----|-------|-----|--------|
| **ObjectStorage** | S3 | Blob Storage | Cloud Storage | Object Storage |
| └ put_object | PutObject | Upload | objects.insert | PutObject |
| └ get_object | GetObject | Download | objects.get | GetObject |
| └ list_objects | ListObjectsV2 | List | objects.list | ListObjects |
| └ delete_object | DeleteObject | Delete | objects.delete | DeleteObject |
| **KeyValueStore** | DynamoDB | Cosmos DB | Firestore | NoSQL |
| └ get | GetItem | ReadItem | get | Get |
| └ put | PutItem | UpsertItem | set | Put |
| └ query | Query | Query | where | Query |
| **MessageQueue** | SQS | Service Bus | Cloud Tasks | Streaming |
| └ send | SendMessage | SendMessage | createTask | PublishMessage |
| └ receive | ReceiveMessage | ReceiveMessage | - | Consume |
| **PubSub** | SNS | Event Grid | Pub/Sub | - |
| └ publish | Publish | Publish | publish | - |
| **Functions** | Lambda | Functions | Cloud Functions | Functions |
| └ invoke | Invoke | - | callFunction | Invoke |

---

## 7. Related Documents

- [01-overview.md](01-overview.md) - Project overview
- [03-api-design.md](03-api-design.md) - API contracts
- [06-configuration.md](06-configuration.md) - Configuration details
