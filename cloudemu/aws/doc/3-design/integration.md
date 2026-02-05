# AWS Integration Guide

## WHAT
Details on how CloudEmu's AWS module integrates with external tools and the CloudEmu unified server.

## WHY
To ensure seamless interoperability with standard AWS tooling and correct internal wiring.

## HOW

### 1. Unified Server Integration
The AWS module provides the `aws-control-facade` crate, which exposes an Axum router.

**Wiring (`cloudemu-server/src/main.rs`):**
```rust
use aws_control_facade::gateway::create_router;

let app = create_router(&config.data_dir).await?;
let addr = SocketAddr::from(([127, 0, 0, 1], 4566));
axum::Server::bind(&addr).serve(app.into_make_service()).await?;
```

### 2. AWS CLI Integration
The emulator implements the standard AWS Signature V4 verification (stubbed as valid) and XML/JSON responses.

**Config:**
```ini
[default]
region = us-east-1
endpoint_url = http://localhost:4566
```

### 3. Terraform Integration
Compatible with the standard `hashicorp/aws` provider.

**Main.tf:**
```hcl
provider "aws" {
  region                      = "us-east-1"
  skip_credentials_validation = true
  skip_requesting_account_id  = true
  endpoints {
    s3       = "http://localhost:4566"
    dynamodb = "http://localhost:4566"
  }
}
```

### 4. SDK Integration (Boto3)
```python
import boto3

s3 = boto3.client('s3', endpoint_url='http://localhost:4566')
```
