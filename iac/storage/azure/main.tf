terraform {
  required_providers {
    # azurerm = {
    #   source  = "hashicorp/azurerm"
    #   version = "~> 3.0"
    # }
  }
}

# Placeholder resource for Azure Storage (e.g., Storage Account)
# resource "azurerm_resource_group" "example" {
#   name     = "example-storage-resources"
#   location = "East US"
# }

# resource "azurerm_storage_account" "example" {
#   name                     = "examplestorageaccount" # Must be unique across Azure
#   resource_group_name      = azurerm_resource_group.example.name
#   location                 = azurerm_resource_group.example.location
#   account_tier             = "Standard"
#   account_replication_type = "GRS"
# }
