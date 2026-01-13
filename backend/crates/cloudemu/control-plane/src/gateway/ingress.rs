use crate::Emulator;
use crate::error::Result;
use data_plane::Config;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

/// Ingress Controller: Starts the server and binds the Gateway
pub async fn start(config: Config) -> Result<()> {
    let addr = format!("{}:{}", config.host, config.port);
    let emulator = Arc::new(Emulator::with_config(config)?);
    
    // Gateway creation (Router)
    let app = super::gateway::create_router(emulator.clone());
    
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
    
    let listener = TcpListener::bind(&addr).await
        .map_err(|e| data_plane::error::EmulatorError::Io(e))?;
        
    axum::serve(listener, app).await
         .map_err(|e| data_plane::error::EmulatorError::Io(e))?;
    
    Ok(())
}
