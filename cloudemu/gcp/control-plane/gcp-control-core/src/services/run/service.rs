use gcp_data_core::storage::StorageEngine;
use gcp_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct CloudRunService {
    storage: Arc<StorageEngine>,
}

impl CloudRunService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // v1/projects/{}/locations/{}/services
        if req.path.contains("/services") && req.method == "POST" {
            return self.create_service(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_service(&self, req: &Request) -> CloudResult<Response> {
        let parts: Vec<&str> = req.path.split('/').collect();
        let p_idx = parts.iter().position(|&x| x == "projects").unwrap();
        let project = parts[p_idx + 1];
        let l_idx = parts.iter().position(|&x| x == "locations").unwrap();
        let location = parts[l_idx + 1];

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        // apiVersion: serving.knative.dev/v1
        let metadata = &body["metadata"];
        let name = metadata["name"].as_str().unwrap_or("service1");
        
        let template = &body["spec"]["template"];
        let image = template["spec"]["containers"][0]["image"].as_str().unwrap_or("gcr.io/hello-world");

        let svc = self.storage.create_run_service(name, project, location, image).map_err(|e| gcp_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "apiVersion": "serving.knative.dev/v1",
            "kind": "Service",
            "metadata": {
                "name": svc.name,
                "namespace": project,
                "selfLink": format!("/apis/serving.knative.dev/v1/namespaces/{}/services/{}", project, svc.name)
            },
            "status": {
                "url": svc.url
            }
        })))
    }
}
