# Storage Engine Architecture: Multi-Cloud Facade Pattern

## Overview

CloudEmu uses a **Facade Pattern** architecture where multiple cloud providers (AWS, Azure, GCP) present different APIs but share the same underlying storage implementation. This document explains the "X-backed" terminology and the architectural rationale.

**Key Principle**: **Different Contracts, Same Implementation**

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                    USER-FACING CONTRACTS                            │
│                      (Different APIs)                               │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌──────────────┐    ┌───────────────┐    ┌──────────────────┐    │
│  │   AWS S3     │    │  Azure Blob   │    │  GCP Storage     │    │
│  ├──────────────┤    ├───────────────┤    ├──────────────────┤    │
│  │ PUT /bucket  │    │ PUT /container│    │ PUT /bucket      │    │
│  │ GET /bucket  │    │ GET /container│    │ GET /bucket      │    │
│  │ x-amz-*      │    │ x-ms-*        │    │ x-goog-*         │    │
│  │ Port 4566    │    │ Port 4567     │    │ Port 4568        │    │
│  └──────┬───────┘    └───────┬───────┘    └────────┬─────────┘    │
│         │                    │                     │               │
└─────────┼────────────────────┼─────────────────────┼───────────────┘
          │                    │                     │
          ▼                    ▼                     ▼
┌─────────────────────────────────────────────────────────────────────┐
│              SERVICE TRANSLATION LAYER                              │
│               (Provider-Specific Logic)                             │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌──────────────┐    ┌───────────────┐    ┌──────────────────┐    │
│  │  S3Service   │    │  BlobService  │    │ StorageService   │    │
│  ├──────────────┤    ├───────────────┤    ├──────────────────┤    │
│  │ • Parse AWS  │    │ • Parse Azure │    │ • Parse GCP      │    │
│  │ • Validate   │    │ • Validate    │    │ • Validate       │    │
│  │ • Translate  │    │ • Translate   │    │ • Translate      │    │
│  │   to storage │    │   to storage  │    │   to storage     │    │
│  └──────┬───────┘    └───────┬───────┘    └────────┬─────────┘    │
│         │                    │                     │               │
└─────────┼────────────────────┼─────────────────────┼───────────────┘
          │                    │                     │
          └────────────────────┼─────────────────────┘
                               │
                               ▼
              ┌────────────────────────────────┐
              │   SHARED STORAGE ENGINE        │
              │   (Same Implementation)        │
              ├────────────────────────────────┤
              │ • put_object(bucket, key, ...) │
              │ • get_object(bucket, key, ...) │
              │ • create_bucket(name, ...)     │
              │ • put_item(table, key, ...)    │
              │ • send_message(queue, ...)     │
              │ • create_function(name, ...)   │
              │ • put_secret_value(name, ...)  │
              └────────────────┬───────────────┘
                               │
                               ▼
                  ┌─────────────────────────┐
                  │   PHYSICAL STORAGE      │
                  ├─────────────────────────┤
                  │ • SQLite (metadata)     │
                  │ • Filesystem (objects)  │
                  └─────────────────────────┘
```

---

## What Does "X-Backed" Mean?

When we say a service is "**S3-backed**", "**DynamoDB-backed**", or "**Lambda-backed**", we mean:

> **The service uses storage engine methods originally written for that AWS service, even though the API presented to users is different.**

### Example Mappings

| Service Category | AWS (Native) | Azure | GCP |
|-----------------|--------------|-------|-----|
| **Object Storage** | S3 | Blob Storage (S3-backed) | Cloud Storage (S3-backed) |
| **NoSQL Database** | DynamoDB | Cosmos DB (DynamoDB-backed) | Firestore (DynamoDB-backed) |
| **Message Queue** | SQS | Service Bus (SQS-backed) | - |
| **Pub/Sub** | SNS | - | Pub/Sub (SNS-backed) |
| **Functions** | Lambda | Azure Functions (Lambda-backed) | Cloud Functions (Lambda-backed) |
| **Secrets** | Secrets Manager | Key Vault (Secrets Manager-backed) | Secret Manager (Secrets Manager-backed) |

---

## Layer-by-Layer Breakdown

### Layer 1: API Contract (Provider-Specific)

Each provider has a **unique** API contract that matches the real cloud provider:

#### AWS S3
```http
PUT /my-bucket/file.txt HTTP/1.1
Host: localhost:4566
x-amz-content-sha256: UNSIGNED-PAYLOAD
Content-Type: text/plain

Hello World
```

#### Azure Blob Storage
```http
PUT /devstoreaccount1/my-container/blob.txt HTTP/1.1
Host: localhost:4567
x-ms-blob-type: BlockBlob
x-ms-version: 2021-08-06

Hello World
```

#### GCP Cloud Storage
```http
PUT /my-bucket/object.txt HTTP/1.1
Host: localhost:4568
x-goog-api-version: 2
Content-Type: application/octet-stream

Hello World
```

### Layer 2: Service Layer (Translation)

Each service translates its specific API into common storage operations:

#### AWS S3Service
```rust
impl S3Service {
    pub async fn handle_request(&self, req: Request) -> Response {
        // Parse: /{bucket}/{key}
        let (bucket, key) = parse_aws_path(&req.path);
        
        // Call shared storage
        self.engine.put_object(bucket, key, &req.body, ...)?;
    }
}
```

#### Azure BlobService
```rust
impl BlobService {
    pub async fn handle_request(&self, req: Request) -> Response {
        // Parse: /{account}/{container}/{blob}
        let (_account, container, blob) = parse_azure_path(&req.path);
        
        // Translate Azure terms → Storage terms
        // container → bucket, blob → key
        self.engine.put_object(container, blob, &req.body, ...)?;
    }
}
```

#### GCP CloudStorageService
```rust
impl CloudStorageService {
    pub async fn handle_request(&self, req: Request) -> Response {
        // Parse: /{bucket}/{object}
        let (bucket, object) = parse_gcp_path(&req.path);
        
        // Call shared storage (GCP already uses similar terms)
        self.engine.put_object(bucket, object, &req.body, ...)?;
    }
}
```

### Layer 3: Storage Engine (Shared Implementation)

**One implementation** used by all providers:

```rust
impl StorageEngine {
    /// Store an object (used by all providers)
    pub fn put_object(
        &self,
        bucket: &str,      // AWS: bucket, Azure: container, GCP: bucket
        key: &str,         // AWS: key, Azure: blob, GCP: object
        data: &[u8],
        content_type: Option<&str>,
        version_id: Option<&str>
    ) -> Result<ObjectMetadata> {
        // 1. Calculate ETag
        let etag = format!("{:x}", md5::compute(data));
        let size = data.len() as u64;
        
        // 2. Store metadata in SQLite
        let db = self.db.lock();
        db.execute(
            "INSERT INTO objects (bucket, key, etag, size, content_type) 
             VALUES (?, ?, ?, ?, ?)",
            params![bucket, key, etag, size, content_type]
        )?;
        
        // 3. Write data to filesystem
        let object_path = self.objects_dir.join(bucket).join(key);
        std::fs::create_dir_all(object_path.parent().unwrap())?;
        std::fs::write(&object_path, data)?;
        
        Ok(ObjectMetadata { bucket, key, etag, size, ... })
    }
}
```

### Layer 4: Physical Storage (SQLite + Filesystem)

All data is stored identically:

```
cloudemu-data/
├── metadata.db                      ← SQLite database
│   ├── objects                      ← Object metadata
│   ├── ddb_tables                   ← Table metadata
│   ├── sqs_queues                   ← Queue metadata
│   ├── lambda_functions             ← Function metadata
│   └── secrets                      ← Secret metadata
│
└── objects/                         ← Filesystem storage
    ├── aws-bucket/
    │   └── file.txt                 ← Stored via S3
    ├── azure-container/
    │   └── blob.txt                 ← Stored via Blob (same mechanism)
    └── gcp-bucket/
        └── object.txt               ← Stored via GCS (same mechanism)
```

---

## Complete Service Mapping

### Object Storage (S3 Engine)

| Provider | Service | API Endpoint | Storage Call |
|----------|---------|-------------|--------------|
| AWS | S3 | `PUT /{bucket}/{key}` | `put_object(bucket, key, ...)` |
| Azure | Blob Storage | `PUT /{account}/{container}/{blob}` | `put_object(container, blob, ...)` |
| GCP | Cloud Storage | `PUT /{bucket}/{object}` | `put_object(bucket, object, ...)` |

**Result**: All three store to `./objects/{bucket}/{key}` and the same SQLite `objects` table.

### NoSQL Database (DynamoDB Engine)

| Provider | Service | API Endpoint | Storage Call |
|----------|---------|-------------|--------------|
| AWS | DynamoDB | `POST / (X-Amz-Target: DynamoDB_20120810.PutItem)` | `put_item(table, pk, sk, ...)` |
| Azure | Cosmos DB | `POST /dbs/{db}/colls/{coll}/docs` | `put_item("{db}_{coll}", id, ...)` |
| GCP | Firestore | `POST /projects/{p}/databases/{d}/documents/{c}` | `put_item("firestore_{c}", id, ...)` |

**Result**: All three store to SQLite `ddb_items` table.

### Message Queue (SQS Engine)

| Provider | Service | API Endpoint | Storage Call |
|----------|---------|-------------|--------------|
| AWS | SQS | `POST /?Action=SendMessage` | `send_message(queue, body)` |
| Azure | Service Bus | `POST /{queue}/messages` | `send_message(queue, body)` |

**Result**: Both store to SQLite `sqs_messages` table.

### Serverless Functions (Lambda Engine)

| Provider | Service | API Endpoint | Storage Call |
|----------|---------|-------------|--------------|
| AWS | Lambda | `POST /2015-03-31/functions` | `create_function(params)` |
| Azure | Azure Functions | `POST /admin/functions/{name}` | `create_function(params)` |
| GCP | Cloud Functions | `POST /v1/projects/{p}/locations/{l}/functions` | `create_function(params)` |

**Result**: All three store to SQLite `lambda_functions` table.

### Secrets Management (Secrets Manager Engine)

| Provider | Service | API Endpoint | Storage Call |
|----------|---------|-------------|--------------|
| AWS | Secrets Manager | `POST / (X-Amz-Target: CreateSecret)` | `create_secret(...), put_secret_value(...)` |
| Azure | Key Vault | `PUT /secrets/{name}` | `create_secret(...), put_secret_value(...)` |
| GCP | Secret Manager | `POST /v1/projects/{p}/secrets` | `create_secret(...), put_secret_value(...)` |

**Result**: All three store to SQLite `secrets` and `secret_versions` tables.

---

## Benefits of This Architecture

### 1. Code Reuse ♻️
- Write storage logic **once** (for AWS)
- Reuse it **three times** (AWS, Azure, GCP)
- Reduced maintenance burden

### 2. Consistency 🎯
- All providers behave identically under the hood
- Predictable behavior across clouds
- Easier debugging

### 3. Multi-Cloud Compatibility 🌐
- Data created via AWS API can be accessed via Azure/GCP APIs
- Example: Upload via S3, download via Blob Storage (same bucket)
- True multi-cloud data portability

### 4. Simplified Testing ✅
- Test storage engine once
- Provider-specific tests only for API translation
- Reduced test surface area

### 5. Single Source of Truth 📊
- One SQLite schema
- One filesystem structure
- One set of storage semantics

---

## Trade-offs

### Advantages ✅
- Dramatically reduced code duplication
- Consistent storage behavior
- Easy to add new providers
- Multi-cloud data compatibility

### Limitations ⚠️
- Provider-specific features may not map perfectly
- Some cloud-native optimizations not available
- Storage schema influenced by AWS (original implementation)

### Acceptable for Emulator ✔️
This architecture is **ideal for an emulator** because:
- Emulation goal is testing, not production
- Developer experience benefits from consistency
- Multi-cloud portability is a feature, not a bug
- Simplified implementation enables rapid iteration

---

## Implementation Status

### ✅ Complete (21 Services)

#### AWS (11 Services - Native)
- S3, DynamoDB, SQS, SNS, Lambda
- Secrets Manager, KMS, EventBridge
- CloudWatch, Cognito, Step Functions

#### Azure (5 Services - Facade)
- Blob Storage → S3 engine
- Cosmos DB → DynamoDB engine
- Service Bus → SQS engine
- Azure Functions → Lambda engine
- Key Vault → Secrets Manager engine

#### GCP (5 Services - Facade)
- Cloud Storage → S3 engine
- Firestore → DynamoDB engine
- Pub/Sub → SNS engine
- Cloud Functions → Lambda engine
- Secret Manager → Secrets Manager engine

---

## Extension Points

To add a **new cloud provider** (e.g., Oracle Cloud):

1. **Create SPI/API/Core crates** following the pattern
2. **Implement provider services** that translate APIs
3. **Reuse existing storage engine** methods
4. **Add routing logic** in provider struct
5. **No storage changes needed** ✨

Example:
```rust
// Oracle Object Storage → S3 engine
impl OracleObjectStorageService {
    async fn put_object(&self, namespace: &str, bucket: &str, object: &str, data: &[u8]) {
        // Translate Oracle terms → Storage terms
        self.engine.put_object(bucket, object, data, ...)?;
    }
}
```

---

## Related Documentation

- [Architecture Overview](./architecture.md)
- [Implementation Status](./implementation-status.md)
- [Request Flow Diagrams](./request-flow-diagrams.md)
- [Multi-Cloud Refactoring Plan](./multi-cloud-refactoring-plan.md)

---

## Glossary

- **Facade Pattern**: Structural design pattern that provides a simplified interface to a complex subsystem
- **Storage Engine**: The underlying implementation that persists data to SQLite and filesystem
- **X-backed**: A service that uses the storage engine methods from another provider's native implementation
- **Translation Layer**: Provider-specific code that converts API requests into storage operations
- **Contract**: The public API specification (endpoints, headers, request/response formats)

---

**Last Updated**: 2026-01-15  
**Status**: ✅ Complete - All 3 providers implemented
