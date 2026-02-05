# Azure IAM (Identity and Access Management)
# Mirrors CloudKit's cloudkit_core/azure/src/iam.rs pattern

terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
}

# User Assigned Managed Identity
resource "azurerm_user_assigned_identity" "this" {
  count = var.create_identity ? 1 : 0

  name                = var.identity_name
  resource_group_name = var.resource_group_name
  location            = var.location
  
  tags = var.tags
}

# Role Assignment (at Resource Group scope)
resource "azurerm_role_assignment" "resource_group" {
  count = var.create_assignment && var.scope_type == "resource_group" ? 1 : 0

  scope                = var.scope_id
  role_definition_name = var.role_definition_name
  principal_id         = var.create_identity ? azurerm_user_assigned_identity.this[0].principal_id : var.principal_id
}

# Role Assignment (at Subscription scope)
resource "azurerm_role_assignment" "subscription" {
  count = var.create_assignment && var.scope_type == "subscription" ? 1 : 0

  scope                = var.scope_id
  role_definition_name = var.role_definition_name
  principal_id         = var.create_identity ? azurerm_user_assigned_identity.this[0].principal_id : var.principal_id
}

# Custom Role Definition
resource "azurerm_role_definition" "this" {
  count = var.create_role_definition ? 1 : 0

  name        = var.role_name
  scope       = var.scope_id
  description = var.role_description

  permissions {
    actions     = var.role_actions
    not_actions = []
  }

  assignable_scopes = [var.scope_id]
}

# Outputs
output "identity_id" {
  description = "The ID of the User Assigned Identity"
  value       = var.create_identity ? azurerm_user_assigned_identity.this[0].id : null
}

output "identity_principal_id" {
  description = "The Principal ID of the User Assigned Identity"
  value       = var.create_identity ? azurerm_user_assigned_identity.this[0].principal_id : null
}

output "identity_client_id" {
  description = "The Client ID of the User Assigned Identity"
  value       = var.create_identity ? azurerm_user_assigned_identity.this[0].client_id : null
}

output "role_definition_id" {
  description = "The Role Definition ID"
  value       = var.create_role_definition ? azurerm_role_definition.this[0].role_definition_id : null
}
