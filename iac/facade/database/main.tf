# Database Facade
# Unified interface for Database resources across providers

terraform {
  required_version = ">= 1.0"
}

# ============================================================================
# IMPORT COMMON LAYER
# ============================================================================

locals {
  # Import size mappings
  db_instance_types = {
    aws = {
      small   = "db.t3.micro"
      medium  = "db.t3.medium"
      large   = "db.m5.large"
      xlarge  = "db.m5.xlarge"
    }
    azure = {
      small   = "S0"
      medium  = "S1"
      large   = "P1"
      xlarge  = "P2"
    }
    gcp = {
      small   = "db-f1-micro"
      medium  = "db-g1-small"
      large   = "db-n1-standard-1"
      xlarge  = "db-n1-standard-2"
    }
  }

  # Common tags merged with user tags
  common_tags = merge(
    var.tags,
    {
      ManagedBy    = "Terraform"
      Environment  = var.environment
      Provider     = var.provider_name
      Project      = var.project_name
      Module       = "Database-Facade"
    }
  )
}

# ============================================================================
# PROVIDER-SPECIFIC MODULE ROUTING
# ============================================================================

# AWS: RDS
module "aws_database" {
  count  = var.provider_name == "aws" ? 1 : 0
  source = "../../aws/core/database"
  
  identifier             = var.identifier
  engine                 = var.engine
  engine_version         = var.engine_version
  instance_class         = local.db_instance_types["aws"][var.instance_class]
  allocated_storage      = var.allocated_storage_gb
  
  database_name          = var.database_name
  master_username        = var.master_username
  master_password        = var.master_password
  
  # Network
  db_subnet_group_name   = lookup(var.provider_config, "subnet_group", null)
  vpc_security_group_ids = lookup(var.provider_config, "security_groups", [])
  publicly_accessible    = var.publicly_accessible
  
  # HA & Backup
  multi_az              = var.multi_az
  storage_encrypted     = var.storage_encrypted
  backup_retention_period = var.backup_retention_days
  
  tags = local.common_tags
}

# Azure: SQL Database
module "azure_database" {
  count  = var.provider_name == "azure" ? 1 : 0
  source = "../../azure/core/database"
  
  server_name         = var.identifier
  database_name       = var.database_name != null ? var.database_name : "main-db"
  
  resource_group_name = var.provider_config["resource_group_name"]
  location            = var.provider_config["location"]
  
  admin_username      = var.master_username
  admin_password      = var.master_password
  
  sku_name            = local.db_instance_types["azure"][var.instance_class]
  max_size_gb         = var.allocated_storage_gb
  zone_redundant      = var.multi_az
  
  tags = local.common_tags
}

# GCP: Cloud SQL
module "gcp_database" {
  count  = var.provider_name == "gcp" ? 1 : 0
  source = "../../gcp/core/database"
  
  instance_name    = var.identifier
  database_name    = var.database_name != null ? var.database_name : "main-db"
  
  region           = var.provider_config["region"]
  tier             = local.db_instance_types["gcp"][var.instance_class]
  
  user_name        = var.master_username
  user_password    = var.master_password
  
  disk_size_gb     = var.allocated_storage_gb
  high_availability = var.multi_az
  
  # Network
  private_network  = lookup(var.provider_config, "network_link", null)
  public_ip_enabled = var.publicly_accessible
}

# ============================================================================
# AGGREGATED OUTPUTS
# ============================================================================

locals {
  # ID
  db_id = (
    var.provider_name == "aws"   ? (length(module.aws_database) > 0 ? module.aws_database[0].db_instance_id : null) :
    var.provider_name == "azure" ? (length(module.azure_database) > 0 ? module.azure_database[0].database_id : null) :
    var.provider_name == "gcp"   ? (length(module.gcp_database) > 0 ? module.gcp_database[0].instance_name : null) :
    null
  )
  
  # Endpoint
  db_endpoint = (
    var.provider_name == "aws"   ? (length(module.aws_database) > 0 ? module.aws_database[0].db_instance_endpoint : null) :
    var.provider_name == "azure" ? (length(module.azure_database) > 0 ? module.azure_database[0].server_fqdn : null) :
    var.provider_name == "gcp"   ? (length(module.gcp_database) > 0 ? module.gcp_database[0].public_ip : null) :
    null
  )
}
