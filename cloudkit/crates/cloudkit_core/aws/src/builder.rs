//! AWS client builder.

use cloudkit_spi::{CloudConfig, CloudResult, Region};
use cloudkit_spi::{CloudContext, ProviderType};
// use cloudkit_spi::spi::{AuthProvider, MetricsCollector, RetryPolicy};
use std::sync::Arc;

/// AWS client builder.
pub struct AwsBuilder {
    region: Option<Region>,
    config: Option<CloudConfig>,
    profile: Option<String>,
}

impl AwsBuilder {
    /// Create a new AWS builder.
    pub fn new() -> Self {
        Self {
            region: None,
            config: None,
            profile: None,
        }
    }

    /// Set the AWS region.
    pub fn region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    /// Set the AWS profile name.
    pub fn profile(mut self, profile: impl Into<String>) -> Self {
        self.profile = Some(profile.into());
        self
    }

    /// Set the configuration.
    pub fn config(mut self, config: CloudConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Build the AWS client.
    pub async fn build(self) -> CloudResult<AwsClient> {
        let mut config = self.config.unwrap_or_default();
        
        if let Some(ref region) = self.region {
            config.region = region.clone();
        }

        let context = CloudContext::builder(ProviderType::Aws)
            .config(config.clone())
            .build()
            .await?;

        // Initialize SDK config
        let mut loader = aws_config::defaults(aws_config::BehaviorVersion::latest());
        
        if let Some(ref profile) = self.profile {
            loader = loader.profile_name(profile);
        }

        if let Some(ref endpoint) = config.endpoint {
            loader = loader.endpoint_url(endpoint);
        }

        let sdk_config = loader.load().await;

        Ok(AwsClient {
            context: Arc::new(context),
            profile: self.profile,
            sdk_config,
        })
    }
}

impl Default for AwsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// AWS client.
pub struct AwsClient {
    context: Arc<CloudContext>,
    profile: Option<String>,
    sdk_config: aws_config::SdkConfig,
}

impl AwsClient {
    /// Get the underlying context.
    pub fn context(&self) -> &CloudContext {
        &self.context
    }

    /// Get the profile name.
    pub fn profile(&self) -> Option<&str> {
        self.profile.as_deref()
    }

    /// Get the S3 storage client.
    #[cfg(feature = "s3")]
    pub fn storage(&self) -> super::s3::S3Storage {
        super::s3::S3Storage::new(self.context.clone(), self.sdk_config.clone())
    }

    /// Get the DynamoDB client.
    #[cfg(feature = "dynamodb")]
    pub fn kv_store(&self) -> super::dynamodb::DynamoDbStore {
        super::dynamodb::DynamoDbStore::new(self.context.clone(), self.sdk_config.clone())
    }

    /// Get the SQS client.
    #[cfg(feature = "sqs")]
    pub fn queue(&self) -> super::sqs::SqsQueue {
        super::sqs::SqsQueue::new(self.context.clone(), self.sdk_config.clone())
    }

    /// Get the SNS client.
    #[cfg(feature = "sns")]
    pub fn pubsub(&self) -> super::sns::SnsPubSub {
        super::sns::SnsPubSub::new(self.context.clone(), self.sdk_config.clone())
    }

    /// Get the Lambda client.
    #[cfg(feature = "lambda")]
    pub fn functions(&self) -> super::lambda::LambdaFunctions {
        super::lambda::LambdaFunctions::new(self.context.clone(), self.sdk_config.clone())
    }

    /// Get the Secrets Manager client.
    #[cfg(feature = "secrets")]
    pub fn secrets(&self) -> super::secrets::AwsSecretsManager {
        super::secrets::AwsSecretsManager::new(self.context.clone(), self.sdk_config.clone())
    }

    /// Get the KMS client.
    #[cfg(feature = "kms")]
    pub fn kms(&self) -> super::kms::AwsKms {
        super::kms::AwsKms::new(self.context.clone(), self.sdk_config.clone())
    }

    /// Get the CloudWatch client.
    #[cfg(feature = "cloudwatch")]
    pub fn monitoring(&self) -> super::cloudwatch::AwsMonitoring {
        super::cloudwatch::AwsMonitoring::new(self.context.clone(), self.sdk_config.clone())
    }

    /// Get the EventBridge client.
    #[cfg(feature = "eventbridge")]
    pub fn events(&self) -> super::eventbridge::AwsEvents {
        super::eventbridge::AwsEvents::new(self.context.clone(), self.sdk_config.clone())
    }

    /// Get the Step Functions client.
    #[cfg(feature = "stepfunctions")]
    pub fn workflow(&self) -> super::stepfunctions::AwsWorkflow {
        super::stepfunctions::AwsWorkflow::new(self.context.clone(), self.sdk_config.clone())
    }

    /// Get the Cognito client.
    #[cfg(feature = "cognito")]
    pub fn identity(&self) -> super::cognito::AwsIdentity {
        super::cognito::AwsIdentity::new(self.context.clone(), self.sdk_config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_aws_builder() {
        let client = AwsBuilder::new()
            .region(Region::aws_us_east_1())
            .profile("default")
            .build()
            .await
            .unwrap();

        assert_eq!(client.context().provider(), ProviderType::Aws);
        assert_eq!(client.profile(), Some("default"));
    }
}

