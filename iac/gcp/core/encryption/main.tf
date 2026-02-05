# GCP KMS Module

resource "google_kms_key_ring" "this" {
  project  = var.project_id
  name     = var.key_ring_name
  location = var.location
}

resource "google_kms_crypto_key" "this" {
  name     = var.key_name
  key_ring = google_kms_key_ring.this.id

  lifecycle {
    prevent_destroy = false
  }

  labels = var.labels
}

output "key_id" {
  value = google_kms_crypto_key.this.id
}

output "key_name" {
  value = google_kms_crypto_key.this.name
}
