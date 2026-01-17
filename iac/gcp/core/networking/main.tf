# GCP Networking (VPC)
# Mirrors CloudKit's cloudkit_core/gcp/src/vpc.rs pattern

terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
  }
}

resource "google_compute_network" "this" {
  name                    = var.network_name
  auto_create_subnetworks = var.auto_create_subnetworks
  routing_mode            = var.routing_mode
  description             = var.description
}

resource "google_compute_subnetwork" "subnets" {
  count = length(var.subnets)
  
  name          = var.subnets[count.index].name
  ip_cidr_range = var.subnets[count.index].cidr
  region        = var.subnets[count.index].region
  network       = google_compute_network.this.id
  
  private_ip_google_access = lookup(var.subnets[count.index], "private_ip_google_access", true)
  
  dynamic "secondary_ip_range" {
    for_each = lookup(var.subnets[count.index], "secondary_ip_ranges", [])
    content {
      range_name    = secondary_ip_range.value.range_name
      ip_cidr_range = secondary_ip_range.value.ip_cidr_range
    }
  }
}

resource "google_compute_firewall" "allow_internal" {
  count = var.create_internal_firewall ? 1 : 0
  
  name    = "${var.network_name}-allow-internal"
  network = google_compute_network.this.name
  
  allow {
    protocol = "icmp"
  }
  
  allow {
    protocol = "tcp"
    ports    = ["0-65535"]
  }
  
  allow {
    protocol = "udp"
    ports    = ["0-65535"]
  }
  
  source_ranges = [for s in var.subnets : s.cidr]
}

resource "google_compute_firewall" "allow_ssh" {
  count = var.create_ssh_firewall ? 1 : 0
  
  name    = "${var.network_name}-allow-ssh"
  network = google_compute_network.this.name
  
  allow {
    protocol = "tcp"
    ports    = ["22"]
  }
  
  source_ranges = var.ssh_source_ranges
}

# Outputs
output "network_name" {
  description = "Network name"
  value       = google_compute_network.this.name
}

output "network_self_link" {
  description = "Network self link"
  value       = google_compute_network.this.self_link
}

output "network_id" {
  description = "Network ID"
  value       = google_compute_network.this.id
}

output "subnet_names" {
  description = "Subnet names"
  value       = google_compute_subnetwork.subnets[*].name
}

output "subnet_self_links" {
  description = "Subnet self links"
  value       = google_compute_subnetwork.subnets[*].self_link
}

output "subnet_regions" {
  description = "Subnet regions"
  value       = google_compute_subnetwork.subnets[*].region
}
