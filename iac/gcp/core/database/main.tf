# GCP Database (Cloud SQL)
# Mirrors CloudKit's cloudkit_core/gcp/src/cloudsql.rs pattern

terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
  }
}

resource "google_sql_database_instance" "this" {
  name             = var.instance_name
  database_version = var.database_version
  region           = var.region
  
  settings {
    tier              = var.tier
    availability_type = var.high_availability ? "REGIONAL" : "ZONAL"
    disk_size         = var.disk_size_gb
    disk_type         = var.disk_type
    disk_autoresize   = var.disk_autoresize
    
    backup_configuration {
      enabled                        = var.backup_enabled
      binary_log_enabled             = var.binary_log_enabled
      start_time                     = var.backup_start_time
      transaction_log_retention_days = var.transaction_log_retention_days
    }
    
    ip_configuration {
      ipv4_enabled    = var.public_ip_enabled
      private_network = var.private_network
      
      dynamic "authorized_networks" {
        for_each = var.authorized_networks
        content {
          name  = authorized_networks.value.name
          value = authorized_networks.value.cidr
        }
      }
    }
    
    database_flags {
      name  = "max_connections"
      value = var.max_connections
    }
  }
  
  deletion_protection = var.deletion_protection
}

resource "google_sql_database" "this" {
  name     = var.database_name
  instance = google_sql_database_instance.this.name
}

resource "google_sql_user" "this" {
  name     = var.user_name
  instance = google_sql_database_instance.this.name
  password = var.user_password
}

# Outputs
output "instance_name" {
  description = "Database instance name"
  value       = google_sql_database_instance.this.name
}

output "connection_name" {
  description = "Connection name"
  value       = google_sql_database_instance.this.connection_name
}

output "public_ip" {
  description = "Public IP address"
  value       = google_sql_database_instance.this.public_ip_address
}

output "private_ip" {
  description = "Private IP address"
  value       = google_sql_database_instance.this.private_ip_address
}

output "database_name" {
  description = "Database name"
  value       = google_sql_database.this.name
}

output "self_link" {
  description = "Self link"
  value       = google_sql_database_instance.this.self_link
}
