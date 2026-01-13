terraform {
  required_providers {
    # azurerm = {
    #   source  = "hashicorp/azurerm"
    #   version = "~> 3.0"
    # }
  }
}

# Placeholder resource for Azure IAM (e.g., Service Principal)
# resource "azuread_application" "example" {
#   display_name = "example-azure-app"
# }

# resource "azuread_service_principal" "example" {
#   application_id = azuread_application.example.application_id
# }
