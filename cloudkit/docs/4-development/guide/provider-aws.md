# AWS Provider Guide

**Audience**: Developers and DevOps Engineers deploying to Amazon Web Services.

## WHAT: AWS Implementation of CloudKit

The AWS provider (`cloudkit-aws`) implements the universal CloudKit traits using the official AWS SDK for Rust. It allows all CloudKit services (Storage, DB, Queues, etc.) to run on AWS infrastructure with high performance and native reliability.

**Scope**:
- Supported services (S3, DynamoDB, SQS, SNS, Lambda).
- Authentication methods (Env vars, profiles, IAM roles).
- Service-specific configuration (FIFO queues, S3 options).
- Error mapping and best practices.

## WHY: Native Performance on AWS

### Problems Addressed

1. **SDK Complexity**
   - Impact: The official AWS SDK has thousands of types and a complex builder pattern.
   - Consequence: High boilerplate for simple operations like "Upload File".

2. **Credential Management**
   - Impact: Securely handling keys across local development and production.
   - Consequence: Risk of leaked keys if not handled through standard AWS patterns.

### Benefits
- **Simplified API**: One-line operations for common S3 and SQS tasks.
- **Native Security**: Seamlessly integrates with AWS IAM roles and profiles.
- **Async Efficiency**: Built on Tokio for non-blocking I/O.

## HOW: Configuration & Usage

### 1. Installation

```toml
[dependencies]
cloudkit = "0.1"
cloudkit-aws = "0.1"
```

### 2. Authentication

The AWS provider supports standard AWS credential discovery:

```rust
use cloudkit_aws::AwsBuilder;

let aws = AwsBuilder::new()
    .region(Region::aws_us_east_1())
    .build()
    .await?;
```

### 3. Service Usage (S3 Example)

```rust
let storage = aws.storage();
storage.put_object("my-bucket", "hello.txt", b"Hello AWS!").await?;
```

---

## Summary

The AWS provider is the most mature implementation in the CloudKit ecosystem. It provides a clean, ergonomic interface while leveraging the full power of the official AWS SDK under the hood.

**Key Takeaways**:
1. Prefers IAM roles for production deployments.
2. Supports all major serverless and storage services.
3. Automatically maps AWS error codes to `CloudError`.

---

**Related Documentation**:
- [Getting Started](./getting-started.md)
- [Architecture Hub](../../3-design/architecture.md)

**Last Updated**: 2026-01-14  
**Version**: 1.0
