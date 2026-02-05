use async_trait::async_trait;
use cloudkit_api::{Message, MessageQueue, ReceiveOptions, SendOptions};
use cloudkit_spi::{CloudError, CloudResult, ResourceId};
use cloudkit_spi::CloudContext;
use std::sync::Arc;
use std::time::Duration;

/// AWS SQS message queue implementation.
pub struct SqsQueue {
    _context: Arc<CloudContext>,
    client: aws_sdk_sqs::Client,
}

impl SqsQueue {
    /// Create a new SQS queue client.
    pub fn new(context: Arc<CloudContext>, sdk_config: aws_config::SdkConfig) -> Self {
        let client = aws_sdk_sqs::Client::new(&sdk_config);
        Self { _context: context, client }
    }
}

#[async_trait]
impl MessageQueue for SqsQueue {
    async fn create_queue(&self, name: &str) -> CloudResult<String> {
        let resp = self.client.create_queue()
            .queue_name(name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.queue_url().unwrap_or_default().to_string())
    }

    async fn delete_queue(&self, queue_url: &str) -> CloudResult<()> {
        self.client.delete_queue()
            .queue_url(queue_url)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn get_queue_url(&self, name: &str) -> CloudResult<String> {
        let resp = self.client.get_queue_url()
            .queue_name(name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.queue_url().unwrap_or_default().to_string())
    }

    async fn list_queues(&self, prefix: Option<&str>) -> CloudResult<Vec<String>> {
        let mut req = self.client.list_queues();
        if let Some(p) = prefix {
            req = req.queue_name_prefix(p);
        }
        let resp = req.send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.queue_urls().iter().map(|u| u.to_string()).collect())
    }

    async fn send(&self, queue_url: &str, body: &str) -> CloudResult<ResourceId> {
        self.send_with_options(queue_url, body, SendOptions::default()).await
    }

    async fn send_with_options(
        &self,
        queue_url: &str,
        body: &str,
        options: SendOptions,
    ) -> CloudResult<ResourceId> {
        let mut req = self.client.send_message()
            .queue_url(queue_url)
            .message_body(body);
            
        if let Some(delay) = options.delay {
            req = req.delay_seconds(delay.as_secs() as i32);
        }
        
        if let Some(group_id) = options.message_group_id {
            req = req.message_group_id(group_id);
        }
        
        if let Some(dedup_id) = options.deduplication_id {
            req = req.message_deduplication_id(dedup_id);
        }
        
        for (k, v) in options.attributes {
            req = req.message_attributes(k, aws_sdk_sqs::types::MessageAttributeValue::builder()
                .string_value(v)
                .data_type("String")
                .build()
                .unwrap());
        }
        
        let resp = req.send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(ResourceId::new(resp.message_id().unwrap_or_default()))
    }

    async fn send_batch(
        &self,
        queue_url: &str,
        messages: &[&str],
    ) -> CloudResult<Vec<ResourceId>> {
        let mut entries = Vec::new();
        for (i, body) in messages.iter().enumerate() {
            entries.push(aws_sdk_sqs::types::SendMessageBatchRequestEntry::builder()
                .id(i.to_string())
                .message_body(body.to_string())
                .build()
                .unwrap());
        }
        
        let resp = self.client.send_message_batch()
            .queue_url(queue_url)
            .set_entries(Some(entries))
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.successful().iter().map(|s| ResourceId::new(s.message_id())).collect())
    }

    async fn receive(
        &self,
        queue_url: &str,
        options: ReceiveOptions,
    ) -> CloudResult<Vec<Message>> {
        let mut req = self.client.receive_message()
            .queue_url(queue_url)
            .set_message_attribute_names(Some(vec!["All".to_string()]))
            .set_message_system_attribute_names(Some(vec![
                aws_sdk_sqs::types::MessageSystemAttributeName::All,
            ]));
            
        if let Some(max) = options.max_messages {
            req = req.max_number_of_messages(max as i32);
        }
        
        if let Some(timeout) = options.visibility_timeout {
            req = req.visibility_timeout(timeout.as_secs() as i32);
        }
        
        if let Some(wait) = options.wait_time {
            req = req.wait_time_seconds(wait.as_secs() as i32);
        }
        
        let resp = req.send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        let messages = resp.messages().iter().map(|m| {
            let mut attributes = std::collections::HashMap::new();
            if let Some(attrs) = m.message_attributes() {
                for (k, v) in attrs {
                    if let Some(s) = v.string_value() {
                        attributes.insert(k.clone(), s.to_string());
                    }
                }
            }

            let receive_count = m.attributes()
                .and_then(|a| a.get(&aws_sdk_sqs::types::MessageSystemAttributeName::ApproximateReceiveCount))
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);

            let sent_at = m.attributes()
                .and_then(|a| a.get(&aws_sdk_sqs::types::MessageSystemAttributeName::SentTimestamp))
                .and_then(|s| s.parse::<i64>().ok())
                .and_then(|ms| chrono::DateTime::<chrono::Utc>::from_timestamp(ms / 1000, (ms % 1000) as u32 * 1_000_000))
                .unwrap_or_else(chrono::Utc::now);

            Message {
                id: ResourceId::new(m.message_id().unwrap_or_default()),
                body: m.body().unwrap_or_default().to_string(),
                receipt_handle: m.receipt_handle().map(|h| h.to_string()),
                attributes,
                receive_count,
                sent_at,
                first_received_at: None, // SQS doesn't provide this directly in receive_message
            }
        }).collect();
        
        Ok(messages)
    }

    async fn delete(&self, queue_url: &str, message: &Message) -> CloudResult<()> {
        let handle = message.receipt_handle.as_ref().ok_or_else(|| CloudError::Validation("Message missing receipt handle".into()))?;
        self.client.delete_message()
            .queue_url(queue_url)
            .receipt_handle(handle)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn delete_batch(&self, queue_url: &str, messages: &[&Message]) -> CloudResult<()> {
        let mut entries = Vec::new();
        for (i, msg) in messages.iter().enumerate() {
            if let Some(handle) = &msg.receipt_handle {
                entries.push(aws_sdk_sqs::types::DeleteMessageBatchRequestEntry::builder()
                    .id(i.to_string())
                    .receipt_handle(handle)
                    .build()
                    .unwrap());
            }
        }
        
        self.client.delete_message_batch()
            .queue_url(queue_url)
            .set_entries(Some(entries))
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(())
    }

    async fn change_visibility(
        &self,
        queue_url: &str,
        message: &Message,
        timeout: Duration,
    ) -> CloudResult<()> {
        let handle = message.receipt_handle.as_ref().ok_or_else(|| CloudError::Validation("Message missing receipt handle".into()))?;
        self.client.change_message_visibility()
            .queue_url(queue_url)
            .receipt_handle(handle)
            .visibility_timeout(timeout.as_secs() as i32)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn get_queue_depth(&self, queue_url: &str) -> CloudResult<u64> {
        let resp = self.client.get_queue_attributes()
            .queue_url(queue_url)
            .attribute_names(aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessages)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        let depth = resp.attributes()
            .and_then(|a| a.get(&aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessages))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
            
        Ok(depth)
    }

    async fn purge(&self, queue_url: &str) -> CloudResult<()> {
        self.client.purge_queue()
            .queue_url(queue_url)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }
}

