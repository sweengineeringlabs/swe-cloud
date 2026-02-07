use azure_data_core::storage::StorageEngine;
use azure_control_spi::{Request, Response, CloudResult, CloudError};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct EventGridService {
    storage: Arc<StorageEngine>,
}

impl EventGridService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // PUT /subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.EventGrid/topics/{topicName}
        if req.path.contains("/providers/Microsoft.EventGrid/topics/") && req.method == "PUT" {
            return self.create_topic(&req);
        }
         // POST /subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.EventGrid/topics/{topicName}/eventSubscriptions/{subName}
        if req.path.contains("/eventSubscriptions/") && req.method == "PUT" {
            return self.create_subscription(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_topic(&self, req: &Request) -> CloudResult<Response> {
        let parts: Vec<&str> = req.path.split('/').collect();
        let rg_idx = parts.iter().position(|&x| x == "resourceGroups").unwrap();
        let resource_group = parts[rg_idx + 1];
        let t_idx = parts.iter().position(|&x| x == "topics").unwrap();
        let name = parts[t_idx + 1];

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let location = body["location"].as_str().unwrap_or("westus");

        let topic = self.storage.create_eventgrid_topic(name, location, resource_group).map_err(|e| CloudError::Internal(e.to_string()))?;

        Ok(Response::ok(json!({
            "name": topic.name,
            "type": "Microsoft.EventGrid/topics",
            "location": topic.location,
            "properties": {
                "endpoint": topic.endpoint,
                "provisioningState": "Succeeded"
            }
        }).to_string()).with_header("Content-Type", "application/json"))
    }

    fn create_subscription(&self, req: &Request) -> CloudResult<Response> {
        // .../topics/{topicName}/eventSubscriptions/{subName}
        let parts: Vec<&str> = req.path.split('/').collect();
        let t_idx = parts.iter().position(|&x| x == "topics").unwrap();
        let topic_name = parts[t_idx + 1];
        let s_idx = parts.iter().position(|&x| x == "eventSubscriptions").unwrap();
        let sub_name = parts[s_idx + 1];

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        // properties.destination.endpointUrl
        let endpoint = body["properties"]["destination"]["endpointUrl"].as_str().unwrap_or("");
        
        // This is a simplified implementation
        let sub = self.storage.create_eventgrid_subscription(topic_name, sub_name, endpoint, "WebHook").map_err(|e| CloudError::Internal(e.to_string()))?;

        Ok(Response::ok(json!({
            "name": sub.name,
            "type": "Microsoft.EventGrid/eventSubscriptions",
            "properties": {
                "topic": sub.topic_name,
                "provisioningState": "Succeeded",
                "destination": {
                    "endpointUrl": sub.endpoint
                }
            }
        }).to_string()).with_header("Content-Type", "application/json"))
    }
}
