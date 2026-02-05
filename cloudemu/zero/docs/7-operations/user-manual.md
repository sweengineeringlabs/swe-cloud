# ZeroCloud User Manual

## 1. Introduction

ZeroCloud is a high-performance local cloud emulator designed for "Zero Latency" development. It supports S3, DynamoDB, EC2, Lambda, SQS, and IAM APIs.

## 2. API Usage

ZeroCloud exposes standard AWS-compatible APIs. You can interact with it using the AWS CLI or SDKs.

### Configuration

```bash
export AWS_ACCESS_KEY_ID=test
export AWS_SECRET_ACCESS_KEY=test
export AWS_DEFAULT_REGION=us-east-1
export ENDPOINT=http://localhost:8080
```

### Examples

**Create a Bucket:**
```bash
aws --endpoint-url $ENDPOINT s3 mb s3://my-bucket
```

**Run an Instance:**
```bash
aws --endpoint-url $ENDPOINT ec2 run-instances --image-id ami-123 --instance-type zero.micro
```

## 3. Data Persistence

By default, data is stored in `.cloudemu/data`.
- **Buckets**: Direct directories on disk.
- **Databases**: SQLite files (`metadata.db`).

To reset the state, simply delete this directory:
```bash
rm -rf .cloudemu/data
```

## 4. Workload Management

ZeroCompute workloads (VMs/Containers) are managed by the internal `zero-data-core` drivers.

- **Mock Driver**: (Default) Simulates state changes in memory.
- **Docker Driver**: (Feature Flag) Spawns real containers.
- **Hyper-V Driver**: (Windows) Spawns real VMs.

To switch drivers, modify `config/zero.toml` (if implemented) or rebuild with feature flags.
