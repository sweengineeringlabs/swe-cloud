use gcp_data_core::storage::StorageEngine;
use gcp_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct NetworkingService {
    storage: Arc<StorageEngine>,
}

impl NetworkingService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // v1/projects/{}/global/networks
        if req.path.contains("/global/networks") && req.method == "POST" {
            return self.create_network(&req);
        }
        // v1/projects/{}/regions/{}/subnetworks
        if req.path.contains("/subnetworks") && req.method == "POST" {
            return self.create_subnetwork(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_network(&self, req: &Request) -> CloudResult<Response> {
        // v1/projects/{project}/global/networks
        let parts: Vec<&str> = req.path.split('/').collect();
        let p_idx = parts.iter().position(|&x| x == "projects").unwrap();
        let project = parts[p_idx + 1];

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let name = body["name"].as_str().unwrap_or("default");
        let auto_create = body["autoCreateSubnetworks"].as_bool().unwrap_or(true);

        let net = self.storage.create_network(name, project, auto_create).map_err(|e| gcp_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "name": net.name,
            "selfLink": format!("https://www.googleapis.com/compute/v1/projects/{}/global/networks/{}", project, name),
            "autoCreateSubnetworks": net.auto_create_subnetworks,
            "creationTimestamp": net.created_at
        })))
    }

    fn create_subnetwork(&self, req: &Request) -> CloudResult<Response> {
        // v1/projects/{project}/regions/{region}/subnetworks
        let parts: Vec<&str> = req.path.split('/').collect();
        let p_idx = parts.iter().position(|&x| x == "projects").unwrap();
        let project = parts[p_idx + 1];
        let r_idx = parts.iter().position(|&x| x == "regions").unwrap();
        let region = parts[r_idx + 1];

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let name = body["name"].as_str().unwrap_or("sub1");
        let network_link = body["network"].as_str().unwrap_or("default"); // Usually a URL
        let network = network_link.split('/').last().unwrap_or("default");
        let ip_cidr = body["ipCidrRange"].as_str().unwrap_or("10.0.0.0/20");

        let subnet = self.storage.create_subnetwork(name, network, project, region, ip_cidr).map_err(|e| gcp_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "name": subnet.name,
            "selfLink": format!("https://www.googleapis.com/compute/v1/projects/{}/regions/{}/subnetworks/{}", project, region, name),
            "network": format!("https://www.googleapis.com/compute/v1/projects/{}/global/networks/{}", project, network),
            "ipCidrRange": subnet.ip_cidr_range,
            "region": format!("https://www.googleapis.com/compute/v1/projects/{}/regions/{}", project, region)
        })))
    }
}
