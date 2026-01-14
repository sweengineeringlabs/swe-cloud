# IAC Integration Tests

Automated integration tests for IAC modules using Terratest and CloudEmu.

## Overview

This directory contains Go-based integration tests that validate IAC Terraform modules deploy correctly to CloudEmu (local cloud emulator).

## Test Suite

### Files

- **`cloudemu_test.go`**: Comprehensive integration tests for all IAC facades

### Test Cases

| Test Function | Coverage | Duration |
|---------------|----------|----------|
| `TestCloudEmuStorageFacade` | S3/Storage facade deployment | ~30s |
| `TestCloudEmuDatabaseFacade` | DynamoDB/Database facade deployment | ~30s |
| `TestCloudEmuMessagingFacade` | SQS/SNS/Messaging facade deployment | ~30s |
| `TestCloudEmuFullStack` | All services together | ~60s |

## Prerequisites

### 1. CloudEmu Server

```bash
# Terminal 1: Start CloudEmu
cd ../../cloudemu
cargo run --release -p cloudemu-server
```

Wait for:
```
AWS Provider listening on 127.0.0.1:4566
Azure Provider listening on 127.0.0.1:4567
GCP Provider listening on 127.0.0.1:4568
```

### 2. Required Tools

- **Go** 1.21+
- **Terraform** 1.5+
- **AWS CLI** 2.0+
- **Terratest** (auto-installed)

## Running Tests

### All Tests

```bash
go test -v -timeout 10m ./...
```

### Single Test

```bash
go test -v -timeout 10m -run TestCloudEmuStorageFacade
```

### Parallel Execution

```bash
go test -v -timeout 10m -parallel 4 ./...
```

### With Coverage

```bash
go test -v -coverprofile=coverage.out ./...
go tool cover -html=coverage.out
```

## Test Structure

Each test follows this pattern:

```go
func TestSomething(t *testing.T) {
    t.Parallel()  // Run concurrently
    
    // 1. Ensure CloudEmu is running
    ensureCloudEmuRunning(t)
    
    // 2. Configure Terraform
    terraformOptions := &terraform.Options{
        TerraformDir: "../../examples/local-cloudemu",
        Vars: map[string]interface{}{
            "bucket_name": "test-" + randomString(),
        },
    }
    
    // 3. Clean up after test
    defer terraform.Destroy(t, terraformOptions)
    
    // 4. Deploy
    terraform.InitAndApply(t, terraformOptions)
    
    // 5. Verify outputs
    bucketName := terraform.Output(t, terraformOptions, "bucket_name")
    assert.NotEmpty(t, bucketName)
    
    // 6. Verify resources in CloudEmu
    verifyS3BucketExists(t, bucketName)
    
    // 7. Test operations
    testS3Upload(t, bucketName)
}
```

## Helper Functions

### CloudEmu Helpers

```go
// Check if CloudEmu is running
ensureCloudEmuRunning(t)

// Verify resources exist
verifyS3BucketExists(t, bucketName)
verifyDynamoDBTableExists(t, tableName)
verifySQSQueueExists(t, queueURL)
verifySNSTopicExists(t, topicARN)
verifyLambdaFunctionExists(t, functionName)
```

### Operation Testers

```go
// Test CRUD operations
testS3Upload(t, bucketName)
testS3Download(t, bucketName)
testDynamoDBPutItem(t, tableName)
testDynamoDBGetItem(t, tableName)
testSQSSendMessage(t, queueURL)
testSQSReceiveMessage(t, queueURL)
testSNSPublish(t, topicARN)
```

## Troubleshooting

### CloudEmu Not Running

**Error**: Test skipped with "CloudEmu not running"

**Solution**:
```bash
# Check if CloudEmu is running
curl http://localhost:4566/health

# If not, start it
cd ../../cloudemu
cargo run --release -p cloudemu-server
```

### Terraform State Conflicts

**Error**: Resource already exists

**Solution**:
```bash
# Tests should clean up automatically, but if not:
cd ../examples/local-cloudemu
terraform destroy -auto-approve
```

### Test Timeouts

**Error**: Test exceeded timeout

**Solution**:
```bash
# Increase timeout
go test -v -timeout 20m ./...

# Or run tests sequentially
go test -v -parallel 1 ./...
```

### AWS CLI Errors

**Error**: Unable to locate credentials

**Solution**:
```bash
# Set dummy credentials (CloudEmu doesn't validate)
export AWS_ACCESS_KEY_ID=test
export AWS_SECRET_ACCESS_KEY=test
export AWS_DEFAULT_REGION=us-east-1
```

## Best Practices

### 1. Use Unique Names

Always generate unique resource names to avoid conflicts:

```go
bucketName := fmt.Sprintf("test-bucket-%d", time.Now().Unix())
```

### 2. Always Clean Up

Use `defer` to ensure cleanup even on failure:

```go
defer terraform.Destroy(t, terraformOptions)
```

### 3. Run Tests in Parallel

Mark tests as parallel when possible:

```go
func TestSomething(t *testing.T) {
    t.Parallel()
    ...
}
```

### 4. Verify Everything

Don't just check Terraform outputs - verify in CloudEmu too:

```go
bucketName := terraform.Output(t, terraformOptions, "bucket_name")
verifyS3BucketExists(t, bucketName)  // Extra verification
```

### 5. Test Operations

Go beyond deployment - test actual operations:

```go
testS3Upload(t, bucketName)
testS3Download(t, bucketName)
```

## CI/CD Integration

These tests are designed to run in CI/CD pipelines:

```yaml
# .github/workflows/test.yml
- name: Run Integration Tests
  run: |
    # Start CloudEmu
    cargo run --release -p cloudemu-server &
    
    # Wait for startup
    sleep 5
    
    # Run tests
    cd iac/test/integration
    go test -v -timeout 10m ./...
```

## Performance

| Metric | Value |
|--------|-------|
| **Single test** | ~30 seconds |
| **Full suite (sequential)** | ~3 minutes |
| **Full suite (parallel)** | ~1 minute |
| **Speedup vs real AWS** | 10-20x |

## Related Documentation

- [CloudEmu Integration Guide](../../doc/4-development/cloudemu-integration.md)
- [Testing Strategy](../../doc/5-testing/testing-strategy.md)
- [Terratest Documentation](https://terratest.gruntwork.io/)

---

**Last Updated**: 2026-01-14
