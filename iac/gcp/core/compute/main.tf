# GCP Compute (Instance)
# Mirrors CloudKit's cloudkit_core/gcp/src/compute.rs pattern

terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
  }
}

resource "google_compute_instance" "this" {
  name         = var.instance_name
  machine_type = var.machine_type
  zone         = var.zone
  
  boot_disk {
    initialize_params {
      image = var.boot_disk_image
      size  = var.boot_disk_size
      type  = var.boot_disk_type
    }
  }
  
  network_interface {
    network    = var.network
    subnetwork = var.subnetwork
    
    dynamic "access_config" {
      for_each = var.create_external_ip ? [1] : []
      content {
        nat_ip = null  # Ephemeral IP
      }
    }
  }
  
  metadata = merge(
    {
      ssh-keys = var.ssh_keys
    },
    var.metadata
  )
  
  metadata_startup_script = var.startup_script
  
  service_account {
    email  = var.service_account_email
    scopes = var.service_account_scopes
  }
  
  tags   = var.network_tags
  labels = var.labels
  
  allow_stopping_for_update = true
}

# Outputs
output "instance_id" {
  description = "Instance ID"
  value       = google_compute_instance.this.instance_id
}

output "instance_name" {
  description = "Instance name"
  value       = google_compute_instance.this.name
}

output "self_link" {
  description = "Instance self link"
  value       = google_compute_instance.this.self_link
}

output "public_ip" {
  description = "Public IP address"
  value       = var.create_external_ip ? google_compute_instance.this.network_interface[0].access_config[0].nat_ip : null
}

output "private_ip" {
  description = "Private IP address"
  value       = google_compute_instance.this.network_interface[0].network_ip
}

output "zone" {
  description = "Instance zone"
  value       = google_compute_instance.this.zone
}
