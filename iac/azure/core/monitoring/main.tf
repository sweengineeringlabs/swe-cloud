# Azure Monitor
# Mirrors CloudKit's cloudkit_core/azure/src/monitor.rs pattern

terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
}

resource "azurerm_monitor_action_group" "this" {
  count = var.create_action_group ? 1 : 0
  
  name                = var.action_group_name
  resource_group_name = var.resource_group_name
  short_name          = var.short_name
  
  dynamic "email_receiver" {
    for_each = var.email_receivers
    content {
      name          = email_receiver.value.name
      email_address = email_receiver.value.email
    }
  }
}

resource "azurerm_monitor_metric_alert" "this" {
  count = var.create_alert ? 1 : 0
  
  name                = var.alert_name
  resource_group_name = var.resource_group_name
  scopes              = var.scopes
  description         = var.description
  
  criteria {
    metric_namespace = var.metric_namespace
    metric_name      = var.metric_name
    aggregation      = var.aggregation
    operator         = var.operator
    threshold        = var.threshold
  }
  
  action {
    action_group_id = var.action_group_id
  }
  
  tags = var.tags
}

# Log Analytics Workspace
resource "azurerm_log_analytics_workspace" "this" {
  count = var.create_workspace ? 1 : 0
  
  name                = var.workspace_name
  location            = var.location
  resource_group_name = var.resource_group_name
  sku                 = var.sku
  retention_in_days   = var.retention_in_days
  
  tags = var.tags
}

output "workspace_id" {
  value = var.create_workspace ? azurerm_log_analytics_workspace.this[0].id : null
}
