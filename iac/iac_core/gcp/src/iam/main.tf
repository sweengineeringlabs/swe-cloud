# GCP IAM (Identity and Access Management)
# Mirrors CloudKit's cloudkit_core/gcp/src/iam.rs pattern

terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
  }
}

# Service Account
resource "google_service_account" "this" {
  count = var.create_service_account ? 1 : 0
  
  account_id   = var.account_id
  display_name = var.display_name
  description  = var.description
}

# Project IAM Member (Role Binding)
resource "google_project_iam_member" "project" {
  count = length(var.project_roles)
  
  project = var.project_id
  role    = var.project_roles[count.index]
  member  = var.create_service_account ? "serviceAccount:${google_service_account.this[0].email}" : var.member
}

# Service Account Key
resource "google_service_account_key" "this" {
  count = var.create_key && var.create_service_account ? 1 : 0
  
  service_account_id = google_service_account.this[0].name
}

# Custom Role
resource "google_project_iam_custom_role" "this" {
  count = var.create_custom_role ? 1 : 0
  
  role_id     = var.role_id
  title       = var.role_title
  description = var.role_description
  permissions = var.permissions
}

# Outputs
output "service_account_email" {
  description = "The email of the service account"
  value       = var.create_service_account ? google_service_account.this[0].email : null
}

output "service_account_name" {
  description = "The fully qualified name of the service account"
  value       = var.create_service_account ? google_service_account.this[0].name : null
}

output "private_key" {
  description = "The private key in JSON format"
  value       = var.create_key ? google_service_account_key.this[0].private_key : null
  sensitive   = true
}

output "role_id" {
  description = "The ID of the custom role"
  value       = var.create_custom_role ? google_project_iam_custom_role.this[0].id : null
}
