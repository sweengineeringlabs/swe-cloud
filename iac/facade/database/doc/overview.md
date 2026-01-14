# Database Facade Module

## WHAT: Managed SQL Database Provisioning

The Database facade provides a unified interface for Amazon RDS, Azure SQL Database, and GCP Cloud SQL. It handles instance creation, engine selection (PostgreSQL/MySQL), and sizing.

**Prerequisites**:
- Terraform `1.0.0+`
- Configured Cloud CLI for the target provider.

## WHY: Abstracting Managed DBA Operations

### Problems Solved
- **SKU Normalization**: Mapping generic database sizes to provider-specific SKU names (e.g., `db.t3.medium` vs `GP_Gen5_2`).
- **Engine Configuration**: Abstracting the differences in parameter groups and engine-specific settings.

## HOW: Usage Example

```hcl
module "main_db" {
  source          = "../../facade/database"
  provider        = "aws"
  identifier      = "prod-db-01"
  engine          = "postgres"
  instance_class  = "medium"
  master_password = var.db_password
}
```

## Examples and Tests
- **Unit Tests**: See `facade/database/database_test.go` for Terratest plan assertions.

---

**Last Updated**: 2026-01-14
