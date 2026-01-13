# CloudKit SEA ↔️ IAC SEA Architecture Mapping

## Side-by-Side Comparison

| Layer | CloudKit (Rust SDK) | IAC (Terraform) | Purpose |
|-------|---------------------|-----------------|---------|
| **5. Facade** | `cloudkit` crate | `facade/` modules | Public API, provider routing |
| **4. Core** | `cloudkit_core` | `core/` modules | Orchestration, composition |
| **3. API** | `cloudkit_api` | `api/` schemas | Service contracts, interfaces |
| **2. SPI** | `cloudkit_spi` (traits) | `spi/` configs | Extension points, providers |
| **1. Common** | `cloudkit_spi` (types) | `common/` defs | Shared types, validation |

---

## Detailed Layer Mapping

### Layer 5: FACADE (Public Interface)

#### CloudKit
```rust
// cloudkit/src/facade/cloudkit.rs
use cloudkit::CloudKit;
use cloudkit::prelude::*;

// User code
let context = CloudKit::aws()
    .region(Region::aws_us_east_1())
    .build()
    .await?;

let storage = context.storage();
storage.put_object("bucket", "key", data).await?;
```

#### IAC
```hcl
# iac/facade/compute/main.tf
module "compute" {
  source = "../../facade/compute"
  
  provider      = "aws"
  instance_size = "medium"
  instance_name = "web-server"
  
  provider_config = {
    ami = "ami-xxxxx"
  }
}
```

**Parallel:**
- Both provide **provider-agnostic** entry points
- Both route to provider-specific implementations
- Both offer **normalized interfaces** (size, region)

---

### Layer 4: CORE (Orchestration)

#### CloudKit
```rust
// cloudkit_core/src/executor.rs
pub struct OperationExecutor {
    retry_policy: Arc<dyn RetryPolicy>,
    metrics: Arc<dyn MetricsCollector>,
}

impl OperationExecutor {
    pub async fn execute<F, T>(&self, operation: F) -> CloudResult<T>
    where
        F: Fn() -> BoxFuture<'static, CloudResult<T>>,
    {
        // Retry logic
        // Metrics collection
        // Error handling
    }
}
```

#### IAC
```hcl
# iac/core/compute/main.tf
locals {
  instance_type = lookup(
    var.compute_instance_types[var.provider],
    var.instance_size,
    var.compute_instance_types[var.provider]["medium"]
  )
}

module "instance" {
  source = "../../providers/${var.provider}/compute"
  
  instance_name = var.instance_name
  instance_type = local.instance_type
  
  # Dependency management
  depends_on = [module.network]
}

# Lifecycle management
resource "null_resource" "instance_ready" {
  depends_on = [module.instance]
  # Post-creation hooks
}
```

**Parallel:**
- Both handle **resource/operation composition**
- Both manage **dependencies**
- Both implement **retry/lifecycle policies**

---

### Layer 3: API (Service Contracts)

#### CloudKit
```rust
// cloudkit_api/src/object_storage.rs
#[async_trait]
pub trait ObjectStorage: Send + Sync {
    async fn create_bucket(&self, bucket: &str) -> CloudResult<()>;
    
    async fn put_object(
        &self,
        bucket: &str,
        key: &str,
        data: &[u8]
    ) -> CloudResult<()>;
    
    async fn get_object(
        &self,
        bucket: &str,
        key: &str
    ) -> CloudResult<Bytes>;
    
    async fn list_objects(
        &self,
        bucket: &str,
        options: ListOptions
    ) -> CloudResult<ListResult<ObjectMetadata>>;
}
```

#### IAC
```hcl
# iac/api/compute/schema.tf

# Input contract
variable "instance_name" {
  description = "Name of the compute instance"
  type        = string
  validation {
    condition     = can(regex("^[a-z0-9-]+$", var.instance_name))
    error_message = "Instance name must be lowercase alphanumeric"
  }
}

variable "instance_size" {
  description = "Normalized instance size"
  type        = string
}

# Output contract
output "instance_id" {
  description = "Unique identifier of the compute instance"
  value       = local.instance_id
}

output "instance_type" {
  description = "Provider-specific instance type used"
  value       = local.instance_type
}

output "public_ip" {
  description = "Public IP address"
  value       = local.public_ip
}
```

**Parallel:**
- Both define **provider-agnostic interfaces**
- Both use **type safety** (traits vs. variable validation)
- Both specify **input/output contracts**

---

### Layer 2: SPI (Service Provider Interface)

#### CloudKit
```rust
// cloudkit_spi/src/auth.rs
#[async_trait]
pub trait AuthProvider: Send + Sync {
    async fn get_credentials(&self) -> CloudResult<Credentials>;
    async fn refresh_credentials(&self) -> CloudResult<()>;
}

// cloudkit_spi/src/retry.rs
pub trait RetryPolicy: Send + Sync {
    fn should_retry(&self, error: &CloudError, attempt: u32) -> bool;
    fn retry_delay(&self, attempt: u32) -> Duration;
}

// cloudkit_spi/src/metrics.rs
pub trait MetricsCollector: Send + Sync {
    fn record_operation(&self, name: &str, duration: Duration);
    fn record_error(&self, error: &CloudError);
}
```

#### IAC
```hcl
# iac/spi/aws/provider.tf
terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  region = var.aws_region

  default_tags {
    tags = local.common_tags
  }

  # Extension point: Assume role
  dynamic "assume_role" {
    for_each = var.aws_assume_role != null ? [1] : []
    content {
      role_arn     = var.aws_assume_role.role_arn
      session_name = var.aws_assume_role.session_name
    }
  }
}

# iac/spi/aws/backend.tf
terraform {
  backend "s3" {
    bucket         = var.state_bucket
    key            = "${var.environment}/terraform.tfstate"
    region         = var.aws_region
    encrypt        = true
    dynamodb_table = var.state_lock_table
  }
}
```

**Parallel:**
- Both define **extension points** for customization
- Both allow **pluggable implementations**
- Both handle **provider-specific setup**

---

### Layer 1: COMMON (Shared Definitions)

#### CloudKit
```rust
// cloudkit_spi/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum CloudError {
    #[error("Authentication failed: {0}")]
    Auth(#[from] AuthError),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("{resource_type} not found: {resource_id}")]
    NotFound {
        resource_type: String,
        resource_id: String,
    },
    
    #[error("Rate limited, retry after {retry_after:?}")]
    RateLimited { retry_after: Duration },
}

// cloudkit_spi/src/region.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    code: String,
    provider: ProviderType,
    display_name: String,
}

impl Region {
    pub fn aws_us_east_1() -> Self { /* ... */ }
    pub fn azure_eastus() -> Self { /* ... */ }
    pub fn gcp_us_central1() -> Self { /* ... */ }
}

// cloudkit_spi/src/config.rs
#[derive(Debug, Clone)]
pub struct CloudConfig {
    pub region: Region,
    pub timeout: Duration,
    pub max_retries: u32,
    pub enable_tracing: bool,
    pub endpoint: Option<String>,
}
```

#### IAC
```hcl
# iac/common/locals.tf
locals {
  # Size normalization
  compute_instance_types = {
    aws = {
      small  = "t3.micro"
      medium = "t3.medium"
      large  = "m5.large"
    }
    azure = {
      small  = "Standard_B1s"
      medium = "Standard_B2s"
      large  = "Standard_DS2_v2"
    }
    gcp = {
      small  = "e2-micro"
      medium = "e2-medium"
      large  = "n2-standard-2"
    }
  }
}

# iac/common/variables.tf
variable "provider" {
  description = "Cloud provider: aws, azure, gcp, oracle"
  type        = string
  validation {
    condition     = contains(["aws", "azure", "gcp", "oracle"], var.provider)
    error_message = "Valid providers: aws, azure, gcp, oracle"
  }
}

variable "resource_size" {
  description = "Resource size: small, medium, large"
  type        = string
  validation {
    condition     = contains(["small", "medium", "large"], var.resource_size)
    error_message = "Valid sizes: small, medium, large"
  }
}

# iac/common/tags.tf
locals {
  common_tags = merge(
    var.tags,
    {
      ManagedBy   = "Terraform"
      Environment = var.environment
      Provider    = var.provider
    }
  )
}
```

**Parallel:**
- Both define **shared types** and constants
- Both provide **validation** (compile-time vs. Terraform validation)
- Both normalize **provider-specific values**

---

## Provider Implementation Comparison

### CloudKit Provider Structure
```
cloudkit-aws/
├── src/
│   ├── builder.rs         # AwsBuilder
│   ├── s3.rs             # S3Storage (implements ObjectStorage)
│   ├── dynamodb.rs       # DynamoDb (implements KeyValueStore)
│   ├── sqs.rs            # SqsQueue (implements MessageQueue)
│   └── lib.rs
```

### IAC Provider Structure
```
iac/providers/aws/
├── compute/
│   ├── main.tf           # EC2 instance
│   ├── variables.tf
│   └── outputs.tf
├── storage/
│   ├── main.tf           # S3 bucket
│   ├── variables.tf
│   └── outputs.tf
└── database/
    ├── main.tf           # RDS instance
    ├── variables.tf
    └── outputs.tf
```

**Parallel:**
- Both organize by **provider** then **service**
- Both implement the **API contracts**
- Both handle **provider-specific logic**

---

## Dependency Flow Comparison

### CloudKit
```
User Application
       │
       ▼
┌──────────────┐
│   cloudkit   │ (Facade)
└──────────────┘
       │
       ▼
┌──────────────┐
│cloudkit-aws  │ (Provider)
└──────────────┘
       │
       ├──→ cloudkit_api (Implements ObjectStorage)
       ├──→ cloudkit_spi (Uses CloudContext)
       └──→ aws-sdk-s3   (Native SDK)
```

### IAC
```
User Configuration
       │
       ▼
┌──────────────┐
│facade/compute│ (Facade)
└──────────────┘
       │
       ▼
┌──────────────┐
│core/compute  │ (Core)
└──────────────┘
       │
       ▼
┌──────────────┐
│providers/aws │ (Provider)
└──────────────┘
       │
       ├──→ api/compute (Follows schema)
       ├──→ spi/aws     (Uses provider config)
       └──→ AWS API     (Terraform AWS provider)
```

---

## Testing Parallel

### CloudKit Testing
```rust
// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_s3_put_object() {
        let storage = S3Storage::new(context, sdk_config);
        storage.put_object("bucket", "key", b"data").await.unwrap();
    }
}

// Integration tests
#[tokio::test]
async fn test_aws_integration() {
    let client = AwsBuilder::new()
        .region(Region::aws_us_east_1())
        .build()
        .await
        .unwrap();
    
    let storage = client.storage();
    storage.create_bucket("test-bucket").await.unwrap();
}
```

### IAC Testing
```hcl
# Validation
terraform validate

# Unit tests (Terratest)
func TestComputeModule(t *testing.T) {
    terraformOptions := &terraform.Options{
        TerraformDir: "../facade/compute",
    }
    
    terraform.InitAndApply(t, terraformOptions)
    
    instanceID := terraform.Output(t, terraformOptions, "instance_id")
    assert.NotEmpty(t, instanceID)
}
```

---

## Key Design Patterns Shared

### 1. **Builder Pattern**

**CloudKit:**
```rust
let client = AwsBuilder::new()
    .region(Region::aws_us_east_1())
    .profile("default")
    .build()
    .await?;
```

**IAC:**
```hcl
module "compute" {
  source        = "..."
  provider      = "aws"
  instance_size = "medium"
  # ... fluent configuration
}
```

### 2. **Normalization Pattern**

**CloudKit:**
```rust
// Region normalization
Region::aws_us_east_1()  → "us-east-1"
Region::azure_eastus()   → "East US"
Region::gcp_us_central1() → "us-central1"
```

**IAC:**
```hcl
# Size normalization
variable "instance_size" = "medium"
  ↓
aws:   "t3.medium"
azure: "Standard_B2s"
gcp:   "e2-medium"
```

### 3. **Facade Pattern**

**CloudKit:**
```rust
// Single entry point
use cloudkit::CloudKit;

// Provider abstraction
CloudKit::aws() | CloudKit::azure() | CloudKit::gcp()
```

**IAC:**
```hcl
# Single module interface
module "compute" {
  source = "../facade/compute"
  
  # Provider routing
  provider = "aws" | "azure" | "gcp"
}
```

### 4. **Dependency Injection**

**CloudKit:**
```rust
// Inject custom retry policy
let context = CloudContext::builder(ProviderType::Aws)
    .retry_policy(CustomRetryPolicy::new())
    .metrics(PrometheusCollector::new())
    .build()
    .await?;
```

**IAC:**
```hcl
# Inject custom backend
terraform {
  backend "s3" {
    bucket = var.custom_state_bucket
    # ...
  }
}

# Inject custom tags
module "compute" {
  tags = merge(var.custom_tags, local.common_tags)
}
```

---

## Benefits Summary

| Benefit | CloudKit | IAC |
|---------|----------|-----|
| **Modularity** | ✅ Crate separation | ✅ Module separation |
| **Testability** | ✅ Unit + Integration | ✅ Validation + Terratest |
| **Extensibility** | ✅ Trait implementation | ✅ Module composition |
| **Portability** | ✅ Provider swap | ✅ Provider variable |
| **Type Safety** | ✅ Rust compiler | ✅ Terraform validation |
| **Observability** | ✅ Tracing + Metrics | ✅ Tagging + Logging |

---

## Conclusion

The SEA architecture translates remarkably well from **CloudKit (Rust SDK)** to **IAC (Terraform)**:

1. **Same principles** apply across different domains (SDK vs. IaC)
2. **Layer separation** provides clear boundaries
3. **Provider abstraction** enables multi-cloud without complexity
4. **Extensibility** through well-defined interfaces
5. **Testability** at each layer independently

By applying CloudKit's SEA pattern to Terraform infrastructure, we achieve:
- **Consistent mental models** across SDK and infrastructure
- **Reduced cognitive load** (same patterns, different tools)
- **Better maintainability** through proven architecture
- **Team efficiency** (developers familiar with CloudKit understand IAC structure)
