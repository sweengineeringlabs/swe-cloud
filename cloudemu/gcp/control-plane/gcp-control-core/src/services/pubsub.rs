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
            let project_id = parts[2];
            
            if parts[3] == "topics" {
                if parts.len() == 4 {
                    // List topics
                    return self.list_topics(project_id).await;
                } else {
                    let topic = parts[4];
                    if parts.len() == 5 {
                        match req.method.as_str() {
                            "PUT" => return self.create_topic(project_id, topic).await,
                            "GET" => return self.get_topic(project_id, topic).await,
                            "DELETE" => return self.delete_topic(topic).await,
                            _ => {}
                        }
                    } else if parts.len() == 6 && parts[5] == "publish" {
                        return self.publish_message(project_id, topic, &req.body).await;
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

    async fn create_topic(&self, project_id: &str, name: &str) -> CloudResult<Response> {
        self.engine.create_pubsub_topic(name, project_id)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        Ok(Response::created(format!(r#"{{"name":"projects/{}/topics/{}"}}"#, project_id, name)))
    }

    async fn get_topic(&self, project_id: &str, name: &str) -> CloudResult<Response> {
        self.engine.get_pubsub_topic(name)
            .map_err(|e| CloudError::NotFound { resource_type: "Topic".into(), resource_id: name.into() })?;
            
        Ok(Response::ok(format!(r#"{{"name":"projects/{}/topics/{}"}}"#, project_id, name)))
    }

    async fn delete_topic(&self, name: &str) -> CloudResult<Response> {
        self.engine.delete_pubsub_topic(name)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
        Ok(Response::no_content())
    }

    async fn list_topics(&self, project_id: &str) -> CloudResult<Response> {
        let topics = self.engine.list_pubsub_topics(project_id)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
        Ok(Response::ok(format!(r#"{{"topics":{:?}}}"#, topics)))
    }

    async fn publish_message(&self, project_id: &str, topic: &str, _body: &[u8]) -> CloudResult<Response> {
        // Verify topic exists
        self.engine.get_pubsub_topic(topic)
            .map_err(|e| CloudError::NotFound { resource_type: "Topic".into(), resource_id: topic.into() })?;
        
        let msg_id = uuid::Uuid::new_v4().to_string();
        Ok(Response::ok(format!(r#"{{"messageIds":["{}"]}}"#, msg_id)))
    }

    async fn pull_messages(&self, _subscription: &str) -> CloudResult<Response> {
        // For simplicity, return empty for now
        Ok(Response::ok(r#"{"receivedMessages":[]}"#))
    }
}
