# CloudEmu

**Production-Grade Local Cloud Emulator**

CloudEmu is a local cloud services emulator that behaves like production AWS. It works with Terraform, AWS SDKs, and AWS CLI out of the box.

## Features

- 🎯 **Production-like behavior** - Accurate AWS API responses
- 🏗️ **Terraform compatible** - Deploy infrastructure locally
- 💾 **Persistent storage** - SQLite for metadata, files for objects
- 🔄 **Versioning support** - Full S3 versioning workflow
- 📋 **Bucket policies** - JSON policy storage and retrieval
- 🚀 **Fast startup** - Ready in milliseconds

## Quick Start

### 1. Start the Emulator

```bash
cargo run -p cloudemu
```

Output:
```
   _____ _                 _ ______                
  / ____| |               | |  ____|               
 | |    | | ___  _   _  __| | |__   _ __ ___  _   _ 
 | |    | |/ _ \| | | |/ _` |  __| | '_ ` _ \| | | |
 | |____| | (_) | |_| | (_| | |____| | | | | | |_| |
  \_____|_|\___/ \__,_|\__,_|______|_| |_| |_|\__,_|
                                                    
  Production-Grade Local Cloud Emulator v0.1.0

  Endpoint:    http://0.0.0.0:4566
  Data Dir:    .cloudemu
  Region:      us-east-1
```

### 2. Use with Terraform

```hcl
provider "aws" {
  endpoints {
    s3 = "http://localhost:4566"
  }
  region                      = "us-east-1"
  skip_credentials_validation = true
  skip_metadata_api_check     = true
  skip_requesting_account_id  = true
  s3_use_path_style           = true
}

resource "aws_s3_bucket" "my_bucket" {
  bucket = "my-bucket"
}

resource "aws_s3_bucket_versioning" "versioning" {
  bucket = aws_s3_bucket.my_bucket.id
  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_s3_bucket_policy" "policy" {
  bucket = aws_s3_bucket.my_bucket.id
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = "*"
      Action    = "s3:GetObject"
      Resource  = "${aws_s3_bucket.my_bucket.arn}/*"
    }]
  })
}
```

```bash
terraform init
terraform apply
```

### 3. Use with AWS CLI

```bash
export AWS_ENDPOINT_URL=http://localhost:4566
export AWS_ACCESS_KEY_ID=test
export AWS_SECRET_ACCESS_KEY=test

# Create bucket
aws s3 mb s3://my-bucket

# Upload file
aws s3 cp hello.txt s3://my-bucket/

# List objects
aws s3 ls s3://my-bucket/

# Enable versioning
aws s3api put-bucket-versioning \
  --bucket my-bucket \
  --versioning-configuration Status=Enabled

# Set bucket policy
aws s3api put-bucket-policy \
  --bucket my-bucket \
  --policy '{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":"*","Action":"s3:GetObject","Resource":"arn:aws:s3:::my-bucket/*"}]}'
```

### 4. Use with AWS SDK (Rust)

```rust
use aws_config::BehaviorVersion;

#[tokio::main]
async fn main() {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url("http://localhost:4566")
        .load()
        .await;

    let s3 = aws_sdk_s3::Client::new(&config);

    // Create bucket
    s3.create_bucket()
        .bucket("my-bucket")
        .send()
        .await
        .unwrap();

    // Enable versioning
    s3.put_bucket_versioning()
        .bucket("my-bucket")
        .versioning_configuration(
            aws_sdk_s3::types::VersioningConfiguration::builder()
                .status(aws_sdk_s3::types::BucketVersioningStatus::Enabled)
                .build()
        )
        .send()
        .await
        .unwrap();

    // Upload object
    s3.put_object()
        .bucket("my-bucket")
        .key("hello.txt")
        .body("Hello, World!".into())
        .send()
        .await
        .unwrap();
}
```

## Configuration

| Environment Variable | Default | Description |
|---------------------|---------|-------------|
| `CLOUDEMU_HOST` | `0.0.0.0` | Bind address |
| `CLOUDEMU_PORT` | `4566` | Listen port |
| `CLOUDEMU_DATA_DIR` | `.cloudemu` | Data directory |
| `CLOUDEMU_REGION` | `us-east-1` | AWS region |
| `CLOUDEMU_ACCOUNT_ID` | `000000000000` | AWS account ID |

## Supported Services ✅

CloudEmu supports the following AWS services:

- **S3** (Object Storage) - Full versioning, policies, and metadata support
- **DynamoDB** (NoSQL Database) - Key-Value storage with basic CRUD
- **SQS** (Message Queues) - Message production, consumption, and visibility timeouts
- **SNS** (Pub/Sub) - Topic management and subscriptions
- **Lambda** (Serverless) - Function management and mock invocations
- **Secrets Manager** - Secure secret storage and versioning
- **KMS** (Key Management) - Key creation, encryption/decryption, and signing
- **EventBridge** (Events) - Event buses, rules, and targets
- **CloudWatch** (Monitoring & Logs) - Metrics and log group/stream management
- **Cognito** (Identity) - User pools, groups, and basic auth
- **Step Functions** (Workflows) - State machine creation and execution tracking

### Coming Soon 🚧
- Multipart upload (S3)
- Secondary Indexes (DynamoDB)
- Dead Letter Queues (SQS)
- Real Lambda Execution (Docker/WASR)

## Data Storage

```
.cloudemu/
├── metadata.db     # SQLite database (buckets, objects, policies)
└── objects/        # Object data (content-addressed)
    ├── ab/
    │   └── cdef1234...
    └── ...
```

## License

MIT
