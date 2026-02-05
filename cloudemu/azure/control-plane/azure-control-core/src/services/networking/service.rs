use azure_data_core::storage::StorageEngine;
use azure_control_spi::{Request, Response, CloudResult, CloudError};
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
        // /subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.Network/virtualNetworks/{vnetName}
        if req.path.contains("/virtualNetworks/") && req.method == "PUT" {
            if req.path.contains("/subnets/") {
                return self.create_subnet(&req);
            } else {
                return self.create_vnet(&req);
            }
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_vnet(&self, req: &Request) -> CloudResult<Response> {
        let parts: Vec<&str> = req.path.split('/').collect();
        let rg_idx = parts.iter().position(|&x| x == "resourceGroups").unwrap();
        let resource_group = parts[rg_idx + 1];
        let vnet_idx = parts.iter().position(|&x| x == "virtualNetworks").unwrap();
        let name = parts[vnet_idx + 1];

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let location = body["location"].as_str().unwrap_or("westus");
        let cidr = body["properties"]["addressSpace"]["addressPrefixes"][0].as_str().unwrap_or("10.0.0.0/16");

        let vnet = self.storage.create_vnet(name, resource_group, location, cidr).map_err(|e| CloudError::Internal(e.to_string()))?;

        Ok(Response::ok(json!({
            "name": vnet.name,
            "id": format!("/subscriptions/sub1/resourceGroups/{}/providers/Microsoft.Network/virtualNetworks/{}", resource_group, name),
            "location": vnet.location,
            "properties": {
                "addressSpace": {
                    "addressPrefixes": [vnet.address_space]
                },
                "provisioningState": "Succeeded"
            }
        }).to_string()).with_header("Content-Type", "application/json"))
    }

    fn create_subnet(&self, req: &Request) -> CloudResult<Response> {
        // .../virtualNetworks/{vnet}/subnets/{subnet}
        let parts: Vec<&str> = req.path.split('/').collect();
        let rg_idx = parts.iter().position(|&x| x == "resourceGroups").unwrap();
        let resource_group = parts[rg_idx + 1];
        let v_idx = parts.iter().position(|&x| x == "virtualNetworks").unwrap();
        let vnet_name = parts[v_idx + 1];
        let s_idx = parts.iter().position(|&x| x == "subnets").unwrap();
        let subnet_name = parts[s_idx + 1];

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let cidr = body["properties"]["addressPrefix"].as_str().unwrap_or("10.0.0.0/24");

        let subnet = self.storage.create_subnet(subnet_name, vnet_name, resource_group, cidr).map_err(|e| CloudError::Internal(e.to_string()))?;

        Ok(Response::ok(json!({
            "name": subnet.name,
            "id": format!("/subscriptions/sub1/resourceGroups/{}/providers/Microsoft.Network/virtualNetworks/{}/subnets/{}", resource_group, vnet_name, subnet_name),
            "properties": {
                "addressPrefix": subnet.address_prefix,
                "provisioningState": "Succeeded"
            }
        }).to_string()).with_header("Content-Type", "application/json"))
    }
}
