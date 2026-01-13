use crate::Emulator;
use crate::error::EmulatorError;
use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::info;

pub async fn handle_request(
    State(emulator): State<Arc<Emulator>>,
    _headers: HeaderMap,
    Json(body): Json<Value>,
) -> Response {
    let action = body["Action"].as_str().unwrap_or("");

    let result = match action {
        "CreateTopic" => create_topic(&emulator, body).await,
        "Subscribe" => subscribe(&emulator, body).await,
        "Publish" => publish(&emulator, body).await,
        "ListTopics" => list_topics(&emulator, body).await,
        _ => Err(EmulatorError::InvalidRequest(format!("Unsupported SNS action: {}", action))),
    };

    match result {
        Ok(json_val) => Json(json_val).into_response(),
        Err(e) => {
            let status = e.status_code();
            let json_err = json!({
                "Error": {
                    "Code": e.code(),
                    "Message": e.message()
                }
            });
            (status, Json(json_err)).into_response()
        }
    }
}

async fn create_topic(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let name = body["Name"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Name".into()))?;
    let topic = emulator.storage.create_topic(name, &emulator.config.account_id, &emulator.config.region)?;
    
    Ok(json!({
        "CreateTopicResponse": {
            "CreateTopicResult": {
                "TopicArn": topic.arn
            }
        }
    }))
}

async fn subscribe(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let topic_arn = body["TopicArn"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing TopicArn".into()))?;
    let protocol = body["Protocol"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Protocol".into()))?;
    let endpoint = body["Endpoint"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Endpoint".into()))?;
    
    let sub_arn = emulator.storage.subscribe(topic_arn, protocol, endpoint)?;
    
    Ok(json!({
        "SubscribeResponse": {
            "SubscribeResult": {
                "SubscriptionArn": sub_arn
            }
        }
    }))
}

async fn publish(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let topic_arn = body["TopicArn"].as_str()
        .ok_or_else(|| EmulatorError::InvalidArgument("Missing TopicArn".into()))?;
    let message = body["Message"].as_str()
        .ok_or_else(|| EmulatorError::InvalidArgument("Missing Message".into()))?;
    let subject = body["Subject"].as_str();
    
    let message_id = uuid::Uuid::new_v4().to_string();
    
    // Get all subscriptions for this topic
    let subscriptions = emulator.storage.list_subscriptions_by_topic(topic_arn)?;
    
    info!("SNS: Publishing message to {} ({} subscribers)", topic_arn, subscriptions.len());
    
    // Deliver to each subscriber
    for sub in subscriptions {
        match sub.protocol.as_str() {
            "sqs" => {
                // Deliver to SQS queue
                if let Some(queue_name) = sub.endpoint.split('/').last() {
                    let sqs_message = json!({
                        "Type": "Notification",
                        "MessageId": message_id,
                        "TopicArn": topic_arn,
                        "Subject": subject.unwrap_or(""),
                        "Message": message,
                        "Timestamp": chrono::Utc::now().to_rfc3339()
                    });
                    
                    if let Err(e) = emulator.storage.send_message(
                        queue_name,
                        &sqs_message.to_string()
                    ) {
                        tracing::warn!("Failed to deliver SNS message to SQS {}: {}", queue_name, e);
                    } else {
                        info!("SNS: Delivered to SQS queue {}", queue_name);
                    }
                }
            },
            "http" | "https" => {
                // For HTTP/HTTPS, we'd need to make actual HTTP requests
                // For a local emulator, we'll log it
                info!("SNS: Would deliver to HTTP endpoint {} (not implemented in emulator)", sub.endpoint);
            },
            "email" | "email-json" => {
                info!("SNS: Would send email to {} (not implemented in emulator)", sub.endpoint);
            },
            "sms" => {
                info!("SNS: Would send SMS to {} (not implemented in emulator)", sub.endpoint);
            },
            "lambda" => {
                // Invoke Lambda function
                info!("SNS: Would invoke Lambda {} (Lambda execution not yet implemented)", sub.endpoint);
            },
            _ => {
                tracing::warn!("SNS: Unknown protocol {}", sub.protocol);
            }
        }
    }
    
    Ok(json!({
        "PublishResponse": {
            "PublishResult": {
                "MessageId": message_id
            }
        }
    }))
}

async fn list_topics(emulator: &Emulator, _body: Value) -> Result<Value, EmulatorError> {
    let topics = emulator.storage.list_topics()?;
    
    let topic_list: Vec<Value> = topics.into_iter().map(|t| {
        json!({ "TopicArn": t.arn })
    }).collect();
    
    Ok(json!({
        "ListTopicsResponse": {
            "ListTopicsResult": {
                "Topics": topic_list
            }
        }
    }))
}
