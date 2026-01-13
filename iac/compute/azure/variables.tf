variable "instance_name" {
  description = "The name of the Azure virtual machine."
  type        = string
  default     = "example-azure-compute"
}

variable "location" {
  description = "The Azure region where the resources will be created."
  type        = string
}

variable "resource_group_name" {
  description = "The name of the resource group."
  type        = string
}

variable "vm_size" {
  description = "The size of the Azure virtual machine."
  type        = string
}

variable "admin_username" {
  description = "The admin username for the virtual machine."
  type        = string
  default     = "adminuser"
}

variable "os_image_publisher" {
  description = "The publisher of the OS image."
  type        = string
  default     = "Canonical"
}

variable "os_image_offer" {
  description = "The offer of the OS image."
  type        = string
  default     = "UbuntuServer"
}

variable "os_image_sku" {
  description = "The SKU of the OS image."
  type        = string
  default     = "18.04-LTS"
}

variable "os_image_version" {
  description = "The version of the OS image."
  type        = string
  default     = "latest"
}

variable "admin_password" {
  description = "The admin password for the virtual machine. Should be populated from a secure source."
  type        = string
  sensitive   = true
}
