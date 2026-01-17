variable "vm_name" {
  description = "Virtual machine name"
  type        = string
}

variable "resource_group_name" {
  description = "Resource group name"
  type        = string
}

variable "location" {
  description = "Azure region"
  type        = string
}

variable "vm_size" {
  description = "VM size (e.g., Standard_B2s)"
  type        = string
}

variable "admin_username" {
  description = "Admin username"
  type        = string
  default     = "azureuser"
}

variable "ssh_public_key" {
  description = "SSH public key"
  type        = string
}

variable "subnet_id" {
  description = "Subnet ID"
  type        = string
}

variable "create_public_ip" {
  description = "Create a public IP"
  type        = bool
  default     = true
}

variable "os_disk_storage_type" {
  description = "OS disk storage type"
  type        = string
  default     = "Premium_LRS"
}

variable "image_publisher" {
  description = "Image publisher"
  type        = string
  default     = "Canonical"
}

variable "image_offer" {
  description = "Image offer"
  type        = string
  default     = "0001-com-ubuntu-server-jammy"
}

variable "image_sku" {
  description = "Image SKU"
  type        = string
  default     = "22_04-lts-gen2"
}

variable "image_version" {
  description = "Image version"
  type        = string
  default     = "latest"
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
