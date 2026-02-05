use crate::Emulator;
use crate::error::Result;
use aws_data_core::Config;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

/// Ingress Controller: Starts the server and binds the Gateway
pub async fn start(host: &str, port: u16, data_dir: PathBuf) -> Result<()> {
    let config = Config {
        host: host.to_string(),
        port,
        data_dir,
        ..Default::default()
    };
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
        .map_err(aws_data_core::error::EmulatorError::Io)?;
        
    axum::serve(listener, app).await
         .map_err(aws_data_core::error::EmulatorError::Io)?;
    
    Ok(())
}
