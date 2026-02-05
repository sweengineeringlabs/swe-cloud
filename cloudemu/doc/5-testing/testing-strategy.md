# CloudEmu Testing Strategy

**Audience**: QA Engineers, Core Contributors, and DevOps Engineers.

## WHAT: Multi-Tiered Testing Approach

CloudEmu uses a comprehensive testing strategy that validates the emulator's accuracy against real AWS API responses. Due to the complexity of emulating multiple services, we employ unit tests, integration tests, and compatibility tests.

**Scope**:
- Unit testing for individual service handlers and storage operations.
- Integration testing against the full HTTP stack.
- Compatibility testing with Terraform and AWS SDKs.
- Performance benchmarking for production load scenarios.

## WHY: Accuracy and Reliability

### Problems Addressed

1. **API Compatibility**
   - Impact: CloudEmu must match AWS API responses exactly.
   - Consequence: Terraform and SDKs fail if XML/JSON responses are malformed.

2. **Concurrent Access**
   - Impact: Multiple clients accessing the same bucket or queue.
   - Consequence: Race conditions or data corruption if not properly tested.

### Benefits
- **Developer Confidence**: Know that local tests translate to production behavior.
- **Regression Prevention**: Automated tests catch breaking changes before deployment.
- **Performance Validation**: Benchmark against target throughput (1000+ req/s).

## HOW: Testing Hierarchy

### 1. Unit Tests (Service Layer)

Test individual service handlers in isolation using mock storage:

```rust
#[tokio::test]
async fn test_s3_put_object_handler() {
    let mock_storage = MockStorage::new();
    let handler = S3Handler::new(Arc::new(mock_storage));
    
    let result = handler.put_object("bucket", "key", b"data").await;
    assert!(result.is_ok());
}
```

### 2. Integration Tests (HTTP Layer)

Test the full request/response cycle through Axum:

```rust
#[tokio::test]
async fn test_terraform_s3_workflow() {
    let app = start_cloudemu_test_server().await;
    
    // Use Terraform AWS provider against CloudEmu endpoint
    terraform_apply(&app.endpoint).await;
    
    // Verify resources were created
    assert_bucket_exists(&app, "my-bucket").await;
}
```

### 3. Compatibility Tests

Validate that AWS SDKs work seamlessly:

```rust
#[tokio::test]
#[ignore] // Requires CloudEmu running
async fn test_aws_sdk_s3() {
    let sdk_config = aws_config::load_from_env()
        .endpoint_url("http://localhost:4566")
        .await;
    
    let s3 = aws_sdk_s3::Client::new(&sdk_config);
    
    s3.create_bucket().bucket("test").send().await.unwrap();
}
```

### 4. Performance Benchmarks

Use Criterion to ensure throughput targets:

```rust
fn bench_s3_put(c: &mut Criterion) {
    c.bench_function("s3_put_1kb", |b| {
        b.iter(|| s3.put_object("bucket", "key", black_box(&data)))
    });
}
```

---

## Summary

CloudEmu's testing strategy ensures that the emulator is a drop-in replacement for AWS services during development. By combining unit, integration, compatibility, and performance tests, we maintain high confidence in production readiness.

**Key Takeaways**:
1. All service handlers must have unit tests.
2. Integration tests must validate full HTTP workflows.
3. Use the `#[ignore]` attribute for tests requiring a running emulator.

---

**Related Documentation**:
- [Backlog](../4-development/backlog.md)
- [Architecture](../3-design/architecture.md)

**Last Updated**: 2026-01-14  
**Version**: 1.0
