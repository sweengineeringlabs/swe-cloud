# CloudEmu Integration Guide

**Audience**: Developers, DevOps Engineers

Comprehensive guide for using CloudEmu with IAC Terraform modules for local development and testing.

## WHAT: Local Cloud Testing with CloudEmu

CloudEmu integration enables:
- **Zero-cost testing** of infrastructure changes
- **Fast iteration** (<1 minute deployments vs 5-10 minutes on AWS)
- **Offline development** without internet connectivity
- **CI/CD automation** with consistent environments

## WHY: Benefits of Local Testing

### Problems Solved

1. **AWS Development Costs**
   - Impact: Every `terraform apply` incurs cloud charges
   - Consequence: Developers hesitate to test frequently

2. **Slow Feedback Loops**
   - Impact: AWS API calls take 5-10 minutes for complex deployments
   - Consequence: Reduced productivity

3. **Internet Dependency**
   - Impact: Cannot develop without cloud connectivity
   - Consequence: Travel/offline work blocked

### Benefits

- **Fast**: 10-15 second deployments vs 5-10 minutes
- **Free**: Unlimited testing without AWS costs
- **Reliable**: Deterministic local environment
- **Offline**: Work anywhere

## HOW: Using CloudEmu Integration

### Prerequisites

1. **CloudEmu Server** (from cloudemu crate):
   ```bash
   cd ../cloudemu
   cargo build --release -p cloudemu-server
   ```

2. **Terraform** (1.5.0+):
   ```bash
   terraform version
   ```

3. **AWS CLI** (for verification):
   ```bash
   aws --version  # 2.0+
   ```

4. **Go** (1.21+, for Terratest):
   ```bash
   go version
   ```

### Quick Start

#### 1. Start CloudEmu

```bash
cd ../cloudemu
cargo run --release -p cloudemu-server
```

Wait for output:
```
Starting CloudEmu Multi-Cloud Server...
AWS Provider listening on 127.0.0.1:4566
Azure Provider listening on 127.0.0.1:4567
GCP Provider listening on 127.0.0.1:4568
```

#### 2. Deploy Example Infrastructure

```bash
cd iac/examples/local-cloudemu

# Initialize Terraform
terraform init

# Deploy
terraform apply -auto-approve
```

#### 3. Verify Resources

```bash
# Use verification script
./cloud verify-cloudemu

# Or manually with AWS CLI
aws --endpoint-url=http://localhost:4566 s3 ls
aws --endpoint-url=http://localhost:4566 dynamodb list-tables
aws --endpoint-url=http://localhost:4566 sqs list-queues
```

#### 4. Run Integration Tests

```bash
cd test/integration
go test -v -timeout 10m ./...
```

### Development Workflow

```
┌─────────────────────┐
│ 1. Start CloudEmu   │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│ 2. Write Terraform  │
│    Configuration    │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│ 3. terraform apply  │
│    (to CloudEmu)    │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│ 4. Verify/Test      │
│    (AWS CLI/Tests)  │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│ 5. Iterate quickly  │
│    (< 1 min cycle)  │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│ 6. Deploy to real   │
│    cloud when ready │
└─────────────────────┘
```

## Supported Services

### Fully Supported

| Service | CloudEmu | IAC Facade | Usage |
|---------|----------|-----------|-------|
| **S3** | ✅ Complete | ✅ Storage | Buckets, objects, versioning, policies |
| **DynamoDB** | ✅ Complete | ✅ Database | Tables, items, queries, scans |
| **SQS** | ✅ Complete | ✅ Messaging | Queues, send/receive messages |
| **SNS** | ✅ Complete | ✅ Messaging | Topics, subscriptions, publish |
| **Lambda** | ✅ Mock | ✅ Lambda | Function management, mock invocation |
| **KMS** | ✅ Complete | ⚠️ Partial | Encryption keys, encrypt/decrypt |
| **Secrets Manager** | ✅ Complete | ⚠️ Partial | Secret storage and retrieval |

### Not Supported

| Service | Why Not | Alternative |
|---------|---------|-------------|
| **EC2** | Complex compute emulation | Use real AWS or Docker |
| **VPC** | Network infrastructure | Use real AWS |
| **RDS** | Managed database engines | Use DynamoDB or Docker |
| **IAM** | Authentication/authorization | Mock policies only |

## Configuration Patterns

### Basic AWS Provider Setup

```hcl
provider "aws" {
  region = "us-east-1"
  
  endpoints {
    s3       = "http://localhost:4566"
    dynamodb = "http://localhost:4566"
    sqs      = "http://localhost:4566"
    sns      = "http://localhost:4566"
  }
  
  skip_credentials_validation = true
  skip_metadata_api_check     = true
  skip_requesting_account_id  = true
  s3_use_path_style          = true
  
  access_key = "test"
  secret_key = "test"
}
```

### Using IAC Facades

```hcl
module "storage" {
  source = "../../facade/storage"
  
  provider_name = "aws"
  bucket_name   = "my-test-bucket"
  environment   = "local"
}

module "database" {
  source = "../../facade/database"
  
  provider_name = "aws"
  database_name = "my-test-table"
  environment   = "local"
}
```

### Environment Variables

```bash
# CloudEmu configuration
export CLOUDEMU_DATA_DIR=./.cloudemu
export CLOUDEMU_AWS_PORT=4566

# AWS CLI configuration
export AWS_ENDPOINT_URL=http://localhost:4566
export AWS_ACCESS_KEY_ID=test
export AWS_SECRET_ACCESS_KEY=test
export AWS_DEFAULT_REGION=us-east-1
```

## Testing Strategies

### Unit Tests (Terraform)

```bash
# Validate syntax
terraform validate

# Format check
terraform fmt -check
```

### Integration Tests (Terratest)

```go
func TestStorageFacade(t *testing.T) {
    ensureCloudEmuRunning(t)
    
    terraformOptions := &terraform.Options{
        TerraformDir: "../../examples/local-cloudemu",
    }
    
    defer terraform.Destroy(t, terraformOptions)
    terraform.InitAndApply(t, terraformOptions)
    
    bucketName := terraform.Output(t, terraformOptions, "bucket_name")
    verifyS3BucketExists(t, bucketName)
}
```

### End-to-End Tests

```bash
# Deploy full stack
terraform apply -auto-approve

# Run verification
./cloud verify-cloudemu

# Run application tests
go test -v ./test/e2e/...

# Clean up
terraform destroy -auto-approve
```

## CI/CD Integration

### GitHub Actions

The CloudEmu integration includes a complete CI/CD workflow:

```yaml
# .github/workflows/cloudemu-integration.yml
name: CloudEmu Integration Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - Build CloudEmu
      - Start CloudEmu Server
      - Run Terraform Apply
      - Run Integration Tests
      - Terraform Destroy
```

**To enable**:
- Workflow runs automatically on PR
- Tests complete in ~5 minutes
- No AWS credentials required

### Local Pre-Commit

Add to `.git/hooks/pre-commit`:

```bash
#!/bin/bash
cd iac/examples/local-cloudemu
terraform validate || exit 1
terraform fmt -check || exit 1
echo "✓ Terraform validation passed"
```

## Performance Benchmarks

| Operation | CloudEmu | Real AWS | Speedup |
|-----------|----------|----------|---------|
| **Create S3 Bucket** | 0.1s | 2-3s | 20-30x |
| **Create DynamoDB Table** | 0.2s | 10-15s | 50-75x |
| **Full Stack Deploy** | 10-15s | 5-10 min | 20-40x |
| **Test Iteration** | <1 min | 5-10 min | 5-10x |

## Troubleshooting

### CloudEmu Not Responding

**Symptom**: `Error: connection refused`

**Solution**:
```bash
# Check if CloudEmu is running
curl http://localhost:4566/health

# If not, start it
cd ../cloudemu
cargo run --release -p cloudemu-server
```

### Resource Already Exists

**Symptom**: `BucketAlreadyExists` error

**Solution**:
```bash
# Stop CloudEmu
pkill cloudemu-server

# Clear data
rm -rf ../cloudemu/.cloudemu

# Restart CloudEmu
cargo run --release -p cloudemu-server
```

### Terraform State Issues

**Symptom**: State out of sync

**Solution**:
```bash
# Destroy and recreate
terraform destroy -auto-approve
rm -f terraform.tfstate*
terraform apply -auto-approve
```

### Test Flakiness

**Symptom**: Intermittent test failures

**Solution**:
```bash
# Add retries to Terratest
terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
    ...
    MaxRetries:         3,
    TimeBetweenRetries: 5 * time.Second,
})
```

## Best Practices

### 1. Use Unique Names

```hcl
variable "bucket_name" {
  default = "test-bucket-${random_id.suffix.hex}"
}

resource "random_id" "suffix" {
  byte_length = 4
}
```

### 2. Clean Up After Tests

```go
defer terraform.Destroy(t, terraformOptions)
```

### 3. Verify CloudEmu Health

```go
func ensureCloudEmuRunning(t *testing.T) {
    resp, err := http.Get("http://localhost:4566/health")
    if err != nil {
        t.Skip("CloudEmu not running")
    }
}
```

### 4. Use Parallel Tests

```go
func TestSomething(t *testing.T) {
    t.Parallel()  # Run tests concurrently
    ...
}
```

### 5. Document Limitations

```hcl
# Note: EC2 instances not supported in CloudEmu
# Use real AWS for compute testing
```

## Migration Path

### From Manual AWS Testing

```
Before:
1. Write Terraform
2. Deploy to AWS dev account
3. Wait 5-10 minutes
4. Test manually
5. Destroy resources
6. Pay AWS costs

After:
1. Write Terraform
2. Deploy to CloudEmu
3. Wait 10-15 seconds
4. Run automated tests
5. Iterate quickly
6. Deploy to AWS when confident
```

### From LocalStack

CloudEmu vs LocalStack:
- **Performance**: CloudEmu is Rust-based (faster)
- **Simplicity**: Single binary, no Docker
- **Open Source**: MIT licensed
- **Integration**: Built specifically for this IAC framework

## Advanced Topics

### Custom CloudEmu Ports

```bash
export CLOUDEMU_AWS_PORT=14566
cargo run --release -p cloudemu-server

# Update Terraform
endpoints {
  s3 = "http://localhost:14566"
}
```

### Multiple CloudEmu Instances

```bash
# Instance 1 (default ports)
cargo run --release -p cloudemu-server

# Instance 2 (custom ports)
CLOUDEMU_AWS_PORT=24566 cargo run --release -p cloudemu-server
```

### Debugging CloudEmu

```bash
# Enable debug logging
RUST_LOG=debug cargo run -p cloudemu-server

# View CloudEmu data
ls -la ../cloudemu/.cloudemu/
```

## Resources

- [CloudEmu Documentation](../../../cloudemu/docs/overview.md)
- [IAC Architecture](../3-design/architecture.md)
- [Integration Plan](../2-planning/iac-cloudemu-integration-plan.md)
- [Example Configurations](../../examples/local-cloudemu/)
- [Integration Tests](../../test/integration/cloudemu_test.go)

---

**Last Updated**: 2026-01-14  
**Maintained By**: Infrastructure Team
