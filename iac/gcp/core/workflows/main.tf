# GCP Workflows

resource "google_workflows_workflow" "this" {
  name            = var.name
  project         = var.project_id
  region          = var.region
  source_contents = var.source_contents
  
  labels = var.labels
}

output "workflow_id" {
  value = google_workflows_workflow.this.id
}

output "workflow_name" {
  value = google_workflows_workflow.this.name
}
