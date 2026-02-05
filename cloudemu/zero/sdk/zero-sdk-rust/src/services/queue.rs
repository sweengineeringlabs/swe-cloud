use crate::{ClientInner, ZeroSdkError, common::request};
use std::sync::Arc;
use serde_json::json;

pub struct QueueClient {
    inner: Arc<ClientInner>,
}

pub struct Message {
    pub id: String,
    pub body: String,
    pub receipt_handle: String,
}

impl QueueClient {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn create_queue(&self, name: &str) -> Result<String, ZeroSdkError> {
        let resp = request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::POST,
            "/queue/queues",
            Some(json!({ "name": name })),
        ).await?;
        
        resp["QueueUrl"].as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| ZeroSdkError::Internal("Missing QueueUrl".into()))
    }

    pub async fn send_message(&self, queue_name: &str, body: &str) -> Result<String, ZeroSdkError> {
        let resp = request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::POST,
            &format!("/queue/queues/{}/messages", queue_name),
            Some(json!({ "body": body })),
        ).await?;
        
        resp["MessageId"].as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| ZeroSdkError::Internal("Missing MessageId".into()))
    }

    pub async fn receive_message(&self, queue_name: &str) -> Result<Option<Message>, ZeroSdkError> {
        let resp = request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::GET,
            &format!("/queue/queues/{}/messages", queue_name),
            None,
        ).await?;
        
        let messages = resp["Messages"].as_object();
        if let Some(msg) = messages {
             Ok(Some(Message {
                 id: msg["MessageId"].as_str().unwrap_or_default().to_string(),
                 body: msg["Body"].as_str().unwrap_or_default().to_string(),
                 receipt_handle: msg["ReceiptHandle"].as_str().unwrap_or_default().to_string(),
             }))
        } else {
             Ok(None)
        }
    }

    pub async fn delete_message(&self, queue_name: &str, receipt_handle: &str) -> Result<(), ZeroSdkError> {
        request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::DELETE,
            &format!("/queue/queues/{}/messages/{}", queue_name, receipt_handle),
            None,
        ).await?;
        Ok(())
    }

    pub async fn list_queues(&self) -> Result<Vec<String>, ZeroSdkError> {
        let resp = request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::GET,
            "/queue/queues",
            None,
        ).await?;
        
        let urls = resp["QueueUrls"].as_array().ok_or_else(|| {
            ZeroSdkError::Internal("Invalid response format: missing QueueUrls field".to_string())
        })?;
        
        Ok(urls.iter().map(|v| v.as_str().unwrap_or_default().to_string()).collect())
    }
}
