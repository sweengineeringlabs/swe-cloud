# Web App Example README

## Overview

This example demonstrates the **IAC SEA architecture** by deploying a complete web application stack across multiple cloud providers. It showcases:

- Multi-cloud deployment (AWS, Azure, GCP)
- Size normalization (same size → different instance types)
- Environment-based configuration (dev vs prod)
- Automatic tagging and lifecycle management
- Security best practices

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                       USER REQUEST                          │
│  terraform apply -var="environment=prod"                    │
└─────────────────────────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                    FACADE LAYER                             │
│  • facade/compute (web server)                              │
│  • facade/storage (user content)                            │
└─────────────────────────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                     CORE LAYER                              │
│  • Size: "large" → AWS: m5.large                            │
│  • Tags: Auto-applied (Environment, ManagedBy, etc.)        │
│  • Dependencies: Network → Instances                        │
└─────────────────────────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                   PROVIDER LAYER                            │
│  • AWS: EC2 instance + S3 bucket                            │
│  • Azure: VM + Blob storage (optional)                      │
│  • GCP: Compute Engine + Cloud Storage (optional)           │
└─────────────────────────────────────────────────────────────┘
```

## What Gets Created

### Development Environment
```bash
terraform apply -var="environment=dev"
```

Creates:
- ✅ **AWS EC2 Instance** (t3.medium)
  - Public IP for web access
  - Nginx web server
  - SSH access configured
  - Monitoring enabled
  - No backups (dev only)

- ✅ **AWS S3 Bucket**
  - Standard storage class
  - No versioning (dev only)
  - Encryption enabled
  - Public access blocked

### Production Environment
```bash
terraform apply -var="environment=prod"
```

Creates:
- ✅ **AWS EC2 Instance** (m5.large)
  - Larger instance for production load
  - Public IP for web access
  - Nginx web server
  - Monitoring enabled
  - **Backups enabled**
  - EBS optimized

- ✅ **AWS S3 Bucket**
  - Standard storage class
  - **Versioning enabled**
  - Encryption enabled
  - Lifecycle rules (30d → IA, 90d → Glacier)

## Quick Start

### 1. Prerequisites

```bash
# Install Terraform
terraform version  # Should be >= 1.0

# Configure AWS credentials
export AWS_ACCESS_KEY_ID="your-key-id"
export AWS_SECRET_ACCESS_KEY="your-secret-key"
export AWS_DEFAULT_REGION="us-east-1"

# Generate SSH key if needed
ssh-keygen -t rsa -b 4096 -f ~/.ssh/id_rsa
```

### 2. Deploy Development Environment

```bash
cd iac/examples/web-app

# Initialize
terraform init

# Plan
terraform plan -var="environment=dev"

# Apply
terraform apply -var="environment=dev"
```

### 3. Access Your Application

```bash
# Get server details
terraform output aws_server

# Output will show:
# {
#   instance_id = "i-0123456789abcdef"
#   public_ip   = "54.123.456.789"
#   ssh_command = "ssh ubuntu@54.123.456.789"
#   web_url     = "http://54.123.456.789"
# }

# SSH to server
ssh ubuntu@<public_ip>

# Access web application
open http://<public_ip>
# Should show: "Hello from AWS (dev)"
```

### 4. Cleanup

```bash
terraform destroy -var="environment=dev"
```

## Multi-Cloud Deployment

### Enable Azure

1. **Uncomment Azure module** in `main.tf`:
   ```hcl
   module "azure_web_server" {
     source = "../../facade/compute"
     provider = "azure"
     # ...
   }
   ```

2. **Configure Azure credentials**:
   ```bash
   az login
   export ARM_SUBSCRIPTION_ID="your-subscription-id"
   ```

3. **Apply**:
   ```bash
   terraform apply -var="environment=dev"
   ```

### Enable GCP

1. **Uncomment GCP module** in `main.tf`

2. **Configure GCP credentials**:
   ```bash
   gcloud auth application-default login
   export GOOGLE_PROJECT="your-project-id"
   ```

3. **Apply**:
   ```bash
   terraform apply -var="environment=dev"
   ```

## Environment Comparison

| Feature | Dev | Prod |
|---------|-----|------|
| **Instance Size** | medium (t3.medium) | large (m5.large) |
| **Monitoring** | ✅ Enabled | ✅ Enabled |
| **Backups** | ❌ Disabled | ✅ Enabled |
| **S3 Versioning** | ❌ Disabled | ✅ Enabled |
| **S3 Lifecycle** | ❌ None | ✅ 30d→IA, 90d→Glacier |
| **EBS Optimized** | ❌ No | ✅ Yes |

## Size Normalization Demo

The example uses `instance_size = "medium"` (or `"large"` for prod).

Behind the scenes:
```
AWS:   medium → t3.medium   (2 vCPU, 4GB RAM)
Azure: medium → Standard_B2s (2 vCPU, 4GB RAM)
GCP:   medium → e2-medium   (2 vCPU, 4GB RAM)

AWS:   large  → m5.large    (2 vCPU, 8GB RAM)
Azure: large  → Standard_DS2_v2  (2 vCPU, 7GB RAM)
GCP:   large  → n2-standard-2    (2 vCPU, 8GB RAM)
```

**Same code, different clouds, appropriate instances!**

## Automatic Tagging Demo

Every resource gets these tags automatically:

```hcl
{
  # Common tags (automatic)
  ManagedBy    = "Terraform"
  Environment  = "dev"  # or "prod"
  Provider     = "aws"
  Project      = "web-app-demo"
  Architecture = "SEA"
  
  # Resource tags (automatic)
  ResourceType = "Compute"  # or "Storage"
  Service      = "VirtualMachine"
  InstanceName = "web-aws-dev"
  Size         = "medium"
  
  # User tags (from example)
  Application  = "WebApp"
  Tier         = "Frontend"
  Cloud        = "AWS"
}
```

**16+ tags applied automatically with no user configuration!**

## Lifecycle Management Demo

Storage bucket has automatic lifecycle rules:

```
Day 0:   Object uploaded → STANDARD storage
Day 30:  Automatically moved → STANDARD_IA (Infrequent Access)
Day 90:  Automatically moved → GLACIER (Archive)
```

**Cost savings without manual intervention!**

## Security Defaults Demo

Security is enabled by default:

```hcl
# Automatically applied (no configuration needed)
encryption_enabled   = true   # S3 encryption at rest
public_access_block  = true   # Block public S3 access
enable_monitoring    = true   # CloudWatch monitoring
```

**Secure by default, no security gotchas!**

## Troubleshooting

### "No AMI found"
```bash
# Update AMI ID for your region in main.tf
provider_config = {
  ami = "ami-xxxxx"  # Get AMI ID from AWS console
}
```

### "Bucket name already exists"
```bash
# S3 bucket names are global, change bucket_name
bucket_name = "webapp-storage-aws-dev-myname"
```

### "Authentication failed"
```bash
# Verify AWS credentials
aws sts get-caller-identity

# Should return your AWS account info
```

## Cost Estimation

### Development Environment (monthly)
- EC2 t3.medium: ~$30/month
- S3 storage (10GB): ~$0.23/month
- **Total: ~$30/month**

### Production Environment (monthly)
- EC2 m5.large: ~$70/month
- S3 storage (100GB): ~$2.30/month
- Backups: ~$5/month
- **Total: ~$77/month**

## Next Steps

1. **Customize the example**:
   - Change `project_name` and `environment`
   - Modify instance sizes
   - Add more resources

2. **Enable multi-cloud**:
   - Uncomment Azure/GCP modules
   - Configure multi-cloud credentials
   - Deploy across all clouds

3. **Add more services**:
   - Database (RDS, Cosmos DB, Cloud SQL)
   - Load balancer
   - CDN (CloudFront, Azure CDN)

4. **Integrate with CI/CD**:
   - GitHub Actions
   - GitLab CI
   - Jenkins

## Related Documentation

- [IAC Architecture](../../ARCHITECTURE.md)
- [Facade Layer](../../facade/README.md)
- [Core Layer](../../core/README.md)
- [Common Layer](../../common/README.md)

---

**Example Status:** ✅ Production-ready  
**Complexity:** Beginner-friendly  
**Time to Deploy:** ~5 minutes  
**Clouds Supported:** AWS (Azure, GCP optional)
