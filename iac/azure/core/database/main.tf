# Azure Database (SQL Database)
# Mirrors CloudKit's cloudkit_core/azure/src/cosmos.rs pattern

terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
}

resource "azurerm_mssql_server" "this" {
  name                         = var.server_name
  resource_group_name          = var.resource_group_name
  location                     = var.location
  version                      = var.server_version
  administrator_login          = var.admin_username
  administrator_login_password = var.admin_password
  
  minimum_tls_version = "1.2"
  
  tags = var.tags
}

resource "azurerm_mssql_database" "this" {
  name      = var.database_name
  server_id = azurerm_mssql_server.this.id
  
  sku_name                    = var.sku_name
  max_size_gb                 = var.max_size_gb
  zone_redundant              = var.zone_redundant
  storage_account_type        = var.storage_account_type
  
  tags = var.tags
}

# Firewall rule for Azure services
resource "azurerm_mssql_firewall_rule" "azure_services" {
  name             = "AllowAzureServices"
  server_id        = azurerm_mssql_server.this.id
  start_ip_address = "0.0.0.0"
  end_ip_address   = "0.0.0.0"
}

# Optional: Public IP firewall rules
resource "azurerm_mssql_firewall_rule" "public" {
  count = length(var.allowed_ip_ranges)
  
  name             = "AllowIP-${count.index}"
  server_id        = azurerm_mssql_server.this.id
  start_ip_address = var.allowed_ip_ranges[count.index].start
  end_ip_address   = var.allowed_ip_ranges[count.index].end
}

# Outputs
output "server_id" {
  description = "SQL Server ID"
  value       = azurerm_mssql_server.this.id
}

output "server_fqdn" {
  description = "Fully qualified domain name"
  value       = azurerm_mssql_server.this.fully_qualified_domain_name
}

output "database_id" {
  description = "Database ID"
  value       = azurerm_mssql_database.this.id
}

output "database_name" {
  description = "Database name"
  value       = azurerm_mssql_database.this.name
}

output "connection_string" {
  description = "Database connection string"
  value       = "Server=tcp:${azurerm_mssql_server.this.fully_qualified_domain_name},1433;Initial Catalog=${azurerm_mssql_database.this.name};Persist Security Info=False;User ID=${var.admin_username};MultipleActiveResultSets=False;Encrypt=True;TrustServerCertificate=False;Connection Timeout=30;"
  sensitive   = true
}
