use gcp_control_spi::{Request, Response, CloudResult, CloudError};
use gcp_data_core::storage::StorageEngine;
use std::sync::Arc;

/// GCP Pub/Sub Handler
pub struct PubSubService {
    engine: Arc<StorageEngine>,
}

impl PubSubService {
    pub fn new(engine: Arc<StorageEngine>) -> Self {
        Self { engine }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        let path = req.path.trim_start_matches('/');
        let parts: Vec<&str> = path.split('/').collect();

        if parts.is_empty() {
            return Ok(Response::ok("Pub/Sub Emulator"));
        }

        // Pub/Sub paths: /v1/projects/{project}/topics/{topic}
        // or /v1/projects/{project}/subscriptions/{subscription}
        if parts.len() >= 4 && parts[0] == "v1" && parts[1] == "projects" {
            if parts[3] == "topics" {
                if parts.len() == 4 {
                    // List topics
                    return self.list_topics().await;
                } else {
                    let topic = parts[4];
                    if parts.len() == 5 {
                        match req.method.as_str() {
                            "PUT" => return self.create_topic(topic).await,
                            "GET" => return self.get_topic(topic).await,
                            "DELETE" => return self.delete_topic(topic).await,
                            _ => {}
                        }
                    } else if parts.len() == 6 && parts[5] == "publish" {
                        return self.publish_message(topic, &req.body).await;
                    }
                }
            } else if parts[3] == "subscriptions" {
                if parts.len() > 4 {
                    let subscription = parts[4];
                    if parts.len() == 6 && parts[5] == "pull" {
                        return self.pull_messages(subscription).await;
                    }
                }
            }
        }

        Err(CloudError::Validation(format!("Unsupported Pub/Sub operation: {} {}", req.method, req.path)))
    }

    async fn create_topic(&self, name: &str) -> CloudResult<Response> {
        self.engine.create_topic(name, "gcp", "local")
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        Ok(Response::created(format!(r#"{{"name":"{}"}}"#, name)))
    }

    async fn get_topic(&self, name: &str) -> CloudResult<Response> {
        // Verify exists by listing
        let topics = self.engine.list_topics().unwrap_or_default();
        let _found = topics.iter().find(|t| t.name == name)
            .ok_or_else(|| CloudError::NotFound { resource_type: "Topic".into(), resource_id: name.into() })?;
            
        Ok(Response::ok(format!(r#"{{"name":"{}"}}"#, name)))
    }

    async fn delete_topic(&self, _name: &str) -> CloudResult<Response> {
        // Engine doesn't have delete_topic, just return success
        Ok(Response::no_content())
    }

    async fn list_topics(&self) -> CloudResult<Response> {
        let topics = self.engine.list_topics()
            .unwrap_or_default();
        Ok(Response::ok(format!(r#"{{"topics":{:?}}}"#, topics)))
    }

    async fn publish_message(&self, topic: &str, body: &[u8]) -> CloudResult<Response> {
        // SNS doesn't have direct publish, we'll just verify topic exists
        let topics = self.engine.list_topics().unwrap_or_default();
        let _found = topics.iter().find(|t| t.name == topic)
            .ok_or_else(|| CloudError::NotFound { resource_type: "Topic".into(), resource_id: topic.into() })?;
        
        let msg_id = uuid::Uuid::new_v4().to_string();
        Ok(Response::ok(format!(r#"{{"messageIds":["{}"]}}"#, msg_id)))
    }

    async fn pull_messages(&self, _subscription: &str) -> CloudResult<Response> {
        // For simplicity, return empty for now
        Ok(Response::ok(r#"{"receivedMessages":[]}"#))
    }
}
