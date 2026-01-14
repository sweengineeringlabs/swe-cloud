# Azure Networking (VNet)
# Mirrors CloudKit's cloudkit_core/azure/src/vnet.rs pattern

terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
}

resource "azurerm_virtual_network" "this" {
  name                = var.vnet_name
  resource_group_name = var.resource_group_name
  location            = var.location
  address_space       = [var.address_space]
  
  tags = var.tags
}

resource "azurerm_subnet" "public" {
  count = length(var.public_subnets)
  
  name                 = var.public_subnets[count.index].name
  resource_group_name  = var.resource_group_name
  virtual_network_name = azurerm_virtual_network.this.name
  address_prefixes     = [var.public_subnets[count.index].address_prefix]
}

resource "azurerm_subnet" "private" {
  count = length(var.private_subnets)
  
  name                 = var.private_subnets[count.index].name
  resource_group_name  = var.resource_group_name
  virtual_network_name = azurerm_virtual_network.this.name
  address_prefixes     = [var.private_subnets[count.index].address_prefix]
}

resource "azurerm_network_security_group" "default" {
  count = var.create_default_nsg ? 1 : 0
  
  name                = "${var.vnet_name}-default-nsg"
  location            = var.location
  resource_group_name = var.resource_group_name
  
  security_rule {
    name                       = "AllowVnetInbound"
    priority                   = 100
    direction                  = "Inbound"
    access                     = "Allow"
    protocol                   = "*"
    source_port_range          = "*"
    destination_port_range     = "*"
    source_address_prefix      = "VirtualNetwork"
    destination_address_prefix = "VirtualNetwork"
  }
  
  tags = var.tags
}

# Outputs
output "vnet_id" {
  description = "Virtual network ID"
  value       = azurerm_virtual_network.this.id
}

output "vnet_name" {
  description = "Virtual network name"
  value       = azurerm_virtual_network.this.name
}

output "address_space" {
  description = "Address space"
  value       = azurerm_virtual_network.this.address_space
}

output "public_subnet_ids" {
  description = "Public subnet IDs"
  value       = azurerm_subnet.public[*].id
}

output "private_subnet_ids" {
  description = "Private subnet IDs"
  value       = azurerm_subnet.private[*].id
}

output "default_nsg_id" {
  description = "Default network security group ID"
  value       = var.create_default_nsg ? azurerm_network_security_group.default[0].id : null
}
