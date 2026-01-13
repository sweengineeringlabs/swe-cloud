# Standard tagging schema for all resources
# Following CloudKit SEA architecture pattern

locals {
  # ============================================================================
  # STANDARD TAGS
  # ============================================================================
  
  standard_tags = {
    ManagedBy   = "Terraform"
    Environment = var.environment
    Provider    = var.provider
    Project     = var.project_name
    CostCenter  = var.cost_center
    Owner       = var.owner
    Architecture = "SEA"  # Stratified Encapsulation Architecture
  }

  # ============================================================================
  # COMMON TAGS (user tags + standard tags)
  # ============================================================================
  
  common_tags = var.enable_auto_tagging ? merge(
    var.tags,
    local.standard_tags
  ) : var.tags

  # ============================================================================
  # PROVIDER-SPECIFIC TAG FORMATS
  # ============================================================================
  
  # AWS tags (as-is)
  aws_tags = {
    for k, v in local.common_tags :
    k => v
  }

  # Azure tags (lowercase keys)
  azure_tags = {
    for k, v in local.common_tags :
    lower(k) => v
  }

  # GCP labels (lowercase, underscores, max 63 chars)
  gcp_labels = {
    for k, v in local.common_tags :
    replace(lower(substr(k, 0, 63)), " ", "_") => lower(substr(v, 0, 63))
  }

  # Oracle tags (as-is, but validated)
  oracle_tags = {
    for k, v in local.common_tags :
    k => v
    if can(regex("^[a-zA-Z0-9._-]+$", k))
  }

  # ============================================================================
  # RESOURCE-SPECIFIC TAGS
  # ============================================================================
  
  compute_tags = merge(
    local.common_tags,
    {
      ResourceType = "Compute"
      Service      = "VirtualMachine"
    }
  )

  storage_tags = merge(
    local.common_tags,
    {
      ResourceType = "Storage"
      Service      = "ObjectStorage"
    }
  )

  database_tags = merge(
    local.common_tags,
    {
      ResourceType = "Database"
      Service      = "RelationalDB"
    }
  )

  networking_tags = merge(
    local.common_tags,
    {
      ResourceType = "Networking"
      Service      = "VPC"
    }
  )

  # ============================================================================
  # COST ALLOCATION TAGS
  # ============================================================================
  
  cost_allocation_tags = {
    "cost:project"     = var.project_name
    "cost:environment" = var.environment
    "cost:owner"       = var.cost_center
  }

  # Combined tags for billing
  billing_tags = merge(
    local.common_tags,
    local.cost_allocation_tags
  )
}
