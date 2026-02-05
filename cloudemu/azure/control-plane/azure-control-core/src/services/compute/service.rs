use azure_data_core::storage::StorageEngine;
use azure_control_spi::{Request, Response, CloudResult, CloudError};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct ComputeService {
    storage: Arc<StorageEngine>,
}

impl ComputeService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        let path_parts: Vec<&str> = req.path.split('/').collect();
        // PUT /subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.Compute/virtualMachines/{vm}
        if req.method == "PUT" && req.path.contains("virtualMachines") {
            return self.create_vm(&req, path_parts);
        }
        // GET /subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.Compute/virtualMachines
        if req.method == "GET" && req.path.contains("virtualMachines") {
            let rg = path_parts.iter().position(|&x| x == "resourceGroups").map(|i| path_parts[i+1]);
            return self.list_vms(rg);
        }

        Ok(Response::not_found("Not Found"))
    }

    fn create_vm(&self, req: &Request, parts: Vec<&str>) -> CloudResult<Response> {
        let name_idx = parts.iter().position(|&x| x == "virtualMachines").unwrap() + 1;
        let name = parts[name_idx];
        let rg_idx = parts.iter().position(|&x| x == "resourceGroups").unwrap() + 1;
        let rg = parts[rg_idx];

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let location = body["location"].as_str().unwrap_or("eastus");
        let size = body["properties"]["hardwareProfile"]["vmSize"].as_str().unwrap_or("Standard_DS1_v2");
        let os = body["properties"]["storageProfile"]["osDisk"]["osType"].as_str().unwrap_or("Linux");
        let admin = body["properties"]["osProfile"]["adminUsername"].as_str().unwrap_or("azureuser");

        let _metadata = self.storage.create_virtual_machine(
            name, 
            location, 
            rg, 
            size, 
            os, 
            admin
        ).map_err(|e| CloudError::Internal(e.to_string()))?;

        let resp_json = json!({
            "id": format!("/subscriptions/sub-1/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}", rg, name),
            "name": name,
            "type": "Microsoft.Compute/virtualMachines",
            "location": location,
            "properties": {
                "provisioningState": "Succeeded"
            }
        });

        Ok(Response::ok(resp_json.to_string()).with_header("Content-Type", "application/json"))
    }

    fn list_vms(&self, rg: Option<&str>) -> CloudResult<Response> {
        let vms = self.storage.list_vms(rg).map_err(|e| CloudError::Internal(e.to_string()))?;
        
        let value: Vec<Value> = vms.iter().map(|vm| json!({
            "name": vm.name,
            "location": vm.location,
            "properties": {
                "provisioningState": vm.status
            }
        })).collect();

        Ok(Response::ok(json!({ "value": value }).to_string()).with_header("Content-Type", "application/json"))
    }
}
