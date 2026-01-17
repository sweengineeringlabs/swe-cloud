# GCP Secret Manager

resource "google_secret_manager_secret" "this" {
  project   = var.project_id
  secret_id = var.secret_id

  replication {
    auto {}
  }

  labels = var.labels
}

resource "google_secret_manager_secret_version" "this" {
  secret      = google_secret_manager_secret.this.id
  secret_data = var.secret_data
}

output "secret_id" {
  value = google_secret_manager_secret.this.id
}

output "secret_name" {
  value = google_secret_manager_secret.this.name
}
