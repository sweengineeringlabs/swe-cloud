use azure_control_spi::{Request, Response, CloudResult, CloudError};

use azure_data_core::storage::StorageEngine;
use std::sync::Arc;

/// Azure Service Bus Handler
pub struct ServiceBusService {
    engine: Arc<StorageEngine>,
}

impl ServiceBusService {
    pub fn new(engine: Arc<StorageEngine>) -> Self {
        Self { engine }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // Simple routing: /{queue}/messages
        
        let path = req.path.trim_start_matches('/');
        let parts: Vec<&str> = path.split('/').collect();

        if parts.len() < 2 {
             return Err(CloudError::Validation("Invalid Service Bus path".into()));
        }

        let queue_name = parts[0];
        let resource = parts[1];

        if resource == "messages" {
            match req.method.as_str() {
                "POST" => return self.send_message(queue_name, &req.body).await,
                "DELETE" if req.method == "DELETE" => return self.receive_message(queue_name).await, 
                 // Note: Receive is technically DELETE on /messages/head usually
                _ => {}
            }
        }
        
        // Handle /messages/head
        if parts.len() > 2 && parts[2] == "head" {
             if req.method == "POST" || req.method == "DELETE" {
                 return self.receive_message(queue_name).await;
             }
        }

        // Management: Create Queue (PUT /{queue} or PUT /queue/{name})
        if parts.len() == 1 && req.method == "PUT" {
            return self.create_queue(queue_name).await;
        }
        if parts.len() == 2 && parts[0] == "queue" && req.method == "PUT" {
            return self.create_queue(parts[1]).await;
        }

        Err(CloudError::Validation(format!("Unsupported Service Bus operation: {}", req.path)))
    }

    async fn create_queue(&self, name: &str) -> CloudResult<Response> {
        let url = format!("https://servicebus.windows.net/{}", name);
        let arn = format!("arn:azure:servicebus:local:azure:queue/{}", name);
        
        self.engine.create_queue(name, &url, &arn)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        Ok(Response::created("").with_header("Content-Type", "application/xml"))
    }

    async fn send_message(&self, queue: &str, body: &[u8]) -> CloudResult<Response> {
        let message_body = String::from_utf8(body.to_vec()).unwrap_or_default();
        
        let msg_id = self.engine.send_message(queue, &message_body)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        Ok(Response::created(format!(r#"{{"messageId":"{}"}}"#, msg_id)))
    }

    async fn receive_message(&self, queue: &str) -> CloudResult<Response> {
        let messages = self.engine.receive_message(queue, 1)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        if messages.is_empty() {
            Ok(Response::no_content())
        } else {
            let msg = &messages[0];
            Ok(Response::ok(format!(r#"{{"messageId":"{}","body":"{}"}}"#, msg.id, msg.body)))
        }
    }
}
