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

async fn publish(_emulator: &Emulator, _body: Value) -> Result<Value, EmulatorError> {
    // Basic implementation - just returns a message ID
    let message_id = uuid::Uuid::new_v4().to_string();
    
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
