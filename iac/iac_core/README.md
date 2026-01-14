# IAC Core Layer
# Provider-Grouped Modules (Matching CloudKit Pattern)

## Structure

Exactly mirrors CloudKit's organization:

```
iac_core/
├── aws/
│   ├── compute/       ← Like cloudkit_core/aws/ec2.rs
│   ├── storage/       ← Like cloudkit_core/aws/s3.rs
│   ├── database/      ← Like cloudkit_core/aws/dynamodb.rs
│   ├── networking/    ← Like cloudkit_core/aws/vpc.rs
│   └── iam/           ← Like cloudkit_core/aws/iam.rs
├── azure/
│   ├── compute/
│   ├── storage/
│   ├── database/
│   └── networking/
└── gcp/
    ├── compute/
    ├── storage/
    ├── database/
    └── networking/
```

## CloudKit Parallel

**CloudKit SDK:**
```
cloudkit_core/
├── aws/
│   ├── s3.rs          ← S3 service implementation
│   ├── dynamodb.rs    ← DynamoDB service implementation
│   ├── lambda.rs      ← Lambda service implementation
│   └── sqs.rs         ← SQS service implementation
```

**IAC (Perfect Match!):**
```
iac_core/
├── aws/
│   ├── storage/       ← S3 bucket implementation
│   ├── database/      ← DynamoDB table implementation
│   ├── compute/       ← Lambda & EC2 implementation
│   └── messaging/     ← SQS queue implementation
```

## Key Principle

**Group by Provider First, Then by Resource Type Within Provider**

- ✅ `iac_core/aws/compute/` - AWS-specific compute
- ✅ `iac_core/aws/storage/` - AWS-specific storage
- ❌ NOT `iac_core/compute/aws/` - Wrong! Resource type first is incorrect

## Usage

### From Orchestration Layer:

```hcl
# Use AWS compute module
module "aws_compute" {
  source = "../../iac_core/aws/compute"
  
  ami           = "ami-xxxxx"
  instance_type = "t3.medium"
  tags          = local.tags
}

# Use AWS storage module
module "aws_storage" {
  source = "../../iac_core/aws/storage"
  
  bucket_name = "my-bucket"
  tags        = local.tags
}
```

### Module Pattern:

Each resource module is self-contained:

```
iac_core/aws/compute/
├── main.tf        ← Resource definitions
├── variables.tf   ← Input variables
└── outputs.tf     ← Output values (optional)
```

## Benefits

1. **CloudKit Alignment** - Same directory structure
2. **Provider Cohesion** - All AWS resources in `aws/`
3. **Easy Navigation** - "Need AWS S3?" → `iac_core/aws/storage/`
4. **Team Ownership** - AWS team owns `aws/` directory
5. **Mental Model** - Matches CloudKit SDK thinking

## Comparison

| What | CloudKit SDK | IAC |
|------|-------------|-----|
| **AWS S3** | `cloudkit_core/aws/s3.rs` | `iac_core/aws/storage/` |
| **AWS DynamoDB** | `cloudkit_core/aws/dynamodb.rs` | `iac_core/aws/database/` |
| **AWS EC2** | `cloudkit_core/aws/ec2.rs` | `iac_core/aws/compute/` |
| **Azure Blob** | `cloudkit_core/azure/blob.rs` | `iac_core/azure/storage/` |
| **GCP GCS** | `cloudkit_core/gcp/gcs.rs` | `iac_core/gcp/storage/` |

**Perfect 1:1 mapping!** 🎯

---

**Organization:** Provider-first, then resource type  
**Matches:** CloudKit SDK exactly  
**Benefit:** Same mental model across codebase
