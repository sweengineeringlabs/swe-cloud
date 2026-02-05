terraform {
  required_providers {
    azurerm = {
      source = "hashicorp/azurerm"
    }
  }
}

variable "function_name" { type = string }
variable "handler" { type = string }
variable "runtime" { type = string }
variable "filename" { type = string }
variable "environment_variables" { type = map(string) }
variable "tags" { type = map(string) }

resource "azurerm_resource_group" "this" {
  name     = "${var.function_name}-rg"
  location = "East US"
}

resource "azurerm_storage_account" "this" {
  name                     = replace(lower(var.function_name), "-", "")
  resource_group_name      = azurerm_resource_group.this.name
  location                 = azurerm_resource_group.this.location
  account_tier             = "Standard"
  account_replication_type = "LRS"
}

resource "azurerm_service_plan" "this" {
  name                = "${var.function_name}-plan"
  resource_group_name = azurerm_resource_group.this.name
  location            = azurerm_resource_group.this.location
  os_type             = "Linux"
  sku_name            = "Y1"
}

resource "azurerm_linux_function_app" "this" {
  name                = var.function_name
  resource_group_name = azurerm_resource_group.this.name
  location            = azurerm_resource_group.this.location

  storage_account_name       = azurerm_storage_account.this.name
  storage_account_access_key = azurerm_storage_account.this.primary_access_key
  service_plan_id            = azurerm_service_plan.this.id

  site_config {}
  
  app_settings = var.environment_variables

  tags = var.tags
}
