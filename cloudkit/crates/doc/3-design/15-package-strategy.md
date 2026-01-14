# ADR: Package by Provider (Vertical Slicing)

**Status:** Accepted  
**Date:** 2026-01-14  
**Deciders:** Architecture Team  
**Context:** CloudKit Multi-Cloud SDK Organization

---

## Context and Problem Statement

When organizing a multi-cloud SDK with multiple providers (AWS, Azure, GCP, Oracle) and multiple service types (S3, DynamoDB, Blob, Cosmos, etc.), we must decide:

**Should we organize code by provider (vertical slicing) or by service type (horizontal slicing)?**

### Two Approaches

**Option A: Package by Service Type (Horizontal Slicing)**
```
cloudkit_core/
├── storage/
│   ├── aws_s3.rs
│   ├── azure_blob.rs
│   └── gcp_gcs.rs
├── database/
│   ├── aws_dynamodb.rs
│   ├── azure_cosmos.rs
│   └── gcp_firestore.rs
└── messaging/
    ├── aws_sqs.rs
    ├── azure_queue.rs
    └── gcp_pubsub.rs
```

**Option B: Package by Provider (Vertical Slicing)** ✅
```
cloudkit_core/
├── aws/
│   ├── s3.rs
│   ├── dynamodb.rs
│   └── sqs.rs
├── azure/
│   ├── blob.rs
│   ├── cosmos.rs
│   └── queue.rs
└── gcp/
    ├── gcs.rs
    ├── firestore.rs
    └── pubsub.rs
```

---

## Decision Drivers

1. **Team Organization** - How teams are structured (provider teams vs feature teams)
2. **Code Cohesion** - Related code should be together
3. **Dependency Management** - Provider SDKs are separate dependencies
4. **Navigation** - Developer ease of finding code
5. **Change Patterns** - How changes typically happen
6. **Bounded Contexts** - DDD principle of context boundaries
7. **Conway's Law** - Architecture mirrors organization structure

---

## Considered Options

### Option A: Package by Service Type (Horizontal Slicing)

**Pros:**
- ✅ Easy to see all implementations of a service (e.g., all storage solutions)
- ✅ Facilitates cross-provider abstractions
- ✅ Service-specific utilities can be shared

**Cons:**
- ❌ Provider-specific code scattered across directories
- ❌ Changes to one provider touch multiple directories
- ❌ Hard to isolate dependencies per provider
- ❌ Team boundaries unclear (who owns storage?)
- ❌ Adding new provider touches many files
- ❌ Breaking provider context boundaries

### Option B: Package by Provider (Vertical Slicing) ✅

**Pros:**
- ✅ High cohesion: all AWS code together
- ✅ Clear team ownership: AWS team owns `aws/`
- ✅ Isolated dependencies: AWS SDK only in `aws/`
- ✅ Easy navigation: "Need AWS?" → `aws/`
- ✅ Adding provider = one new directory
- ✅ Matches provider bounded contexts
- ✅ Follows Conway's Law
- ✅ Feature flags per provider simple
- ✅ Vendor-specific optimizations isolated

**Cons:**
- ❌ Harder to see all storage implementations at once
- ❌ Cross-provider abstractions require indirection

---

## Decision Outcome

**Chosen option: Package by Provider (Option B)** ✅

### Rationale

1. **Team Boundaries**
   - Real-world teams are organized by cloud provider (AWS team, Azure team)
   - Each team can own their provider directory independently
   - Reduces coordination overhead between teams

2. **Dependency Isolation**
   ```rust
   // AWS dependencies only loaded when aws feature enabled
   [dependencies]
   aws-sdk-s3 = { workspace = true, optional = true }
   
   [features]
   aws = ["dep:cloudkit-aws"]
   ```

3. **Change Patterns**
   - 90% of changes affect single provider
   - Provider API changes affect one directory
   - Adding new service to AWS only touches `aws/`

4. **Domain-Driven Design**
   - Each provider is a **bounded context**
   - AWS, Azure, GCP have different concepts, idioms, APIs
   - Forcing cross-provider abstraction prematurely violates contexts

5. **Conway's Law**
   - Organization: AWS team, Azure team, GCP team
   - Architecture mirrors this naturally with provider-grouped code

6. **Navigation & Discoverability**
   ```
   Developer: "I need to add AWS Lambda support"
   With Provider Grouping: → cloudkit_core/aws/lambda.rs ✅
   With Service Grouping: → cloudkit_core/functions/aws_lambda.rs 🤔
   ```

---

## Implementation

### Directory Structure
```
cloudkit_core/
├── aws/
│   ├── Cargo.toml         # Separate crate
│   ├── src/
│   │   ├── lib.rs
│   │   ├── builder.rs
│   │   ├── s3.rs          # S3 service
│   │   ├── dynamodb.rs    # DynamoDB service
│   │   ├── sqs.rs         # SQS service
│   │   ├── sns.rs         # SNS service
│   │   ├── lambda.rs      # Lambda service
│   │   └── ...
│   └── README.md
├── azure/
│   ├── Cargo.toml
│   └── src/
│       ├── blob.rs
│       ├── cosmos.rs
│       └── ...
└── gcp/
    ├── Cargo.toml
    └── src/
        ├── gcs.rs
        ├── firestore.rs
        └── ...
```

### Workspace Configuration
```toml
[workspace]
members = [
    "cloudkit_core/aws",
    "cloudkit_core/azure",
    "cloudkit_core/gcp",
]

[features]
aws = ["dep:cloudkit-aws"]
azure = ["dep:cloudkit-azure"]
gcp = ["dep:cloudkit-gcp"]
```

### Team Ownership
```
CODEOWNERS:
/cloudkit_core/aws/     @aws-team
/cloudkit_core/azure/   @azure-team
/cloudkit_core/gcp/     @gcp-team
```

---

## Consequences

### Positive

1. **Clear Ownership**
   - Each provider team has clear directory ownership
   - No cross-team file conflicts

2. **Isolated Dependencies**
   - AWS SDK only compiled when AWS feature enabled
   - Reduces compilation time for single-provider users

3. **Easy Provider Addition**
   ```bash
   # Add Oracle support
   mkdir cloudkit_core/oracle
   # All Oracle code goes there
   ```

4. **Provider-Specific Optimizations**
   - AWS can use AWS-specific patterns without affecting others
   - Azure can leverage Azure idioms

5. **Feature Flags**
   ```rust
   #[cfg(feature = "aws")]
   pub use cloudkit_aws::AwsBuilder;
   ```

### Negative

1. **Cross-Provider Abstractions Harder**
   - Solution: Define traits in `cloudkit_api` layer
   - Providers implement common traits

2. **Duplication Possible**
   - Solution: Extract common patterns to `cloudkit_spi`
   - Share utilities via shared crate

### Mitigation Strategies

**For Cross-Provider Abstractions:**
```rust
// cloudkit_api/src/storage.rs
pub trait ObjectStorage {
    async fn get_object(&self, key: &str) -> Result<Bytes>;
    async fn put_object(&self, key: &str, data: Bytes) -> Result<()>;
}

// cloudkit_core/aws/src/s3.rs
impl ObjectStorage for S3Client { ... }

// cloudkit_core/azure/src/blob.rs
impl ObjectStorage for BlobClient { ... }
```

**For Shared Utilities:**
```rust
// cloudkit_spi/src/retry.rs
pub struct RetryPolicy { ... }  // Used by all providers
```

---

## Related Decisions

- [ADR: Stratified Encapsulation Architecture](./architecture.md)
- [ADR: Provider Feature Flags](./feature-flags.md)
- [ADR: Workspace Structure](./workspace.md)

---

## References

- **Domain-Driven Design** - Eric Evans (Bounded Contexts)
- **Conway's Law** - Melvin Conway, 1967
- **Package by Feature** - Uncle Bob Martin (Clean Architecture)
- **Screaming Architecture** - Robert C. Martin
- **Vertical Slice Architecture** - Jimmy Bogard

---

## Examples in the Wild

Projects using **Package by Provider**:
- AWS SDK for Rust (by service)
- Pulumi (by cloud provider)
- Terraform Providers (separate per cloud)
- Google Cloud Client Libraries (by product)

Projects using **Package by Service Type**:
- Some ORMs (by database type: postgres/, mysql/)
- Some web frameworks (by layer: controllers/, models/)

---

**Decision:** Package by Provider (Vertical Slicing) ✅  
**Aligns with:** DDD, Conway's Law, Team Boundaries, Dependency Isolation  
**Result:** `cloudkit_core/aws/`, `cloudkit_core/azure/`, `cloudkit_core/gcp/`
