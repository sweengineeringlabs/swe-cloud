# Backend Configuration
# Uses Azure Blob Storage for state

terraform {
  backend "azurerm" {
    # These should be passed via -backend-config or environment variables
    # resource_group_name  = "tfstate-rg"
    # storage_account_name = "tfstate"
    # container_name       = "tfstate"
    # key                  = "prod.terraform.tfstate"
  }
}
