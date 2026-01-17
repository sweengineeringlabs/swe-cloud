terraform {
  required_providers {
    azurerm = {
      source = "hashicorp/azurerm"
    }
  }
}

variable "queue_name" { type = string }
variable "topic_name" { type = string }
variable "create_queue" { type = bool }
variable "create_topic" { type = bool }
variable "tags" { type = map(string) }

# Service Bus Namespace
resource "azurerm_servicebus_namespace" "this" {
  count               = var.create_queue || var.create_topic ? 1 : 0
  name                = "${var.queue_name != null ? var.queue_name : var.topic_name}-ns"
  location            = "East US"
  resource_group_name = "azure-test-rg" # In real implementation, this comes from a data source or input
  sku                 = "Standard"
  tags                = var.tags
}

# Service Bus Queue
resource "azurerm_servicebus_queue" "this" {
  count        = var.create_queue ? 1 : 0
  name         = var.queue_name
  namespace_id = azurerm_servicebus_namespace.this[0].id
}

# Service Bus Topic
resource "azurerm_servicebus_topic" "this" {
  count        = var.create_topic ? 1 : 0
  name         = var.topic_name
  namespace_id = azurerm_servicebus_namespace.this[0].id
}
