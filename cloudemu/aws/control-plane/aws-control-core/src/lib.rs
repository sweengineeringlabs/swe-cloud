pub mod adapters;
pub mod error;
pub mod gateway;
pub mod services;

pub use error::{ApiError, Result};
// use std::sync::Arc;
// use tokio::net::TcpListener;
// use tracing::info;

use aws_data_core::{Config, StorageEngine};

/// Emulator state containing all services and storage
pub struct Emulator {
    pub config: Config,
    pub storage: StorageEngine,
    #[cfg(feature = "s3")]
    pub s3: services::s3::S3Service,
    #[cfg(feature = "dynamodb")]
    pub dynamodb: services::dynamodb::DynamoDbService,
    #[cfg(feature = "sqs")]
    pub sqs: services::sqs::SqsService,
    #[cfg(feature = "secretsmanager")]
    pub secrets: services::secrets::SecretsService,
    #[cfg(feature = "eventbridge")]
    pub events: services::events::EventsService,
    #[cfg(feature = "kms")]
    pub kms: services::kms::KmsService,
    #[cfg(feature = "cloudwatch")]
    pub monitoring: services::monitoring::MonitoringService,
    #[cfg(feature = "cognito")]
    pub identity: services::identity::IdentityService,
    #[cfg(feature = "stepfunctions")]
    pub workflows: services::workflows::WorkflowsService,
    #[cfg(feature = "sns")]
    pub sns: services::sns::SnsService,
    #[cfg(feature = "lambda")]
    pub lambda: services::lambda::LambdaService,
    #[cfg(feature = "ec2")]
    pub ec2: services::ec2::Ec2Service,
}

impl Emulator {
    /// Create a new emulator with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(Config::default())
    }

    /// Create a new in-memory emulator
    pub fn in_memory() -> Result<Self> {
        let config = Config::default();
        let storage = StorageEngine::in_memory()?;
        
        Ok(Self {
            #[cfg(feature = "s3")]
            s3: services::s3::S3Service::new(storage.clone()),
            #[cfg(feature = "dynamodb")]
            dynamodb: services::dynamodb::DynamoDbService::new(storage.clone()),
            #[cfg(feature = "sqs")]
            sqs: services::sqs::SqsService::new(storage.clone()),
            #[cfg(feature = "secretsmanager")]
            secrets: services::secrets::SecretsService::new(storage.clone()),
            #[cfg(feature = "eventbridge")]
            events: services::events::EventsService::new(storage.clone()),
            #[cfg(feature = "kms")]
            kms: services::kms::KmsService::new(storage.clone()),
            #[cfg(feature = "cloudwatch")]
            monitoring: services::monitoring::MonitoringService::new(storage.clone()),
            #[cfg(feature = "cognito")]
            identity: services::identity::IdentityService::new(storage.clone()),
            #[cfg(feature = "stepfunctions")]
            workflows: services::workflows::WorkflowsService::new(storage.clone()),
            #[cfg(feature = "sns")]
            sns: services::sns::SnsService::new(storage.clone()),
            #[cfg(feature = "lambda")]
            lambda: services::lambda::LambdaService::new(storage.clone()),
            #[cfg(feature = "ec2")]
            ec2: services::ec2::Ec2Service::new(storage.clone()),
            storage,
            config,
        })
    }

    /// Create a new emulator with custom configuration
    pub fn with_config(config: Config) -> Result<Self> {
        // Initialize storage engine
        let storage = StorageEngine::new(&config)?;
        
        Ok(Self {
            #[cfg(feature = "s3")]
            s3: services::s3::S3Service::new(storage.clone()),
            #[cfg(feature = "dynamodb")]
            dynamodb: services::dynamodb::DynamoDbService::new(storage.clone()),
            #[cfg(feature = "sqs")]
            sqs: services::sqs::SqsService::new(storage.clone()),
            #[cfg(feature = "secretsmanager")]
            secrets: services::secrets::SecretsService::new(storage.clone()),
            #[cfg(feature = "eventbridge")]
            events: services::events::EventsService::new(storage.clone()),
            #[cfg(feature = "kms")]
            kms: services::kms::KmsService::new(storage.clone()),
            #[cfg(feature = "cloudwatch")]
            monitoring: services::monitoring::MonitoringService::new(storage.clone()),
            #[cfg(feature = "cognito")]
            identity: services::identity::IdentityService::new(storage.clone()),
            #[cfg(feature = "stepfunctions")]
            workflows: services::workflows::WorkflowsService::new(storage.clone()),
            #[cfg(feature = "sns")]
            sns: services::sns::SnsService::new(storage.clone()),
            #[cfg(feature = "lambda")]
            lambda: services::lambda::LambdaService::new(storage.clone()),
            storage,
            config,
        })
    }
    
    /// Get the endpoint URL
    pub fn endpoint(&self) -> String {
        format!("http://{}:{}", self.config.host, self.config.port)
    }
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new().expect("Failed to create emulator")
    }
}

/// Start the emulator server
pub use gateway::ingress::start as start_server;
