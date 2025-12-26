# 14 - Production-Grade S3 Emulator Design

## Document Information

| Field | Value |
|-------|-------|
| **Version** | 1.0.0 |
| **Status** | Design Phase |
| **Last Updated** | 2025-12-26 |

---

## 1. Vision

Build an S3 emulator that:
- **Behaves exactly like production AWS S3**
- **Works with Terraform** for infrastructure provisioning
- **Supports the full lifecycle**: provision → deploy → operate → destroy
- **Can be swapped with real S3** with zero code changes

---

## 2. What "Production-Like" Means

```
┌─────────────────────────────────────────────────────────────────┐
│                  Production S3 Behavior                          │
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                   API Compatibility                      │   │
│   │                                                          │   │
│   │   • Exact AWS API responses (XML format)                │   │
│   │   • Same error codes and messages                        │   │
│   │   • Same HTTP status codes                               │   │
│   │   • Same headers (ETag, x-amz-*)                        │   │
│   │   • AWS Signature V4 validation (optional)              │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                   Feature Parity                         │   │
│   │                                                          │   │
│   │   • Bucket operations (CRUD, policies, ACLs)            │   │
│   │   • Object operations (CRUD, multipart, copy)           │   │
│   │   • Versioning                                           │   │
│   │   • Lifecycle rules                                      │   │
│   │   • Event notifications                                  │   │
│   │   • Presigned URLs                                       │   │
│   │   • CORS configuration                                   │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                   Behavioral Accuracy                    │   │
│   │                                                          │   │
│   │   • Eventual consistency simulation (optional)          │   │
│   │   • Rate limiting / throttling                          │   │
│   │   • Request validation                                   │   │
│   │   • Proper pagination                                    │   │
│   │   • Conditional operations (If-Match, etc.)             │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 3. Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                Production-Grade S3 Emulator                      │
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │              Layer 1: HTTP Gateway                       │   │
│   │                                                          │   │
│   │   • AWS Signature V4 validation                         │   │
│   │   • Request parsing (path-style & virtual-hosted)       │   │
│   │   • Rate limiting                                        │   │
│   │   • Request logging (CloudTrail-like)                   │   │
│   │   • CORS handling                                        │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │              Layer 2: Access Control                     │   │
│   │                                                          │   │
│   │   • Bucket policies (JSON policy evaluation)            │   │
│   │   • ACLs (canned and custom)                            │   │
│   │   • Public access block                                  │   │
│   │   • Object ownership                                     │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │              Layer 3: S3 Operations                      │   │
│   │                                                          │   │
│   │   Bucket Ops    │  Object Ops     │  Advanced           │   │
│   │   ───────────   │  ──────────     │  ────────           │   │
│   │   CreateBucket  │  PutObject      │  Multipart          │   │
│   │   DeleteBucket  │  GetObject      │  Versioning         │   │
│   │   ListBuckets   │  DeleteObject   │  Lifecycle          │   │
│   │   HeadBucket    │  CopyObject     │  Notifications      │   │
│   │   GetBucketLoc  │  HeadObject     │  Replication        │   │
│   │                 │  ListObjects    │                      │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │              Layer 4: Storage Engine                     │   │
│   │                                                          │   │
│   │   ┌───────────────┐    ┌───────────────────────────┐    │   │
│   │   │   Metadata    │    │     Object Data           │    │   │
│   │   │   (SQLite)    │    │     (File System)         │    │   │
│   │   │               │    │                           │    │   │
│   │   │ • Buckets     │    │ • Chunked storage         │    │   │
│   │   │ • Objects     │    │ • Content-addressable     │    │   │
│   │   │ • Versions    │    │ • Deduplication           │    │   │
│   │   │ • Policies    │    │                           │    │   │
│   │   └───────────────┘    └───────────────────────────┘    │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 4. Terraform Compatibility

```
┌─────────────────────────────────────────────────────────────────┐
│                   Terraform Workflow                             │
│                                                                  │
│   # main.tf                                                      │
│   provider "aws" {                                               │
│     endpoints {                                                  │
│       s3 = "http://localhost:4566"                              │
│     }                                                            │
│     skip_credentials_validation = true                          │
│     skip_metadata_api_check     = true                          │
│     skip_requesting_account_id  = true                          │
│     s3_use_path_style           = true                          │
│   }                                                              │
│                                                                  │
│   resource "aws_s3_bucket" "my_bucket" {                        │
│     bucket = "my-app-bucket"                                    │
│   }                                                              │
│                                                                  │
│   resource "aws_s3_bucket_versioning" "versioning" {            │
│     bucket = aws_s3_bucket.my_bucket.id                         │
│     versioning_configuration {                                   │
│       status = "Enabled"                                        │
│     }                                                            │
│   }                                                              │
│                                                                  │
│   resource "aws_s3_bucket_lifecycle_configuration" "lifecycle" {│
│     bucket = aws_s3_bucket.my_bucket.id                         │
│     rule {                                                       │
│       id     = "expire-old"                                     │
│       status = "Enabled"                                        │
│       expiration { days = 90 }                                  │
│     }                                                            │
│   }                                                              │
│                                                                  │
│   ─────────────────────────────────────────────────────────────  │
│                                                                  │
│   $ terraform init                                               │
│   $ terraform plan    # Shows what will be created              │
│   $ terraform apply   # Creates resources in emulator           │
│   $ terraform destroy # Removes resources                        │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 5. API Coverage Required for Terraform

| API | Terraform Resource | Priority |
|-----|---------------------|----------|
| CreateBucket | aws_s3_bucket | P0 |
| DeleteBucket | aws_s3_bucket | P0 |
| HeadBucket | aws_s3_bucket (data) | P0 |
| GetBucketLocation | aws_s3_bucket | P0 |
| PutBucketVersioning | aws_s3_bucket_versioning | P1 |
| GetBucketVersioning | aws_s3_bucket_versioning | P1 |
| PutBucketLifecycle | aws_s3_bucket_lifecycle_configuration | P1 |
| GetBucketLifecycle | aws_s3_bucket_lifecycle_configuration | P1 |
| PutBucketPolicy | aws_s3_bucket_policy | P1 |
| GetBucketPolicy | aws_s3_bucket_policy | P1 |
| PutBucketAcl | aws_s3_bucket_acl | P2 |
| PutBucketCors | aws_s3_bucket_cors_configuration | P2 |
| PutBucketNotification | aws_s3_bucket_notification | P2 |
| PutObject | aws_s3_object | P0 |
| GetObject | aws_s3_object | P0 |
| DeleteObject | aws_s3_object | P0 |

---

## 6. Storage Engine Design

```
┌─────────────────────────────────────────────────────────────────┐
│                   Storage Engine                                 │
│                                                                  │
│   data/                                                          │
│   ├── metadata.db              # SQLite database                │
│   │   ├── buckets              # Bucket metadata                │
│   │   ├── objects              # Object metadata                │
│   │   ├── versions             # Version history                │
│   │   ├── policies             # Bucket policies                │
│   │   ├── lifecycle_rules      # Lifecycle configurations       │
│   │   └── multipart_uploads    # In-progress uploads           │
│   │                                                              │
│   └── objects/                 # Object data (content-addressed)│
│       ├── ab/                                                    │
│       │   └── cdef1234...      # Object content (by hash)       │
│       ├── cd/                                                    │
│       │   └── ef567890...                                       │
│       └── ...                                                    │
│                                                                  │
│   Benefits:                                                      │
│   • Deduplication (same content = same file)                    │
│   • Fast metadata queries (SQLite)                              │
│   • Atomic operations (SQLite transactions)                     │
│   • Persistent across restarts                                   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### SQLite Schema

```sql
-- Buckets table
CREATE TABLE buckets (
    name TEXT PRIMARY KEY,
    region TEXT NOT NULL DEFAULT 'us-east-1',
    created_at DATETIME NOT NULL,
    owner_id TEXT NOT NULL,
    acl TEXT,  -- JSON
    policy TEXT,  -- JSON (bucket policy)
    versioning TEXT DEFAULT 'Disabled',  -- Enabled, Suspended, Disabled
    lifecycle_rules TEXT,  -- JSON array
    cors_rules TEXT,  -- JSON array
    notification_config TEXT,  -- JSON
    public_access_block TEXT,  -- JSON
    object_lock_enabled BOOLEAN DEFAULT FALSE
);

-- Objects table
CREATE TABLE objects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    bucket TEXT NOT NULL,
    key TEXT NOT NULL,
    version_id TEXT,
    is_latest BOOLEAN DEFAULT TRUE,
    content_hash TEXT NOT NULL,  -- Points to file in objects/
    content_length INTEGER NOT NULL,
    content_type TEXT DEFAULT 'application/octet-stream',
    etag TEXT NOT NULL,
    last_modified DATETIME NOT NULL,
    metadata TEXT,  -- JSON (user metadata)
    storage_class TEXT DEFAULT 'STANDARD',
    is_delete_marker BOOLEAN DEFAULT FALSE,
    
    UNIQUE(bucket, key, version_id),
    FOREIGN KEY (bucket) REFERENCES buckets(name)
);

-- Multipart uploads
CREATE TABLE multipart_uploads (
    upload_id TEXT PRIMARY KEY,
    bucket TEXT NOT NULL,
    key TEXT NOT NULL,
    initiated DATETIME NOT NULL,
    
    FOREIGN KEY (bucket) REFERENCES buckets(name)
);

-- Multipart parts
CREATE TABLE multipart_parts (
    upload_id TEXT NOT NULL,
    part_number INTEGER NOT NULL,
    content_hash TEXT NOT NULL,
    size INTEGER NOT NULL,
    etag TEXT NOT NULL,
    
    PRIMARY KEY (upload_id, part_number),
    FOREIGN KEY (upload_id) REFERENCES multipart_uploads(upload_id)
);

-- Indexes for performance
CREATE INDEX idx_objects_bucket_key ON objects(bucket, key);
CREATE INDEX idx_objects_bucket_latest ON objects(bucket, is_latest);
```

---

## 7. Implementation Phases

### Phase 1: Core (Week 1-2)
- [ ] SQLite storage engine
- [ ] CreateBucket, DeleteBucket, HeadBucket, ListBuckets
- [ ] PutObject, GetObject, DeleteObject, HeadObject
- [ ] ListObjectsV2 with pagination
- [ ] Proper XML responses
- [ ] Basic Terraform compatibility

### Phase 2: Versioning (Week 3)
- [ ] PutBucketVersioning, GetBucketVersioning
- [ ] Version IDs on objects
- [ ] ListObjectVersions
- [ ] Delete markers
- [ ] Get specific version

### Phase 3: Policies & ACLs (Week 4)
- [ ] PutBucketPolicy, GetBucketPolicy
- [ ] Basic policy evaluation
- [ ] Canned ACLs
- [ ] PutBucketAcl, GetBucketAcl

### Phase 4: Lifecycle (Week 5)
- [ ] PutBucketLifecycleConfiguration
- [ ] GetBucketLifecycleConfiguration
- [ ] Background lifecycle processor
- [ ] Object expiration
- [ ] Transition to storage classes

### Phase 5: Advanced (Week 6+)
- [ ] Multipart upload
- [ ] CopyObject (including cross-bucket)
- [ ] Presigned URLs
- [ ] CORS
- [ ] Event notifications (webhooks)

---

## 8. Project Structure

```
crates/cloudemu/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── error.rs
│   │
│   ├── gateway/
│   │   ├── mod.rs
│   │   ├── router.rs           # Route matching
│   │   ├── auth.rs             # AWS Signature V4
│   │   ├── cors.rs             # CORS handling
│   │   └── logging.rs          # Request logging
│   │
│   ├── services/
│   │   └── s3/
│   │       ├── mod.rs
│   │       ├── api/
│   │       │   ├── mod.rs
│   │       │   ├── bucket.rs   # Bucket operations
│   │       │   ├── object.rs   # Object operations
│   │       │   ├── multipart.rs
│   │       │   ├── versioning.rs
│   │       │   ├── lifecycle.rs
│   │       │   └── policy.rs
│   │       ├── handlers/
│   │       │   ├── mod.rs
│   │       │   └── ...
│   │       ├── xml/
│   │       │   ├── mod.rs
│   │       │   ├── request.rs  # Parse XML requests
│   │       │   └── response.rs # Generate XML responses
│   │       └── policy/
│   │           ├── mod.rs
│   │           └── evaluator.rs # Policy evaluation engine
│   │
│   └── storage/
│       ├── mod.rs
│       ├── engine.rs           # Storage abstraction
│       ├── sqlite.rs           # SQLite implementation
│       └── filesystem.rs       # Object data storage
│
├── migrations/
│   └── 001_initial.sql
│
└── tests/
    ├── terraform/              # Terraform test configs
    │   ├── basic/
    │   └── versioning/
    └── integration/
        ├── bucket_test.rs
        ├── object_test.rs
        └── terraform_test.rs
```

---

## 9. Validation Strategy

```
┌─────────────────────────────────────────────────────────────────┐
│                   Validation Approach                            │
│                                                                  │
│   1. AWS SDK TESTS                                               │
│      ────────────────                                            │
│      Run official AWS SDK tests against emulator                 │
│      Compare responses with real AWS                             │
│                                                                  │
│   2. TERRAFORM TESTS                                             │
│      ───────────────                                             │
│      Apply terraform configs                                     │
│      Verify state matches expected                               │
│      Test plan/apply/destroy cycle                               │
│                                                                  │
│   3. COMPATIBILITY MATRIX                                        │
│      ────────────────────                                        │
│      Test each API operation                                     │
│      Compare request/response with real S3                       │
│      Document any deviations                                     │
│                                                                  │
│   4. BEHAVIORAL TESTS                                            │
│      ─────────────────                                           │
│      Versioning workflows                                        │
│      Lifecycle expiration                                        │
│      Policy evaluation                                           │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 10. Related Documents

- [11-cloudemu-architecture.md](11-cloudemu-architecture.md) - Base architecture
- [12-infrastructure-emulation.md](12-infrastructure-emulation.md) - Emulation depth
- [13-cloudemu-vs-localstack.md](13-cloudemu-vs-localstack.md) - Comparison
