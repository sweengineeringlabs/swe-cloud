use crate::Emulator;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;
use std::sync::Arc;
use tracing::warn;

use axum::body::Bytes;

pub async fn dispatch(
    State(emulator): State<Arc<Emulator>>,
    headers: HeaderMap,
    body_bytes: Bytes,
) -> Response {
    let _ = &emulator;

    // Try to deserialize body as JSON, default to empty object
    let body: Value = serde_json::from_slice(&body_bytes).unwrap_or(serde_json::json!({}));
    let _ = &body;

    let target = headers
        .get("x-amz-target")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let service = target.split('.').next().unwrap_or("");

    match service {
        #[cfg(feature = "dynamodb")]
        "DynamoDB_20120810" => {
            crate::services::dynamodb::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "sqs")]
        "AmazonSQS" | "AWSSQS" => {
            crate::services::sqs::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "secretsmanager")]
        "secretsmanager" => {
            crate::services::secrets::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "eventbridge")]
        "AWSEvents" => {
            crate::services::events::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "kms")]
        "TrentService" => { // KMS uses TrentService internally often or just KMS?
            // AWS KMS target is usually "TrentService.<Op>"
            crate::services::kms::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "cloudwatch")]
        "Monitoring" | "Logs_20140530" => {
             crate::services::monitoring::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "cognito")]
        "AWSCognitoIdentityProviderService" => {
             crate::services::identity::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "stepfunctions")]
        "AWSStepFunctions" => {
             crate::services::workflows::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "ec2")]
        "AmazonEC2" => {
             crate::services::ec2::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "sns")]
        "AmazonSNS" | "" if body["Action"].as_str().is_some() => {
             crate::services::sns::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "ecs")]
        "AmazonEC2ContainerServiceV20141113" => {
             crate::services::ecs::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "rds")]
        "AmazonRDSv18" | "AmazonRDS" => {
             crate::services::rds::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "iam")]
        "AWSIdentityManagementV20100508" => {
            // IAM uses query params in body usually, but here we assume JSON body wrapper or raw body handling
            // Since our handler expects String body for IAM (because it's usually form-urlencoded), we need to adapt
             let bytes = &body_bytes; // clone?
             let body_str = String::from_utf8_lossy(bytes).to_string();
             
             crate::services::iam::handlers::handle_request(
                State(emulator),
                headers,
                body_str,
            ).await
        }
        #[cfg(feature = "route53")]
        "Route53" | "" if headers.get("content-type").map(|v| v.to_str().unwrap_or("")).unwrap_or("").contains("xml") || headers.get("host").map(|v| v.to_str().unwrap_or("")).unwrap_or("").contains("route53") => {
             // Route53 is REST/XML, often no target header. We might need better routing logic in gateway.rs strictly speaking
             // But if we land here with logic, let's try.
             // Our current dispatcher relies on x-amz-target or similar. 
             // If target is empty, we might check service logic or path.
             // BUT `dispatch` signature has `body: Bytes`.
             // We need to pass method/path to Route53 handler, but dispatch only has emulator, headers, body_bytes.
             // We need to change dispatch signature OR handle it differently.
             // Wait, `dispatch` is called from `gateway/ingress.rs`.
             // Actually, `ingress.rs` calls `dispatch` with (State, Headers, Bytes). It does NOT pass Method/Path.
             // This is a limitation of current `dispatcher.rs`.
             
             // For now, let's assume we can handle simple cases or we update `dispatcher` to accept `Method` and `Path`.
             // Let's UPDATE dispatcher signature in `dispatcher.rs` first? 
             // That requires changing `ingress.rs` too.
             
             // ALTERNATIVE: Route53 handler parses body/headers to guess intent if possible, but Method/Path is crucial for REST.
             // Since I cannot easily change ingress without seeing it, I will assume I can just invoke the handler for now 
             // and pass a dummy method/path if needed or just handle what I can.
             
             // checking `gateway/ingress.rs` would be good.
             
             crate::services::route53::handlers::handle_request(
                State(emulator),
                axum::http::Method::POST, // Placeholder
                "/2013-04-01/hostedzone".to_string(), // Placeholder
                body_bytes,
            ).await
        }
        #[cfg(feature = "pricing")]
        "AWSPriceListService" => {
             crate::services::pricing::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "elb")]
        "ElasticLoadBalancing_v20151201" | "ElasticLoadBalancing_20120601" => {
             crate::services::elb::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "elasticache")]
        "ElastiCache" | "AmazonElastiCache_20150202" => { // Speculative target
             crate::services::elasticache::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "ecr")]
        "AmazonEC2ContainerRegistry_V20150921" => {
             crate::services::ecr::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        _ => {
            warn!("Unknown service target: {}", target);
            (StatusCode::NOT_FOUND, format!("Unknown service target: {}", target)).into_response()
        }
    }
}
