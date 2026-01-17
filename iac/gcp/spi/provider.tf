# GCP Provider Configuration
# SPI Layer for GCP

provider "google" {
  project = var.project_id
  region  = var.region
  zone    = var.zone
}

provider "google-beta" {
  project = var.project_id
  region  = var.region
  zone    = var.zone
}

# Standard labels for all resources
locals {
  spi_labels = {
    managed_by = "terraform"
    stack      = var.stack_name
  }
}
