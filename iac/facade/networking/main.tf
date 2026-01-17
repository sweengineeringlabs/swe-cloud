# Networking Facade
# Unified interface for Network resources across providers

terraform {
  required_version = ">= 1.0"
}

# ============================================================================
# IMPORT COMMON LAYER
# ============================================================================

locals {
  common_tags = merge(
    var.tags,
    {
      ManagedBy    = "Terraform"
      Environment  = var.environment
      Provider     = var.provider_name
      Project      = var.project_name
      Module       = "Networking-Facade"
    }
  )
}

# ============================================================================
# PROVIDER-SPECIFIC MODULE ROUTING
# ============================================================================

# AWS: VPC
module "aws_networking" {
  count  = var.provider_name == "aws" ? 1 : 0
  source = "../../aws/core/networking"
  
  vpc_name            = var.network_name
  vpc_cidr            = var.metrics.cidr
  availability_zones  = var.metrics.azs
  
  public_subnet_cidrs  = var.metrics.public_subnets
  private_subnet_cidrs = var.metrics.private_subnets
  
  create_internet_gateway = var.internet_access
  create_default_security_group = true
  
  tags = local.common_tags
}

# Azure: VNet
module "azure_networking" {
  count  = var.provider_name == "azure" ? 1 : 0
  source = "../../azure/core/networking"
  
  vnet_name           = var.network_name
  resource_group_name = try(var.provider_config.resource_group_name, "default-rg")
  location            = try(var.provider_config.location, "eastus")
  
  address_space       = var.metrics.cidr
  
  # Map generic subnets to Azure format
  public_subnets = [
    for i, cidr in var.metrics.public_subnets : {
      name           = "${var.network_name}-public-${i}"
      address_prefix = cidr
    }
  ]
  
  private_subnets = [
    for i, cidr in var.metrics.private_subnets : {
      name           = "${var.network_name}-private-${i}"
      address_prefix = cidr
    }
  ]
  
  create_default_nsg = true
  tags               = local.common_tags
}

# GCP: VPC
module "gcp_networking" {
  count  = var.provider_name == "gcp" ? 1 : 0
  source = "../../gcp/core/networking"
  
  network_name = var.network_name
  routing_mode = "GLOBAL"
  
  # Map generic subnets to GCP format
  subnets = concat(
    [
      for i, cidr in var.metrics.public_subnets : {
        name   = "${var.network_name}-public-${i}"
        cidr   = cidr
        region = try(var.provider_config.region, "us-central1")
      }
    ],
    [
      for i, cidr in var.metrics.private_subnets : {
        name                     = "${var.network_name}-private-${i}"
        cidr                     = cidr
        region                   = try(var.provider_config.region, "us-central1")
        private_ip_google_access = true
      }
    ]
  )
  
  create_internal_firewall = true
  create_ssh_firewall      = true
}

# ============================================================================
# AGGREGATED OUTPUTS
# ============================================================================

locals {
  network_id = (
    var.provider_name == "aws"   ? (length(module.aws_networking) > 0 ? module.aws_networking[0].vpc_id : null) :
    var.provider_name == "azure" ? (length(module.azure_networking) > 0 ? module.azure_networking[0].vnet_id : null) :
    var.provider_name == "gcp"   ? (length(module.gcp_networking) > 0 ? module.gcp_networking[0].network_id : null) :
    null
  )
}
