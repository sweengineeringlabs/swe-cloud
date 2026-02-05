# Storage Facade Module

## WHAT: Unified Object Storage

The Storage facade provides a unified interface for S3 (AWS), Blob Storage (Azure), and Cloud Storage (GCP). It handles bucket/container creation and storage class normalization.

**Prerequisites**:
- Terraform `1.0.0+`
- Configured Cloud CLI for the target provider.

## WHY: Multi-Cloud Data Portability

### Problems Solved
- **Storage Class Mappings**: Normalizing `standard`, `cold`, and `archive` across different clouds.
- **Access Control Consistency**: Abstracting provider-specific ACLs and IAM policies.

## HOW: Usage Example

```hcl
module "assets" {
  source      = "../../facade/storage"
  provider_name = "gcp"
  bucket_name = "my-public-assets-789"
  environment = "dev"
}
```

## Examples and Tests
- **Unit Tests**: See `facade/storage/storage_test.go` for Terratest plan assertions.

---

**Last Updated**: 2026-01-14
