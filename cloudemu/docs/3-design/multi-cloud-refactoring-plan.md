# Multi-Cloud Refactoring Plan

**Audience**: Architects, Core Contributors, and Product Managers.

## WHAT: Evolution to Multi-Cloud Emulation

This plan outlines the architectural refactoring required to transform CloudEmu from an AWS-only emulator into a **unified multi-cloud emulator** supporting AWS, Azure, and GCP. The goal is to enable developers to test cloud infrastructure across all three providers without requiring actual cloud accounts.

**Scope**:
- Refactor current AWS-specific architecture into provider-agnostic abstractions.
- Create provider-specific emulation for Azure and GCP.
- Maintain backward compatibility with existing AWS emulation.
- Support running multiple cloud providers simultaneously (e.g., AWS + Azure).

## WHY: Multi-Cloud Development Reality

### Problems Addressed

1. **Multi-Cloud Strategy Adoption**
   - Impact: 87% of enterprises use multiple cloud providers (Flexera 2025 State of the Cloud Report).
   - Consequence: Developers need local environments that mirror production multi-cloud setups.

2. **Vendor Lock-In Testing**
   - Impact: Organizations want to validate that their applications are truly cloud-agnostic.
   - Consequence: Need to test the same Terraform/SDK code against AWS, Azure, and GCP locally.

3. **Cost of Multi-Cloud Testing**
   - Impact: Testing infrastructure across three clouds multiplies AWS costs 3x.
   - Consequence: Teams skip cross-cloud validation to save money.

### Benefits
- **Unified Testing**: One emulator for all clouds (AWS, Azure, GCP).
- **Port-Based Routing**: AWS on 4566, Azure on 4567, GCP on 4568.
- **Cloud-Agnostic Development**: Validate portability assumptions early.
- **Cost Savings**: Test multi-cloud setups locally without triple cloud bills.

---

## HOW: Phased Refactoring Approach

## Phase 1: Extract Shared Abstractions (2-3 weeks)

### 1.1 Create `cloudemu-core` Crate

**Purpose**: Define provider-agnostic traits and shared infrastructure.

```rust
// cloudemu-core/src/lib.rs

/// Universal resource identifier across cloud providers
pub struct CloudResource {
    pub provider: CloudProvider,
    pub service_type: ServiceType,
    pub id: String,
    pub metadata: HashMap<String, String>,
}

pub enum CloudProvider {
    Aws,
    Azure,
    Gcp,
}

pub enum ServiceType {
    ObjectStorage,   // S3, Blob, GCS
    KeyValue,        // DynamoDB, Cosmos, Firestore
    MessageQueue,    // SQS, Service Bus, Pub/Sub
    PubSub,          // SNS, Event Grid, Pub/Sub
    Functions,       // Lambda, Functions, Cloud Functions
}

/// Provider-agnostic HTTP request/response handling
#[async_trait]
pub trait CloudProvider {
    async fn handle_request(&self, req: Request) -> Result<Response>;
    fn supported_services(&self) -> Vec<ServiceType>;
    fn default_port(&self) -> u16;
}

/// Provider-agnostic storage interface
#[async_trait]
pub trait StorageEngine {
    async fn store(&self, resource: CloudResource) -> Result<()>;
    async fn retrieve(&self, id: &str) -> Result<CloudResource>;
    async fn list(&self, filter: ResourceFilter) -> Result<Vec<CloudResource>>;
    async fn delete(&self, id: &str) -> Result<()>;
}
```

**Acceptance Criteria:**
- ✅ Traits compile and can be implemented for AWS/Azure/GCP
- ✅ No AWS-specific types leak into `cloudemu-core`
- ✅ Unit tests for trait implementations

---

### 1.2 Refactor Current Code to Use Abstractions

**Changes Required:**

```
control-plane/src/services/s3/handlers.rs
BEFORE:
  - Direct SQLite calls
  - AWS-specific error handling
  
AFTER:
  - Use StorageEngine trait
  - Map AWS errors to generic CloudError
```

**Migration Steps:**
1. Create adapter layer: `AWSStorageAdapter` implements `StorageEngine`
2. Update S3 handlers to use `Box<dyn StorageEngine>` instead of direct SQLite
3. Run existing tests to verify no regression
4. Repeat for all 11 AWS services

---

## Phase 2: Create Provider-Specific Crates (3-4 weeks)

### 2.1 Restructure Crate Hierarchy

**New Structure:**

```
cloudemu/
├── crates/
│   ├── cloudemu-core/           # Shared abstractions
│   │   ├── src/
│   │   │   ├── traits.rs        # CloudProvider, StorageEngine
│   │   │   ├── types.rs         # CloudResource, ServiceType
│   │   │   ├── http.rs          # Shared HTTP utilities
│   │   │   └── storage.rs       # Unified storage interface
│   │   └── Cargo.toml
│   │
│   ├── cloudemu-aws/            # AWS emulation
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── provider.rs      # impl CloudProvider
│   │   │   ├── services/        # S3, DynamoDB, SQS, ...
│   │   │   └── storage/         # AWS-specific storage
│   │   └── Cargo.toml
│   │
│   ├── cloudemu-azure/          # Azure emulation
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── provider.rs      # impl CloudProvider
│   │   │   ├── services/        # Blob, Cosmos, Service Bus, ...
│   │   │   └── storage/         # Azure-specific storage
│   │   └── Cargo.toml
│   │
│   ├── cloudemu-gcp/            # GCP emulation
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── provider.rs      # impl CloudProvider
│   │   │   ├── services/        # GCS, Firestore, Pub/Sub, ...
│   │   │   └── storage/         # GCP-specific storage
│   │   └── Cargo.toml
│   │
│   └── cloudemu-server/         # Unified HTTP server
│       ├── src/
│       │   ├── main.rs          # Multi-provider router
│       │   ├── router.rs        # Port-based routing
│       │   └── config.rs        # Provider configuration
│       └── Cargo.toml
```

---

### 2.2 Implement Azure Provider

**Services to Implement:**

| Azure Service | AWS Equivalent | Priority |
|---------------|----------------|----------|
| Blob Storage | S3 | P0 |
| Cosmos DB | DynamoDB | P0 |
| Service Bus | SQS/SNS | P1 |
| Functions | Lambda | P1 |
| Key Vault | Secrets Manager | P1 |
| Event Grid | EventBridge | P2 |

**Example Implementation:**

```rust
// cloudemu-azure/src/provider.rs

pub struct AzureProvider {
    storage: Arc<dyn StorageEngine>,
    services: HashMap<ServiceType, Box<dyn ServiceHandler>>,
}

#[async_trait]
impl CloudProvider for AzureProvider {
    async fn handle_request(&self, req: Request) -> Result<Response> {
        // Parse Azure-specific headers
        let service_type = parse_azure_service(&req)?;
        
        match self.services.get(&service_type) {
            Some(handler) => handler.handle(req).await,
            None => Err(CloudError::UnsupportedService(service_type)),
        }
    }
    
    fn default_port(&self) -> u16 { 4567 }
}
```

**Acceptance Criteria:**
- ✅ Azure Blob Storage emulation works with Azure SDKs
- ✅ Terraform Azure provider can create resources
- ✅ 50+ integration tests pass

---

### 2.3 Implement GCP Provider

**Services to Implement:**

| GCP Service | AWS Equivalent | Priority |
|-------------|----------------|----------|
| Cloud Storage (GCS) | S3 | P0 |
| Firestore | DynamoDB | P0 |
| Pub/Sub | SQS/SNS | P1 |
| Cloud Functions | Lambda | P1 |
| Secret Manager | Secrets Manager | P1 |

**Acceptance Criteria:**
- ✅ GCP Cloud Storage emulation works with Google SDKs
- ✅ Terraform GCP provider can create resources
- ✅ 50+ integration tests pass

---

## Phase 3: Unified Multi-Cloud Server (2-3 weeks)

### 3.1 Port-Based Routing

**Configuration:**

```rust
// cloudemu-server/src/config.rs

pub struct CloudEmuConfig {
    pub providers: Vec<ProviderConfig>,
    pub shared_storage: bool,  // One DB for all clouds vs separate DBs
}

pub struct ProviderConfig {
    pub provider: CloudProvider,
    pub port: u16,
    pub enabled: bool,
    pub data_dir: PathBuf,
}

impl Default for CloudEmuConfig {
    fn default() -> Self {
        Self {
            providers: vec![
                ProviderConfig { provider: CloudProvider::Aws, port: 4566, enabled: true, data_dir: ".cloudemu/aws".into() },
                ProviderConfig { provider: CloudProvider::Azure, port: 4567, enabled: false, data_dir: ".cloudemu/azure".into() },
                ProviderConfig { provider: CloudProvider::Gcp, port: 4568, enabled: false, data_dir: ".cloudemu/gcp".into() },
            ],
            shared_storage: false,
        }
    }
}
```

**Router Implementation:**

```rust
// cloudemu-server/src/main.rs

#[tokio::main]
async fn main() {
    let config = CloudEmuConfig::load();
    
    let mut handles = vec![];
    
    for provider_config in config.providers {
        if !provider_config.enabled {
            continue;
        }
        
        let provider: Box<dyn CloudProvider> = match provider_config.provider {
            CloudProvider::Aws => Box::new(AWSProvider::new(&provider_config)),
            CloudProvider::Azure => Box::new(AzureProvider::new(&provider_config)),
            CloudProvider::Gcp => Box::new(GcpProvider::new(&provider_config)),
        };
        
        let handle = tokio::spawn(async move {
            start_provider_server(provider, provider_config.port).await
        });
        
        handles.push(handle);
    }
    
    // Wait for all servers
    for handle in handles {
        handle.await.unwrap();
    }
}
```

**Startup Output:**

```
   _____ _                 _ ______                
  / ____| |               | |  ____|               
 | |    | | ___  _   _  __| | |__   _ __ ___  _   _ 
 | |    | |/ _ \| | | |/ _` |  __| | '_ ` _ \| | | |
 | |____| | (_) | |_| | (_| | |____| | | | | | |_| |
  \_____|_|\___/ \__,_|\__,_|______|_| |_| |_|\__,_|
                                                    
  Multi-Cloud Local Emulator v0.2.0

  [AWS]     http://0.0.0.0:4566   ✅ Running
  [Azure]   http://0.0.0.0:4567   ✅ Running
  [GCP]     http://0.0.0.0:4568   🔄 Starting...

  Data Dir:  .cloudemu
  Mode:      Isolated (separate storage per provider)
```

---

## Phase 4: Cross-Cloud Features (Optional, 3-4 weeks)

### 4.1 Unified Resource Browser

**API Endpoint:**

```
GET /api/resources?provider=all
```

**Response:**

```json
{
  "resources": [
    {
      "provider": "aws",
      "type": "ObjectStorage",
      "id": "s3://my-bucket",
      "created": "2026-01-14T12:00:00Z"
    },
    {
      "provider": "azure",
      "type": "ObjectStorage",
      "id": "blob://my-container",
      "created": "2026-01-14T12:05:00Z"
    },
    {
      "provider": "gcp",
      "type": "ObjectStorage",
      "id": "gcs://my-bucket",
      "created": "2026-01-14T12:10:00Z"
    }
  ]
}
```

**Use Case**: Developers can see all resources across all clouds in one view.

---

### 4.2 Cross-Cloud Replication (Advanced)

**Example Use Case:**

```rust
// Replicate S3 bucket to Azure Blob and GCS
cloudemu.replicate(
    source: "aws:s3://my-bucket",
    targets: ["azure:blob://my-container", "gcp:gcs://my-bucket"]
);
```

**Why This Matters:**
- Test disaster recovery scenarios
- Validate multi-cloud data synchronization logic
- Benchmark cross-cloud performance

---

## Migration Checklist

### Phase 1: Abstractions (Weeks 1-3)
- [ ] Create `cloudemu-core` crate
- [ ] Define `CloudProvider` trait
- [ ] Define `StorageEngine` trait
- [ ] Create AWS adapter implementing traits
- [ ] Run full AWS test suite (verify no regression)

### Phase 2: Azure (Weeks 4-7)
- [ ] Create `cloudemu-azure` crate
- [ ] Implement Blob Storage emulation
- [ ] Implement Cosmos DB emulation
- [ ] Implement Service Bus emulation
- [ ] Write 50+ Azure integration tests
- [ ] Validate Terraform Azure provider compatibility

### Phase 3: GCP (Weeks 8-11)
- [ ] Create `cloudemu-gcp` crate
- [ ] Implement Cloud Storage emulation
- [ ] Implement Firestore emulation
- [ ] Implement Pub/Sub emulation
- [ ] Write 50+ GCP integration tests
- [ ] Validate Terraform GCP provider compatibility

### Phase 4: Unified Server (Weeks 12-14)
- [ ] Create `cloudemu-server` crate
- [ ] Implement port-based routing
- [ ] Add configuration system
- [ ] Add unified resource browser API
- [ ] Update documentation
- [ ] Release v0.2.0

---

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| **Breaking Changes** | Existing users break | Maintain `cloudemu-aws` as standalone binary |
| **Complexity Growth** | Harder to maintain | Strict adherence to abstractions, automated tests |
| **Performance** | Running 3 providers = 3x memory | Make providers opt-in via config |
| **API Drift** | Azure/GCP APIs change | Version-lock emulated APIs (e.g., "Azure Storage v2023-01-01") |

---

## Success Metrics

**v0.2.0 Release Criteria:**
- ✅ AWS emulation maintains 100% backward compatibility
- ✅ Azure Blob Storage passes 80% of Azure SDK test suite
- ✅ GCP Cloud Storage passes 80% of Google SDK test suite
- ✅ Can run all three providers simultaneously
- ✅ 200+ total integration tests across all providers
- ✅ Documentation updated with multi-cloud examples

---

## Summary

This plan transforms CloudEmu from an AWS-only tool into a **universal cloud emulator**. By extracting shared abstractions first, we maintain AWS stability while enabling rapid Azure and GCP development. The phased approach minimizes risk and allows early validation of the architecture.

**Key Decisions:**
1. **Port-based routing** (4566/4567/4568) for simplicity
2. **Trait-based abstractions** for provider independence
3. **Isolated storage** by default (one DB per provider)
4. **Opt-in providers** to control resource usage

---

**Related Documentation:**
- [Architecture](./architecture.md)
- [Backlog](../4-development/backlog.md)

**Last Updated**: 2026-01-14  
**Version**: 1.0  
**Status**: Proposed
