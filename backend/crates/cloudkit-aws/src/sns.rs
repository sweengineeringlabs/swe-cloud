use async_trait::async_trait;
use cloudkit_spi::api::{PubSub, SubscriptionConfig};
use cloudkit_spi::common::{CloudError, CloudResult, ResourceId};
use cloudkit_spi::core::CloudContext;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

/// AWS SNS pub/sub implementation.
pub struct SnsPubSub {
    _context: Arc<CloudContext>,
    client: aws_sdk_sns::Client,
}

impl SnsPubSub {
    /// Create a new SNS pub/sub client.
    pub fn new(context: Arc<CloudContext>, sdk_config: aws_config::SdkConfig) -> Self {
        let client = aws_sdk_sns::Client::new(&sdk_config);
        Self { _context: context, client }
    }
}

#[async_trait]
impl PubSub for SnsPubSub {
    async fn create_topic(&self, name: &str) -> CloudResult<String> {
        let resp = self.client.create_topic()
            .name(name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.topic_arn().unwrap_or_default().to_string())
    }

    async fn delete_topic(&self, topic_arn: &str) -> CloudResult<()> {
        self.client.delete_topic()
            .topic_arn(topic_arn)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn list_topics(&self) -> CloudResult<Vec<String>> {
        let resp = self.client.list_topics()
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.topics().iter().map(|t| t.topic_arn().unwrap_or_default().to_string()).collect())
    }

    async fn get_topic_arn(&self, name: &str) -> CloudResult<String> {
        let topics = self.list_topics().await?;
        for arn in topics {
            if arn.ends_with(&format!(":{}", name)) {
                return Ok(arn);
            }
        }
        Err(CloudError::NotFound {
            resource_type: "Topic".to_string(),
            resource_id: name.to_string(),
        })
    }

    async fn subscribe(
        &self,
        topic_arn: &str,
        protocol: &str,
        endpoint: &str,
    ) -> CloudResult<String> {
        let resp = self.client.subscribe()
            .topic_arn(topic_arn)
            .protocol(protocol)
            .endpoint(endpoint)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.subscription_arn().unwrap_or_default().to_string())
    }

    async fn subscribe_with_config(
        &self,
        topic_arn: &str,
        config: SubscriptionConfig,
    ) -> CloudResult<String> {
        let mut req = self.client.subscribe()
            .topic_arn(topic_arn);
            
        if let Some(endpoint) = config.push_endpoint {
            req = req.endpoint(endpoint).protocol("https"); // Assuming https for push
        }
        
        // AWS SNS doesn't directly take a SubscriptionConfig struct, 
        // attributes are set differently. This is a simplified version.
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        let sub_arn = resp.subscription_arn().unwrap_or_default().to_string();
        
        if let Some(policy) = config.filter_policy {
            let policy_json = serde_json::to_string(&policy).unwrap_or_default();
            self.client.set_subscription_attributes()
                .subscription_arn(&sub_arn)
                .attribute_name("FilterPolicy")
                .attribute_value(policy_json)
                .send()
                .await
                .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        }
        
        Ok(sub_arn)
    }

    async fn unsubscribe(&self, subscription_arn: &str) -> CloudResult<()> {
        self.client.unsubscribe()
            .subscription_arn(subscription_arn)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn list_subscriptions(&self, _topic_arn: &str) -> CloudResult<Vec<String>> {
        let resp = self.client.list_subscriptions()
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.subscriptions().iter().map(|s| s.subscription_arn().unwrap_or_default().to_string()).collect())
    }

    async fn publish(&self, topic_arn: &str, message: &[u8]) -> CloudResult<ResourceId> {
        self.publish_with_attributes(topic_arn, message, HashMap::new()).await
    }

    async fn publish_with_attributes(
        &self,
        topic_arn: &str,
        message: &[u8],
        attributes: HashMap<String, String>,
    ) -> CloudResult<ResourceId> {
        let msg_str = String::from_utf8_lossy(message);
        let mut req = self.client.publish()
            .topic_arn(topic_arn)
            .message(msg_str);
            
        for (k, v) in attributes {
            req = req.message_attributes(k, aws_sdk_sns::types::MessageAttributeValue::builder()
                .string_value(v)
                .data_type("String")
                .build()
                .unwrap());
        }
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(ResourceId::new(resp.message_id().unwrap_or_default()))
    }

    async fn publish_batch(
        &self,
        topic_arn: &str,
        messages: &[&[u8]],
    ) -> CloudResult<Vec<ResourceId>> {
        let mut results = Vec::new();
        for msg in messages {
            results.push(self.publish(topic_arn, msg).await?);
        }
        Ok(results)
    }

    async fn publish_json<T: Serialize + Send + Sync>(
        &self,
        topic_arn: &str,
        message: &T,
    ) -> CloudResult<ResourceId> {
        let json = serde_json::to_vec(message).map_err(|e| CloudError::Serialization(e.to_string()))?;
        self.publish(topic_arn, &json).await
    }
}

