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
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Response {
    let target = headers
        .get("x-amz-target")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    
    let action = target.split('.').next_back().unwrap_or("");
    
    // If no target header, it might be a query-based request or the action is in the body
    let action = if action.is_empty() {
        body["Action"].as_str().unwrap_or("")
    } else {
        action
    };

    let result = match action {
        "CreateQueue" => create_queue(&emulator, body).await,
        "SendMessage" => send_message(&emulator, body).await,
        "ReceiveMessage" => receive_message(&emulator, body).await,
        "DeleteMessage" => delete_message(&emulator, body).await,
        "ListQueues" => list_queues(&emulator, body).await,
        _ => Err(EmulatorError::InvalidRequest(format!("Unsupported SQS action: {}", action))),
    };

    match result {
        Ok(json_val) => Json(json_val).into_response(),
        Err(e) => {
            let status = e.status_code();
            let json_err = json!({
                "__type": e.code(),
                "message": e.message()
            });
            (status, Json(json_err)).into_response()
        }
    }
}

async fn create_queue(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let name = body["QueueName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing QueueName".into()))?;
    let queue = emulator.storage.create_queue(name, &emulator.config.account_id, &emulator.config.region)?;
    
    Ok(json!({
        "QueueUrl": queue.url
    }))
}

async fn send_message(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let queue_url = body["QueueUrl"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing QueueUrl".into()))?;
    let queue_name = queue_url.split('/').next_back().unwrap_or("");
    let message_body = body["MessageBody"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing MessageBody".into()))?;
    
    let message_id = emulator.storage.send_message(queue_name, message_body)?;
    
    Ok(json!({
        "MD5OfMessageBody": "todo",
        "MessageId": message_id
    }))
}

async fn receive_message(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let queue_url = body["QueueUrl"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing QueueUrl".into()))?;
    let queue_name = queue_url.split('/').next_back().unwrap_or("");
    let max_messages = body["MaxNumberOfMessages"].as_i64().unwrap_or(1) as i32;
    
    let messages = emulator.storage.receive_message(queue_name, max_messages)?;
    
    let msg_list: Vec<Value> = messages.into_iter().map(|m| {
        json!({
            "MessageId": m.id,
            "ReceiptHandle": m.receipt_handle,
            "Body": m.body,
            "MD5OfBody": m.md5_body.unwrap_or_else(|| "todo".to_string()),
        })
    }).collect();
    
    Ok(json!({
        "Messages": msg_list
    }))
}

async fn delete_message(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let queue_url = body["QueueUrl"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing QueueUrl".into()))?;
    let queue_name = queue_url.split('/').next_back().unwrap_or("");
    let receipt_handle = body["ReceiptHandle"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing ReceiptHandle".into()))?;
    
    emulator.storage.delete_message(queue_name, receipt_handle)?;
    
    Ok(json!({}))
}

async fn list_queues(_emulator: &Emulator, _body: Value) -> Result<Value, EmulatorError> {
    // Basic implementation for now
    Ok(json!({
        "QueueUrls": []
    }))
}
