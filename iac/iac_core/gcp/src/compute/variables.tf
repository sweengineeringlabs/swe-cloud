variable "instance_name" {
  description = "Instance name"
  type        = string
}

variable "machine_type" {
  description = "Machine type (e.g., e2-medium)"
  type        = string
}

variable "zone" {
  description = "GCP zone"
  type        = string
}

variable "boot_disk_image" {
  description = "Boot disk image"
  type        = string
  default     = "ubuntu-os-cloud/ubuntu-2204-lts"
}

variable "boot_disk_size" {
  description = "Boot disk size in GB"
  type        = number
  default     = 20
}

variable "boot_disk_type" {
  description = "Boot disk type (pd-standard, pd-ssd, pd-balanced)"
  type        = string
  default     = "pd-balanced"
}

variable "network" {
  description = "Network name or self link"
  type        = string
  default     = "default"
}

variable "subnetwork" {
  description = "Subnetwork name or self link"
  type        = string
  default     = null
}

variable "create_external_ip" {
  description = "Create external IP address"
  type        = bool
  default     = true
}

variable "ssh_keys" {
  description = "SSH keys (format: user:ssh-rsa AAAAB3...)"
  type        = string
  default     = null
}

variable "metadata" {
  description = "Instance metadata"
  type        = map(string)
  default     = {}
}

variable "startup_script" {
  description = "Startup script"
  type        = string
  default     = null
}

variable "service_account_email" {
  description = "Service account email"
  type        = string
  default     = null
}

variable "service_account_scopes" {
  description = "Service account scopes"
  type        = list(string)
  default     = ["cloud-platform"]
}

variable "network_tags" {
  description = "Network tags"
  type        = list(string)
  default     = []
}

variable "labels" {
  description = "Resource labels"
  type        = map(string)
  default     = {}
}
