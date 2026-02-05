# Backend Configuration
# Uses Google Cloud Storage (GCS) for state

terraform {
  backend "gcs" {
    # bucket  = "tf-state-bucket"
    # prefix  = "terraform/state"
  }
}
