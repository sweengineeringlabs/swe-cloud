# Azure Storage (Blob)
# Mirrors CloudKit's cloudkit_core/azure/src/blob.rs pattern

terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
}

resource "azurerm_storage_account" "this" {
  name                     = var.storage_account_name
  resource_group_name      = var.resource_group_name
  location                 = var.location
  account_tier             = var.account_tier
  account_replication_type = var.replication_type
  account_kind             = var.account_kind
  
  # Security
  enable_https_traffic_only       = true
  min_tls_version                 = "TLS1_2"
  allow_nested_items_to_be_public = !var.block_public_access
  
  # Blob properties
  blob_properties {
    versioning_enabled = var.versioning_enabled
    
    delete_retention_policy {
      days = var.delete_retention_days
    }
    
    container_delete_retention_policy {
      days = var.container_delete_retention_days
    }
  }
  
  tags = var.tags
}

resource "azurerm_storage_container" "this" {
  count = var.create_container ? 1 : 0
  
  name                  = var.container_name
  storage_account_name  = azurerm_storage_account.this.name
  container_access_type = var.container_access_type
}

# Outputs
output "storage_account_id" {
  description = "Storage account ID"
  value       = azurerm_storage_account.this.id
}

output "storage_account_name" {
  description = "Storage account name"
  value       = azurerm_storage_account.this.name
}

output "primary_blob_endpoint" {
  description = "Primary blob endpoint"
  value       = azurerm_storage_account.this.primary_blob_endpoint
}

output "primary_access_key" {
  description = "Primary access key"
  value       = azurerm_storage_account.this.primary_access_key
  sensitive   = true
}

output "container_name" {
  description = "Container name"
  value       = var.create_container ? azurerm_storage_container.this[0].name : null
}
