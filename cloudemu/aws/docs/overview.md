# AWS Module Overview

## WHAT
This module provides local emulation for Amazon Web Services (AWS) core services. It implements the AWS Wire Protocol (JSON/XML) to ensure compatibility with standard AWS tools.

### Supported Services
| Service | Type | Status |
|---------|------|--------|
| **S3** | Object Storage | ✅ Active |
| **DynamoDB** | NoSQL Database | ✅ Active |
| **SQS** | Queue | ✅ Active |
| **SNS** | Pub/Sub | ✅ Active |
| **Lambda** | Compute | ✅ Active |
| **Secrets Manager** | Security | ✅ Active |
| **Pricing** | FinOps | ✅ Active |

## WHY
- **Cost**: Avoid AWS bill shock during development.
- **Speed**: Instant resource creation/deletion.
- **Offline**: Develop without internet connectivity.

## HOW

### 1. Prerequisites
- **AWS CLI** (v2+)
- **Rust** (to run the server)

### 2. Configuration
Point your AWS tools to the local emulator endpoint:

```bash
export AWS_ENDPOINT_URL=http://localhost:4566
export AWS_ACCESS_KEY_ID=test
export AWS_SECRET_ACCESS_KEY=test
export AWS_REGION=us-east-1
```

### 3. Usage Example

```bash
# Create a bucket
aws s3 mb s3://my-bucket

# Upload a file
echo "Hello" > test.txt
aws s3 cp test.txt s3://my-bucket/
```

### 4. Examples and Tests
- **Integration Tests**: `cloudemu/aws/control-plane/aws-control-core/tests/integration.rs`
- **Unit Tests**: Run `cargo test -p aws-data-core`
