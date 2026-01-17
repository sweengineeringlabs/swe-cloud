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
    #[cfg(feature = "ecs")]
    pub ecs: services::ecs::EcsService,
    #[cfg(feature = "rds")]
    pub rds: services::rds::RdsService,
    #[cfg(feature = "iam")]
    pub iam: services::iam::IamService,
    #[cfg(feature = "route53")]
    pub route53: services::route53::Route53Service,
    #[cfg(feature = "pricing")]
    pub pricing: services::pricing::PricingService,
    #[cfg(feature = "apigateway")]
    pub apigateway: services::apigateway::ApiGatewayService,
    #[cfg(feature = "elb")]
    pub elb: services::elb::ElbService,
    #[cfg(feature = "elasticache")]
    pub elasticache: services::elasticache::ElastiCacheService,
    #[cfg(feature = "ecr")]
    pub ecr: services::ecr::EcrService,
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
            #[cfg(feature = "ecs")]
            ecs: services::ecs::EcsService::new(),
            #[cfg(feature = "rds")]
            rds: services::rds::RdsService::new(),
            #[cfg(feature = "iam")]
            iam: services::iam::IamService::new(),
            #[cfg(feature = "route53")]
            route53: services::route53::Route53Service::new(),
            #[cfg(feature = "pricing")]
            pricing: services::pricing::PricingService::new(storage.clone()),
            #[cfg(feature = "apigateway")]
            apigateway: services::apigateway::ApiGatewayService::new(storage.clone()),
            #[cfg(feature = "elb")]
            elb: services::elb::ElbService::new(storage.clone()),
            #[cfg(feature = "elasticache")]
            elasticache: services::elasticache::ElastiCacheService::new(storage.clone()),
            #[cfg(feature = "ecr")]
            ecr: services::ecr::EcrService::new(storage.clone()),
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
            #[cfg(feature = "ec2")]
            ec2: services::ec2::Ec2Service::new(storage.clone()),
            #[cfg(feature = "ecs")]
            ecs: services::ecs::EcsService::new(),
            #[cfg(feature = "rds")]
            rds: services::rds::RdsService::new(),
            #[cfg(feature = "iam")]
            iam: services::iam::IamService::new(),
            #[cfg(feature = "route53")]
            route53: services::route53::Route53Service::new(),
            #[cfg(feature = "pricing")]
            pricing: services::pricing::PricingService::new(storage.clone()),
            #[cfg(feature = "apigateway")]
            apigateway: services::apigateway::ApiGatewayService::new(storage.clone()),
            #[cfg(feature = "elb")]
            elb: services::elb::ElbService::new(storage.clone()),
            #[cfg(feature = "elasticache")]
            elasticache: services::elasticache::ElastiCacheService::new(storage.clone()),
            #[cfg(feature = "ecr")]
            ecr: services::ecr::EcrService::new(storage.clone()),
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
