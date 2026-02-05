# GCP Storage (Cloud Storage)
# Mirrors CloudKit's cloudkit_core/gcp/src/gcs.rs pattern

terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
  }
}

resource "google_storage_bucket" "this" {
  name          = var.bucket_name
  location      = var.location
  storage_class = var.storage_class
  
  uniform_bucket_level_access = var.uniform_bucket_level_access
  force_destroy               = var.force_destroy
  
  versioning {
    enabled = var.versioning_enabled
  }
  
  encryption {
    default_kms_key_name = var.encryption_key_name
  }
  
  dynamic "lifecycle_rule" {
    for_each = var.lifecycle_rules
    content {
      action {
        type          = lifecycle_rule.value.action.type
        storage_class = lookup(lifecycle_rule.value.action, "storage_class", null)
      }
      
      condition {
        age                   = lookup(lifecycle_rule.value.condition, "age", null)
        num_newer_versions    = lookup(lifecycle_rule.value.condition, "num_newer_versions", null)
        with_state            = lookup(lifecycle_rule.value.condition, "with_state", null)
        matches_storage_class = lookup(lifecycle_rule.value.condition, "matches_storage_class", null)
      }
    }
  }
  
  labels = var.labels
}

# Public access prevention
resource "google_storage_bucket_iam_binding" "public_access_prevention" {
  count = var.block_public_access ? 1 : 0
  
  bucket = google_storage_bucket.this.name
  role   = "roles/storage.objectViewer"
  
  members = []  # No members = block public access
}

# Outputs
output "bucket_name" {
  description = "Bucket name"
  value       = google_storage_bucket.this.name
}

output "bucket_url" {
  description = "Bucket URL"
  value       = google_storage_bucket.this.url
}

output "bucket_self_link" {
  description = "Bucket self link"
  value       = google_storage_bucket.this.self_link
}

output "location" {
  description = "Bucket location"
  value       = google_storage_bucket.this.location
}

output "storage_class" {
  description = "Storage class"
  value       = google_storage_bucket.this.storage_class
}
