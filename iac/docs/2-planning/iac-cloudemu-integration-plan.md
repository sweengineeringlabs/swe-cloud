# IAC-CloudEmu Integration Plan

**Version**: 1.0  
**Date**: 2026-01-14  
**Status**: Planning  
**Owner**: Infrastructure Team

---

## Executive Summary

Integrate the IAC Terraform framework with CloudEmu local cloud emulator to enable:
- **Fast local development** without cloud API latency
- **Zero-cost testing** of infrastructure changes
- **Offline development** capability
- **CI/CD integration** for automated testing

**Target**: AWS integration first (CloudEmu has 11 AWS services emulated)

---

## Current State

### CloudEmu Capabilities

| Provider | Port | Status | Services Available |
|----------|------|--------|-------------------|
| **AWS** | 4566 | ✅ Stable | S3, DynamoDB, SQS, SNS, Lambda, KMS, Secrets Manager, CloudWatch, EventBridge, Cognito, Step Functions |
| **Azure** | 4567 | ⚠️ Beta | Blob Storage (basic) |
| **GCP** | 4568 | 🚧 Alpha | Connectivity only |

### IAC Capabilities

- ✅ **Terraform Facades**: Storage, Compute, Database, Networking, IAM, Monitoring, Lambda, Messaging
- ✅ **Multi-Provider Support**: AWS, Azure, GCP abstraction
- ✅ **Testing Framework**: Terratest (Go)
- ✅ **SEA Architecture**: 5-layer abstraction (Common, SPI, API, Core, Facade)

### Gap Analysis

| Feature | IAC Support | CloudEmu Support | Integration Status |
|---------|-------------|------------------|-------------------|
| S3/Storage | ✅ | ✅ | 🟢 Ready |
| DynamoDB | ✅ | ✅ | 🟢 Ready |
| SQS | ✅ | ✅ | 🟢 Ready |
| SNS | ✅ | ✅ | 🟢 Ready |
| Lambda | ✅ | ✅ | 🟡 Partial (mock invoke) |
| Compute (EC2) | ✅ | ❌ | 🔴 Not Available |
| Networking (VPC) | ✅ | ❌ | 🔴 Not Available |

**Verdict**: AWS S3, DynamoDB, SQS, SNS integration feasible immediately.

---

## Goals and Non-Goals

### Goals

1. ✅ Enable local testing of IAC modules against CloudEmu
2. ✅ Integrate Terratest with CloudEmu for automated testing
3. ✅ Document CloudEmu setup for IAC developers
4. ✅ Create CI/CD workflow for CloudEmu-based testing
5. ✅ Validate multi-cloud facade patterns work locally

### Non-Goals

1. ❌ Emulate AWS features not in CloudEmu (EC2, VPC)
2. ❌ Wait for Azure/GCP emulation maturity
3. ❌ Replace cloud provider testing entirely
4. ❌ Emulate IAM/authentication beyond basic mocking

---

## Architecture

### Integration Pattern

```
Developer Machine / CI Environment
├── CloudEmu Server (Rust)
│   ├── Port 4566: AWS Emulator
│   ├── Port 4567: Azure Emulator
│   └── Port 4568: GCP Emulator
│
└── IAC Framework (Terraform + Go)
    ├── Terraform Configs
    │   └── Provider endpoints → CloudEmu
    ├── Terratest Suite
    │   └── Validates deployed resources
    └── Examples
        └── local-cloudemu/ (demo usage)
```

### Request Flow

```
Terraform Apply
    ↓
AWS Provider (configured with CloudEmu endpoint)
    ↓
HTTP Request to localhost:4566
    ↓
CloudEmu AWS Provider
    ↓
Service Handler (S3, DynamoDB, etc.)
    ↓
Storage Engine (SQLite + FS)
    ↓
Response (AWS-compatible XML/JSON)
    ↓
Terraform State Updated
```

---

## Implementation Phases

### Phase 1: Foundation (Week 1)

**Tasks**:
1. Create `iac/examples/local-cloudemu/` directory
2. Add example Terraform config with CloudEmu endpoints
3. Document CloudEmu setup in `iac/docs/4-development/cloudemu-integration.md`
4. Add prerequisites (Rust, CloudEmu installation)

**Deliverables**:
- [ ] `examples/local-cloudemu/main.tf`
- [ ] `examples/local-cloudemu/variables.tf`
- [ ] `examples/local-cloudemu/outputs.tf`
- [ ] `docs/4-development/cloudemu-integration.md`

**Success Criteria**:
- Developer can run `terraform apply` successfully against CloudEmu
- S3 bucket created in CloudEmu visible via `aws --endpoint-url=http://localhost:4566 s3 ls`

---

### Phase 2: Storage Facade Integration (Week 1-2)

**Tasks**:
1. Adapt `facade/storage` module for CloudEmu
2. Create example using storage facade with CloudEmu backend
3. Add verification scripts

**Deliverables**:
- [ ] `examples/local-cloudemu/storage-example.tf`
- [ ] `scripts/verify-cloudemu-storage.sh`

**Success Criteria**:
- Storage facade deploys successfully to CloudEmu
- Objects can be uploaded/retrieved via AWS CLI

---

### Phase 3: Automated Testing (Week 2)

**Tasks**:
1. Create Terratest integration test for CloudEmu
2. Add helper functions for CloudEmu lifecycle
3. Implement test fixtures for common scenarios

**Deliverables**:
- [ ] `test/integration/cloudemu_storage_test.go`
- [ ] `test/integration/cloudemu_helpers.go`
- [ ] `test/fixtures/cloudemu/`

**Example Test**:
```go
func TestStorageFacadeWithCloudEmu(t *testing.T) {
    // Ensure CloudEmu is running
    ensureCloudEmuRunning(t)
    
    terraformOptions := &terraform.Options{
        TerraformDir: "../../examples/local-cloudemu",
        Vars: map[string]interface{}{
            "bucket_name": "test-bucket-" + random.UniqueId(),
        },
    }
    
    defer terraform.Destroy(t, terraformOptions)
    terraform.InitAndApply(t, terraformOptions)
    
    // Verify bucket exists in CloudEmu
    bucketName := terraform.Output(t, terraformOptions, "bucket_name")
    verifyBucketExistsInCloudEmu(t, bucketName)
}
```

**Success Criteria**:
- `go test -v ./test/integration` passes
- Test creates, verifies, and destroys resources in CloudEmu

---

### Phase 4: CI/CD Integration (Week 3)

**Tasks**:
1. Create GitHub Actions workflow
2. Add CloudEmu startup/shutdown steps
3. Run Terratest suite in CI
4. Add status badges

**Deliverables**:
- [ ] `.github/workflows/cloudemu-integration.yml`
- [ ] `scripts/ci/start-cloudemu.sh`
- [ ] `scripts/ci/stop-cloudemu.sh`

**Workflow Structure**:
```yaml
name: CloudEmu Integration Tests

on: [push, pull_request]

jobs:
  test-with-cloudemu:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        
      - name: Build CloudEmu
        run: |
          cd ../cloudemu
          cargo build --release -p cloudemu-server
          
      - name: Start CloudEmu
        run: |
          ../cloudemu/target/release/cloudemu-server &
          sleep 5
          
      - name: Run Integration Tests
        run: |
          cd iac
          go test -v ./test/integration
```

**Success Criteria**:
- CI passes on every PR
- CloudEmu starts/stops cleanly
- Tests run in <5 minutes

---

### Phase 5: Advanced Features (Week 4+)

**Tasks**:
1. DynamoDB facade integration
2. SQS/SNS messaging integration
3. Multi-service examples
4. Performance benchmarking

**Deliverables**:
- [ ] `examples/local-cloudemu/database-example.tf`
- [ ] `examples/local-cloudemu/messaging-example.tf`
- [ ] `examples/local-cloudemu/multi-service-app.tf`
- [ ] `docs/4-development/cloudemu-performance.md`

**Success Criteria**:
- Database facade works with CloudEmu DynamoDB
- Messaging facade works with CloudEmu SQS/SNS
- Full application stack deployable locally

---

## Technical Specifications

### CloudEmu Configuration

**Start Command**:
```bash
cd ../cloudemu
cargo run --release -p cloudemu-server
```

**Endpoints**:
- AWS: `http://localhost:4566`
- Azure: `http://localhost:4567`
- GCP: `http://localhost:4568`

**Environment Variables**:
```bash
export CLOUDEMU_DATA_DIR=./.cloudemu
export CLOUDEMU_AWS_PORT=4566
export CLOUDEMU_AZURE_PORT=4567
export CLOUDEMU_GCP_PORT=4568
```

### Terraform Provider Configuration

```hcl
provider "aws" {
  region = "us-east-1"
  
  endpoints {
    s3             = "http://localhost:4566"
    dynamodb       = "http://localhost:4566"
    sqs            = "http://localhost:4566"
    sns            = "http://localhost:4566"
    lambda         = "http://localhost:4566"
    kms            = "http://localhost:4566"
    secretsmanager = "http://localhost:4566"
    cloudwatch     = "http://localhost:4566"
    events         = "http://localhost:4566"
  }
  
  # Skip real AWS API calls
  skip_credentials_validation = true
  skip_metadata_api_check     = true
  skip_requesting_account_id  = true
  
  # S3 path-style for compatibility
  s3_use_path_style = true
  
  # Dummy credentials
  access_key = "test"
  secret_key = "test"
}
```

### Testing Configuration

**Terratest Helpers**:
```go
func ensureCloudEmuRunning(t *testing.T) {
    resp, err := http.Get("http://localhost:4566/health")
    if err != nil || resp.StatusCode != 200 {
        t.Skip("CloudEmu not running. Start with: cargo run -p cloudemu-server")
    }
}

func verifyBucketExistsInCloudEmu(t *testing.T, bucketName string) {
    cmd := exec.Command("aws", 
        "--endpoint-url=http://localhost:4566",
        "s3", "ls", "s3://" + bucketName)
    output, err := cmd.CombinedOutput()
    assert.NoError(t, err)
    assert.Contains(t, string(output), bucketName)
}
```

---

## Resource Requirements

### Development Environment

| Resource | Requirement | Purpose |
|----------|-------------|---------|
| **Rust** | 1.70+ | Build CloudEmu |
| **Cargo** | 1.70+ | CloudEmu dependency management |
| **Terraform** | 1.5+ | IAC deployment |
| **Go** | 1.21+ | Terratest |
| **AWS CLI** | 2.0+ | Verification |
| **RAM** | 2GB+ | CloudEmu + Terraform |
| **Disk** | 1GB+ | CloudEmu data dir |

### CI Environment

| Resource | Requirement |
|----------|-------------|
| **GitHub Runner** | ubuntu-latest |
| **Build Time** | ~2 minutes (CloudEmu) |
| **Test Time** | ~3 minutes (Terratest) |
| **Total CI Time** | ~5 minutes |

---

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| CloudEmu API incompatibility | Medium | High | Test with real AWS CLI, version pin CloudEmu |
| CI flakiness | Medium | Medium | Add retries, health checks, timeouts |
| Feature gaps (EC2, VPC) | High | Medium | Document limitations clearly, use real cloud for advanced features |
| Performance issues | Low | Low | Benchmark early, optimize if needed |
| Breaking changes in CloudEmu | Low | High | Pin to CloudEmu releases, automated testing |

---

## Success Metrics

### Phase 1-2 (Foundation)
- [ ] 100% of storage facade tests pass against CloudEmu
- [ ] <1 minute local deployment time
- [ ] Documentation complete

### Phase 3 (Testing)
- [ ] Terratest suite covers 80%+ of CloudEmu services
- [ ] Zero false positives in tests
- [ ] Test execution <3 minutes

### Phase 4 (CI/CD)
- [ ] CI passes on all PRs
- [ ] CloudEmu integration tests run automatically
- [ ] <5 minute total CI time

### Phase 5 (Advanced)
- [ ] Multi-service applications deployable locally
- [ ] 5+ example scenarios documented
- [ ] Developer adoption >80%

---

## Timeline

```
Week 1
├── Phase 1: Foundation (Days 1-3)
└── Phase 2: Storage Facade (Days 4-5)

Week 2
├── Phase 2: Storage Facade completion (Day 1)
└── Phase 3: Automated Testing (Days 2-5)

Week 3
└── Phase 4: CI/CD Integration (Days 1-5)

Week 4+
└── Phase 5: Advanced Features (Ongoing)
```

**Estimated Total Effort**: 3-4 weeks (1 developer)

---

## Next Steps

1. ✅ Review and approve this plan
2. Create tracking issue in GitHub
3. Set up project board
4. Assign developer
5. Begin Phase 1 implementation

---

## References

- [CloudEmu Documentation](../../../cloudemu/docs/overview.md)
- [IAC Architecture](../3-design/architecture.md)
- [Terraform AWS Provider Docs](https://registry.terraform.io/providers/hashicorp/aws/latest/docs)
- [Terratest Guide](https://terratest.gruntwork.io/)

---

**Last Updated**: 2026-01-14  
**Next Review**: 2026-01-21 (Weekly during implementation)
