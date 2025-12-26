# 11 - CloudEmu Architecture

## Document Information

| Field | Value |
|-------|-------|
| **Version** | 1.0.0 |
| **Status** | Design Complete |
| **Last Updated** | 2025-12-26 |

---

## 1. Overview

**CloudEmu** is a local cloud services emulator for development and testing. It provides mock implementations of AWS services, allowing developers to test without cloud credentials or costs.

## 2. System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Your Application                             │
│                                                                  │
│   AWS SDK configured with endpoint: http://localhost:4566       │
└────────────────────────────┬────────────────────────────────────┘
                             │ HTTP Requests
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                        CloudEmu                                  │
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                   HTTP Gateway (Axum)                    │   │
│   │                                                          │   │
│   │   • Parse AWS-style requests                             │   │
│   │   • Route to service handlers                            │   │
│   │   • Generate AWS-compatible responses                    │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│         ┌──────────────────┼──────────────────┐                 │
│         ▼                  ▼                  ▼                 │
│   ┌───────────┐      ┌───────────┐      ┌───────────┐          │
│   │    S3     │      │ DynamoDB  │      │    SQS    │          │
│   │  Service  │      │  Service  │      │  Service  │          │
│   │           │      │           │      │           │          │
│   │ Buckets   │      │ Tables    │      │ Queues    │          │
│   │ Objects   │      │ Items     │      │ Messages  │          │
│   └─────┬─────┘      └─────┬─────┘      └─────┬─────┘          │
│         │                  │                  │                 │
│         └──────────────────┼──────────────────┘                 │
│                            ▼                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                  Storage Backend                         │   │
│   │                                                          │   │
│   │   In-memory (HashMap) or Persistent (File/SQLite)       │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 3. Component Design

### HTTP Gateway

```
┌─────────────────────────────────────────────────────────────────┐
│                      Request Flow                                │
│                                                                  │
│   AWS SDK Request                                                │
│        │                                                         │
│        ▼                                                         │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                     Axum Router                          │   │
│   │                                                          │   │
│   │   Route Matching:                                        │   │
│   │   GET  /              → list_buckets                    │   │
│   │   PUT  /:bucket       → create_bucket                   │   │
│   │   GET  /:bucket       → list_objects                    │   │
│   │   PUT  /:bucket/*key  → put_object                      │   │
│   │   GET  /:bucket/*key  → get_object                      │   │
│   │   DELETE /:bucket/*key→ delete_object                   │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│                            ▼                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                   Service Handler                        │   │
│   │                                                          │   │
│   │   1. Parse request (headers, body, query params)        │   │
│   │   2. Execute operation on service                       │   │
│   │   3. Generate XML/JSON response                         │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### S3 Service

```
┌─────────────────────────────────────────────────────────────────┐
│                     S3 Service Structure                         │
│                                                                  │
│   S3Service                                                      │
│   │                                                              │
│   └── buckets: RwLock<HashMap<String, BucketData>>              │
│           │                                                      │
│           └── BucketData                                         │
│                   │                                              │
│                   ├── bucket: Bucket { name, created, region }  │
│                   │                                              │
│                   └── objects: HashMap<String, S3Object>        │
│                           │                                      │
│                           └── S3Object                          │
│                                   ├── key: String               │
│                                   ├── data: Vec<u8>             │
│                                   ├── content_type: String      │
│                                   ├── etag: String              │
│                                   ├── last_modified: DateTime   │
│                                   └── metadata: HashMap         │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 4. API Compatibility

### S3 Operations Supported

| Operation | Method | Path | Status |
|-----------|--------|------|--------|
| ListBuckets | GET | / | ✅ |
| CreateBucket | PUT | /{bucket} | ✅ |
| DeleteBucket | DELETE | /{bucket} | ✅ |
| HeadBucket | HEAD | /{bucket} | ✅ |
| ListObjects | GET | /{bucket} | ✅ |
| PutObject | PUT | /{bucket}/{key} | ✅ |
| GetObject | GET | /{bucket}/{key} | ✅ |
| HeadObject | HEAD | /{bucket}/{key} | ✅ |
| DeleteObject | DELETE | /{bucket}/{key} | ✅ |
| CopyObject | PUT + x-amz-copy-source | /{bucket}/{key} | ✅ |

### Request/Response Format

AWS SDKs expect specific XML formats:

```xml
<!-- ListBucketResult -->
<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
  <Name>bucket-name</Name>
  <Prefix></Prefix>
  <MaxKeys>1000</MaxKeys>
  <IsTruncated>false</IsTruncated>
  <Contents>
    <Key>file.txt</Key>
    <LastModified>2025-12-26T10:00:00.000Z</LastModified>
    <ETag>"abc123"</ETag>
    <Size>1024</Size>
    <StorageClass>STANDARD</StorageClass>
  </Contents>
</ListBucketResult>
```

---

## 5. Configuration

```
┌─────────────────────────────────────────────────────────────────┐
│                     Configuration Options                        │
│                                                                  │
│   Environment Variable    │ Default        │ Description         │
│   ────────────────────────┼────────────────┼──────────────────   │
│   CLOUDEMU_HOST           │ 127.0.0.1      │ Bind address        │
│   CLOUDEMU_PORT           │ 4566           │ Listen port         │
│   CLOUDEMU_DATA_DIR       │ .cloudemu      │ Data directory      │
│   CLOUDEMU_PERSIST        │ false          │ Enable persistence  │
│   CLOUDEMU_REGION         │ us-east-1      │ AWS region          │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 6. Usage

### Running the Emulator

```bash
# Run with defaults
cargo run -p cloudemu

# With custom port
CLOUDEMU_PORT=5000 cargo run -p cloudemu

# With persistence
CLOUDEMU_PERSIST=true cargo run -p cloudemu
```

### Configuring AWS SDK

```rust
// Rust
let config = aws_config::from_env()
    .endpoint_url("http://localhost:4566")
    .load()
    .await;

let s3 = aws_sdk_s3::Client::new(&config);
```

```bash
# AWS CLI
aws --endpoint-url=http://localhost:4566 s3 ls
```

---

## 7. Project Structure

```
crates/cloudemu/
├── Cargo.toml
├── src/
│   ├── main.rs              # Binary entry point
│   ├── lib.rs               # Library entry point
│   ├── config.rs            # Configuration
│   ├── error.rs             # Error types
│   ├── router.rs            # HTTP router
│   │
│   └── services/
│       ├── mod.rs           # Services module
│       │
│       ├── s3/
│       │   ├── mod.rs       # S3 module
│       │   ├── service.rs   # S3 logic
│       │   ├── handlers.rs  # HTTP handlers
│       │   ├── types.rs     # Data types
│       │   └── xml.rs       # XML generation
│       │
│       ├── dynamodb/
│       │   └── mod.rs       # DynamoDB (stub)
│       │
│       └── sqs/
│           └── mod.rs       # SQS (stub)
```

---

## 8. Future Enhancements

| Feature | Priority | Status |
|---------|----------|--------|
| S3 multipart upload | High | ⬜ |
| S3 presigned URLs | High | ⬜ |
| DynamoDB operations | High | ⬜ |
| SQS operations | High | ⬜ |
| SNS operations | Medium | ⬜ |
| Lambda invocation | Medium | ⬜ |
| Persistence layer | Medium | ⬜ |
| Docker image | Low | ⬜ |
| Web UI dashboard | Low | ⬜ |

---

## 9. Related Documents

- [01-overview.md](01-overview.md) - CloudKit overview
- [04-provider-integration.md](04-provider-integration.md) - SDK details
- [09-testing-strategy.md](09-testing-strategy.md) - Using emulator for tests
