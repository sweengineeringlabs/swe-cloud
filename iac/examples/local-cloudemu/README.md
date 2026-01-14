# CloudEmu Local Testing Example

This example demonstrates deploying IAC modules to CloudEmu for local testing.

## Prerequisites

1. **CloudEmu Server Running**:
   ```bash
   cd ../../../cloudemu
   cargo run --release -p cloudemu-server
   ```

2. **Terraform**:
   ```bash
   terraform version  # 1.5.0+
   ```

3. **AWS CLI** (for verification):
   ```bash
   aws --version  # 2.0+
   ```

## Quick Start

### 1. Start CloudEmu

```bash
# In cloudemu directory
cargo run --release -p cloudemu-server
```

Wait for output:
```
Starting CloudEmu Multi-Cloud Server...
AWS Provider listening on 127.0.0.1:4566
Azure Provider listening on 127.0.0.1:4567
GCP Provider listening on 127.0.0.1:4568
```

### 2. Deploy Infrastructure

```bash
# Initialize Terraform
terraform init

# Plan deployment
terraform plan

# Apply configuration
terraform apply -auto-approve
```

### 3. Verify Resources

Use the AWS CLI with CloudEmu endpoint:

```bash
# List S3 buckets
aws --endpoint-url=http://localhost:4566 s3 ls

# List DynamoDB tables
aws --endpoint-url=http://localhost:4566 dynamodb list-tables

# List SQS queues
aws --endpoint-url=http://localhost:4566 sqs list-queues

# List SNS topics
aws --endpoint-url=http://localhost:4566 sns list-topics

# List Lambda functions
aws --endpoint-url=http://localhost:4566 lambda list-functions
```

### 4. Test Operations

**Upload to S3**:
```bash
echo "Hello CloudEmu!" > test.txt
aws --endpoint-url=http://localhost:4566 s3 cp test.txt s3://cloudemu-test-bucket/
aws --endpoint-url=http://localhost:4566 s3 ls s3://cloudemu-test-bucket/
```

**DynamoDB Operations**:
```bash
aws --endpoint-url=http://localhost:4566 dynamodb put-item \
  --table-name cloudemu-test-table \
  --item '{"id": {"S": "test-1"}, "name": {"S": "Test Item"}}'

aws --endpoint-url=http://localhost:4566 dynamodb scan \
  --table-name cloudemu-test-table
```

**SQS Operations**:
```bash
# Get queue URL
QUEUE_URL=$(terraform output -raw queue_url)

# Send message
aws --endpoint-url=http://localhost:4566 sqs send-message \
  --queue-url "$QUEUE_URL" \
  --message-body "Hello from CloudEmu!"

# Receive message
aws --endpoint-url=http://localhost:4566 sqs receive-message \
  --queue-url "$QUEUE_URL"
```

### 5. Clean Up

```bash
terraform destroy -auto-approve
```

## What Gets Created

This example deploys:

1. **S3 Bucket** (`cloudemu-test-bucket`)
   - Versioning enabled
   - Encryption enabled

2. **DynamoDB Table** (`cloudemu-test-table`)
   - Pay-per-request billing
   - Hash key: `id`

3. **SQS Queue** (`cloudemu-test-queue`)
   - Standard queue

4. **SNS Topic** (`cloudemu-test-topic`)
   - Standard topic

5. **Lambda Function** (`cloudemu-test-function`)
   - Python 3.11 runtime
   - Simple "Hello" handler

## Customization

Override default variables:

```bash
terraform apply -var="bucket_name=my-custom-bucket" \
                -var="environment=dev"
```

Or create `terraform.tfvars`:
```hcl
bucket_name   = "my-custom-bucket"
database_name = "my-custom-table"
environment   = "dev"
```

## Troubleshooting

### CloudEmu Not Responding

**Problem**: `Error: connection refused`  
**Solution**: Ensure CloudEmu is running:
```bash
curl http://localhost:4566/health
# Should return 200 OK
```

### Resource Already Exists

**Problem**: `BucketAlreadyExists` or similar  
**Solution**: Clean up CloudEmu data:
```bash
# Stop CloudEmu
# Delete data directory
rm -rf ../../../cloudemu/.cloudemu
# Restart CloudEmu
```

### Terraform State Issues

**Problem**: State out of sync  
**Solution**: Destroy and recreate:
```bash
terraform destroy -auto-approve
rm -f terraform.tfstate*
terraform apply -auto-approve
```

## Performance Notes

- **Deployment Time**: ~10-15 seconds (vs minutes on real AWS)
- **State Refresh**: <1 second (vs 5-10 seconds on AWS)
- **Test Iterations**: Can run 100+ times without cost concerns

## Limitations

### Supported
- ✅ S3 (buckets, objects, versioning, policies)
- ✅ DynamoDB (tables, items, basic queries)
- ✅ SQS (queues, messages)
- ✅ SNS (topics, subscriptions, publish)
- ✅ Lambda (function management, mock invocation)
- ✅ KMS (keys, encrypt/decrypt)
- ✅ Secrets Manager (secrets CRUD)

### Not Supported
- ❌ EC2 (compute instances)
- ❌ VPC (networking)
- ❌ RDS (managed databases beyond DynamoDB)
- ❌ IAM (real authentication/authorization)

## Next Steps

1. Run integration tests: `go test -v ../../../test/integration/cloudemu_test.go`
2. Explore multi-service examples
3. Integrate into CI/CD pipeline

## Related Documentation

- [CloudEmu Documentation](../../../../cloudemu/doc/overview.md)
- [IAC Architecture](../../doc/3-design/architecture.md)
- [Integration Plan](../../doc/2-planning/iac-cloudemu-integration-plan.md)
