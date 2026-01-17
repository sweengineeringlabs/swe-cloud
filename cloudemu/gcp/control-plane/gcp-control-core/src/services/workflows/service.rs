use gcp_data_core::storage::StorageEngine;
use gcp_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct WorkflowsService {
    storage: Arc<StorageEngine>,
}

impl WorkflowsService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // v1/projects/{}/locations/{}/workflows
        if req.path.contains("/workflows") && req.method == "POST" {
            return self.create_workflow(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_workflow(&self, req: &Request) -> CloudResult<Response> {
        // .../projects/{project}/locations/{location}/workflows
        let args: Vec<&str> = req.path.split('/').collect();
        // find "projects"
        let p_idx = args.iter().position(|&x| x == "projects").unwrap();
        let project = args[p_idx + 1];
        let l_idx = args.iter().position(|&x| x == "locations").unwrap();
        let location = args[l_idx + 1];

        // Query param ?workflowId=...
        // For simplicity assume it's in body or we generate/mock it
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let name_suffix = body["name"].as_str().unwrap_or("wf1"); // usually full name provided?
        // API: POST .../workflows?workflowId=my-workflow
        // Let's assume name is in body or random.
        
        let desc = body["description"].as_str().unwrap_or("");
        
        // Full name structure: projects/{}/locations/{}/workflows/{}
        // We'll just store the suffix
        
        let workflow = self.storage.create_workflow(name_suffix, project, location, desc).map_err(|e| gcp_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
             "name": format!("projects/{}/locations/{}/workflows/{}", workflow.project_id, workflow.region, workflow.name),
             "description": workflow.description,
             "state": workflow.state,
             "createTime": workflow.created_at
        })))
    }
}
