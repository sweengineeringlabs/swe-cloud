variable "instance_name" {
  description = "The name of the GCP instance."
  type        = string
  default     = "example-gcp-compute"
}

variable "project_id" {
  description = "The GCP project ID."
  type        = string
}

variable "zone" {
  description = "The GCP zone."
  type        = string
}

variable "machine_type" {
  description = "The GCP machine type."
  type        = string
}

variable "boot_disk_image" {
  description = "The boot disk image for the GCP instance."
  type        = string
  default     = "debian-cloud/debian-11"
}
