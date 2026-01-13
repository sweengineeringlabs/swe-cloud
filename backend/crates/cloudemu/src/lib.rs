//! # CloudEmu - Production-Grade Local Cloud Emulator
//!
//! CloudEmu emulates AWS cloud services locally with production-level accuracy.
//! It works with Terraform, AWS SDKs, and the AWS CLI out of the box.
//!
//! ## Supported Services
//!
//! - **S3**: Object storage with versioning, policies, lifecycle
//! - **DynamoDB**: NoSQL database
//! - **SQS/SNS**: Messaging and Pub/Sub
//! - **Lambda**: Serverless functions
//! - **KMS/Secrets**: Security and encryption
//! - **And more**: EventBridge, CloudWatch, Cognito, etc.
//!
//! ## Quick Start
//!
//! ```bash
//! # Start the emulator
//! cargo run -p cloudemu
//!
//! # Use with Terraform
//! # provider "aws" {
//! #   endpoints { s3 = "http://localhost:4566" }
//! #   skip_credentials_validation = true
//! # }
//!
//! # Use with AWS CLI
//! aws --endpoint-url=http://localhost:4566 s3 mb s3://my-bucket
//! ```

pub mod config;
pub mod error;
pub mod gateway;
pub mod services;
pub mod storage;

pub use config::Config;
pub use error::{EmulatorError, Result};

use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

/// Emulator state containing all services and storage
pub struct Emulator {
    pub config: Config,
    pub storage: storage::StorageEngine,
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
}

impl Emulator {
    /// Create a new emulator with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(Config::default())
    }

    /// Create a new in-memory emulator
    pub fn in_memory() -> Result<Self> {
        let config = Config::default();
        let storage = storage::StorageEngine::in_memory()?;
        
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

    /// Create a new emulator with custom configuration
    pub fn with_config(config: Config) -> Result<Self> {
        // Initialize storage engine
        let storage = storage::StorageEngine::new(&config)?;
        
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
pub async fn start_server(config: Config) -> Result<()> {
    let addr = format!("{}:{}", config.host, config.port);
    let emulator = Arc::new(Emulator::with_config(config)?);
    
    let app = gateway::create_router(emulator.clone());
    
    info!("CloudEmu starting on http://{}", addr);
    info!("─────────────────────────────────────────");
    info!("Services:");
    #[cfg(feature = "s3")]
    info!("  ✓ S3 (Object Storage)");
    #[cfg(feature = "dynamodb")]
    info!("  ✓ DynamoDB (NoSQL)");
    #[cfg(feature = "sqs")]
    info!("  ✓ SQS (Queues)");
    #[cfg(feature = "secretsmanager")]
    info!("  ✓ Secrets Manager");
    #[cfg(feature = "eventbridge")]
    info!("  ✓ EventBridge");
    #[cfg(feature = "kms")]
    info!("  ✓ KMS");
    #[cfg(feature = "cloudwatch")]
    info!("  ✓ CloudWatch");
    #[cfg(feature = "cognito")]
    info!("  ✓ Cognito");
    #[cfg(feature = "stepfunctions")]
    info!("  ✓ Step Functions");
    #[cfg(feature = "sns")]
    info!("  ✓ SNS");
    #[cfg(feature = "lambda")]
    info!("  ✓ Lambda");
    info!("─────────────────────────────────────────");
    info!("Data directory: {}", emulator.config.data_dir.display());
    info!("Region: {}", emulator.config.region);
    info!("─────────────────────────────────────────");
    info!("Ready for connections");
    
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
