# Azure Provider Configuration
# SPI Layer for Azure

provider "azurerm" {
  features {}
  
  subscription_id = var.subscription_id
  tenant_id       = var.tenant_id
  client_id       = var.client_id
  client_secret   = var.client_secret
}

# Standard tags for all resources in this stack
locals {
  spi_tags = {
    ManagedBy = "Terraform"
    Stack     = var.stack_name
  }
}
