# Azure Compute (VM)
# Mirrors CloudKit's cloudkit_core/azure/src/vm.rs pattern

terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
}

resource "azurerm_linux_virtual_machine" "this" {
  name                = var.vm_name
  resource_group_name = var.resource_group_name
  location            = var.location
  size                = var.vm_size
  
  admin_username                  = var.admin_username
  disable_password_authentication = true
  
  admin_ssh_key {
    username   = var.admin_username
    public_key = var.ssh_public_key
  }
  
  network_interface_ids = [
    azurerm_network_interface.this.id,
  ]
  
  os_disk {
    caching              = "ReadWrite"
    storage_account_type = var.os_disk_storage_type
  }
  
  source_image_reference {
    publisher = var.image_publisher
    offer     = var.image_offer
    sku       = var.image_sku
    version   = var.image_version
  }
  
  tags = var.tags
}

resource "azurerm_network_interface" "this" {
  name                = "${var.vm_name}-nic"
  location            = var.location
  resource_group_name = var.resource_group_name
  
  ip_configuration {
    name                          = "internal"
    subnet_id                     = var.subnet_id
    private_ip_address_allocation = "Dynamic"
    public_ip_address_id          = var.create_public_ip ? azurerm_public_ip.this[0].id : null
  }
  
  tags = var.tags
}

resource "azurerm_public_ip" "this" {
  count = var.create_public_ip ? 1 : 0
  
  name                = "${var.vm_name}-pip"
  location            = var.location
  resource_group_name = var.resource_group_name
  allocation_method   = "Static"
  sku                 = "Standard"
  
  tags = var.tags
}

# Outputs
output "vm_id" {
  description = "Virtual machine ID"
  value       = azurerm_linux_virtual_machine.this.id
}

output "vm_name" {
  description = "Virtual machine name"
  value       = azurerm_linux_virtual_machine.this.name
}

output "public_ip" {
  description = "Public IP address"
  value       = var.create_public_ip ? azurerm_public_ip.this[0].ip_address : null
}

output "private_ip" {
  description = "Private IP address"
  value       = azurerm_network_interface.this.private_ip_address
}

output "network_interface_id" {
  description = "Network interface ID"
  value       = azurerm_network_interface.this.id
}
