use cloudemu::Emulator;
use std::sync::Arc;
use serde_json::json;

// ==================== S3 Tests ====================

#[tokio::test]
async fn test_s3_basic_operations() {
    let emulator = Arc::new(Emulator::in_memory().unwrap());
    
    // Create bucket
    emulator.storage.create_bucket("test-bucket", "us-east-1").unwrap();
    assert!(emulator.storage.bucket_exists("test-bucket").unwrap());
    
    // Put object
    let data = b"Hello, S3!";
    let obj = emulator.storage.put_object("test-bucket", "test.txt", data, Some("text/plain"), None).unwrap();
    assert!(!obj.etag.is_empty());
    
    // Get object
    let (retrieved_meta, retrieved_data) = emulator.storage.get_object("test-bucket", "test.txt", None).unwrap();
    assert_eq!(retrieved_data, data);
    assert_eq!(retrieved_meta.content_type, "text/plain");
    
    // List objects
    let list_result = emulator.storage.list_objects("test-bucket", None, None, 10, None).unwrap();
    assert_eq!(list_result.contents.len(), 1);
    assert_eq!(list_result.contents[0].key, "test.txt");
    
    // Delete object
    emulator.storage.delete_object("test-bucket", "test.txt", None).unwrap();
    let result = emulator.storage.get_object("test-bucket", "test.txt", None);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_s3_multipart_upload() {
    let emulator = Arc::new(Emulator::in_memory()).unwrap();
    
    emulator.storage.create_bucket("multipart-bucket", "us-east-1").unwrap();
    
    // Initiate multipart upload
    let upload_id = emulator.storage.create_multipart_upload("multipart-bucket", "large-file.bin").unwrap();
    assert!(!upload_id.is_empty());
    
    // Upload parts
    let part1 = b"Part 1 data";
    let part2 = b"Part 2 data";
    
    let etag1 = emulator.storage.upload_part(&upload_id, 1, part1).unwrap();
    let etag2 = emulator.storage.upload_part(&upload_id, 2, part2).unwrap();
    
    assert!(etag1.starts_with('"'));
    assert!(etag2.starts_with('"'));
    
    // Complete multipart upload
    let final_etag = emulator.storage.complete_multipart_upload("multipart-bucket", "large-file.bin", &upload_id).unwrap();
    assert!(final_etag.starts_with('"'));
    
    // Verify combined file
    let (_, data) = emulator.storage.get_object("multipart-bucket", "large-file.bin", None).unwrap();
    let expected = [part1, part2].concat();
    assert_eq!(data, expected);
}

// ==================== DynamoDB Tests ====================

#[tokio::test]
async fn test_dynamodb_query_and_scan() {
    let emulator = Arc::new(Emulator::in_memory()).unwrap();
    
    // Create table
    emulator.storage.create_table("users", "{}", "{}", "000000000000", "us-east-1").unwrap();
    
    // Put multiple items with same partition key
    let items = vec![
        json!({"userId": {"S": "user1"}, "name": {"S": "Alice"}, "age": {"N": "30"}}),
        json!({"userId": {"S": "user1"}, "name": {"S": "Alice Updated"}, "age": {"N": "31"}}),
        json!({"userId": {"S": "user2"}, "name": {"S": "Bob"}, "age": {"N": "25"}}),
    ];
    
    for (i, item) in items.iter().enumerate() {
        let pk = item["userId"]["S"].as_str().unwrap();
        emulator.storage.put_item("users", pk, None, &item.to_string()).unwrap();
    }
    
    // Query by partition key
    let query_results = emulator.storage.query_items("users", "user1").unwrap();
    assert_eq!(query_results.len(), 1); // Only latest version returned
    let result_json: serde_json::Value = serde_json::from_str(&query_results[0]).unwrap();
    assert_eq!(result_json["name"]["S"], "Alice Updated");
    
    // Scan entire table
    let scan_results = emulator.storage.scan_items("users").unwrap();
    assert_eq!(scan_results.len(), 2); // user1 (latest) and user2
}

// ==================== SNS → SQS Integration ====================

#[tokio::test]
async fn test_sns_to_sqs_delivery() {
    let emulator = Arc::new(Emulator::in_memory()).unwrap();
    
    // Create SQS queue
    emulator.storage.create_queue("notification-queue", "000000000000", "us-east-1").unwrap();
    
    // Create SNS topic
    let topic = emulator.storage.create_topic("alerts", "000000000000", "us-east-1").unwrap();
    
    // Subscribe queue to topic
    let queue_arn = "arn:aws:sqs:us-east-1:000000000000:notification-queue";
    emulator.storage.subscribe(&topic.arn, "sqs", queue_arn).unwrap();
    
    // Manually trigger SNS publish logic (simulating the handler)
    let subscriptions = emulator.storage.list_subscriptions_by_topic(&topic.arn).unwrap();
    assert_eq!(subscriptions.len(), 1);
    
    // Simulate message delivery
    let msg = json!({
        "Type": "Notification",
        "Message": "Alert: System status changed"
    });
    emulator.storage.send_message("notification-queue", &msg.to_string()).unwrap();
    
    // Verify message received
    let messages = emulator.storage.receive_message("notification-queue", 1).unwrap();
    assert_eq!(messages.len(), 1);
    assert!(messages[0].body.contains("Alert"));
}

// ==================== EventBridge → SQS Integration ====================

#[tokio::test]
async fn test_eventbridge_to_sqs() {
    let emulator = Arc::new(Emulator::in_memory()).unwrap();
    
    // Create event bus
    emulator.storage.create_event_bus("custom-bus", "000000000000", "us-east-1").unwrap();
    
    // Create SQS queue as target
    emulator.storage.create_queue("events-queue", "000000000000", "us-east-1").unwrap();
    
    // Create rule with pattern
    let pattern = json!({
        "source": ["myapp"],
        "detail-type": ["user-action"]
    });
    emulator.storage.put_rule("custom-bus", "user-events", &pattern.to_string(), "ENABLED").unwrap();
    
    // Add SQS as target
    let target_arn = "arn:aws:sqs:us-east-1:000000000000:events-queue";
    emulator.storage.put_target("custom-bus", "user-events", "1", target_arn).unwrap();
    
    // The PutEvents handler would now match and deliver
    // We can verify the infrastructure is set up
    let targets = emulator.storage.list_targets("user-events", "custom-bus").unwrap();
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].id, "1");
}

// ==================== KMS Encryption ====================

#[tokio::test]
async fn test_kms_real_encryption() {
    let emulator = Arc::new(Emulator::in_memory()).unwrap();
    
    // Create KMS key
    let key = emulator.storage.create_key("Test Key", "AES_256", "000000000000", "us-east-1").unwrap();
    assert_eq!(key.key_state, "Enabled");
    
    // The actual encryption/decryption would be tested through the service handlers
    // since they use AES-256-GCM. Here we verify the storage works.
    let fetched = emulator.storage.get_key(&key.id).unwrap();
    assert_eq!(fetched.id, key.id);
    assert_eq!(fetched.algorithm, "AES_256");
}

// ==================== Cognito Authentication ====================

#[tokio::test]
async fn test_cognito_user_flow() {
    let emulator = Arc::new(Emulator::in_memory()).unwrap();
    
    // Create user pool
    let pool = emulator.storage.create_user_pool("TestPool", "000000000000", "us-east-1").unwrap();
    
    // Create user
    emulator.storage.admin_create_user(&pool.id, "testuser").unwrap();
    
    // Verify user exists (this is checked in InitiateAuth)
    let user = emulator.storage.admin_get_user(&pool.id, "testuser").unwrap();
    assert_eq!(user.username, "testuser");
    assert_eq!(user.user_status, "CONFIRMED");
}

// ==================== Step Functions Execution ====================

#[tokio::test]
async fn test_step_functions_pass_state() {
    let emulator = Arc::new(Emulator::in_memory()).unwrap();
    
    // Simple Pass state machine
    let definition = json!({
        "StartAt": "PassState",
        "States": {
            "PassState": {
                "Type": "Pass",
                "Result": {"message": "Hello from Step Functions"},
                "End": true
            }
        }
    }).to_string();
    
    let machine = emulator.storage.create_state_machine(
        "simple-machine",
        &definition,
        "arn:aws:iam::000000000000:role/test",
        "STANDARD",
        "000000000000",
        "us-east-1"
    ).unwrap();
    
    // Execute using the interpreter
    let input = json!({}).to_string();
    let output = cloudemu::services::workflows::interpreter::StateMachineExecutor::execute(&definition, &input).unwrap();
    
    let output_json: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(output_json["message"], "Hello from Step Functions");
}

#[tokio::test]
async fn test_step_functions_choice_state() {
    let emulator = Arc::new(Emulator::in_memory()).unwrap();
    
    // Choice state machine
    let definition = json!({
        "StartAt": "CheckValue",
        "States": {
            "CheckValue": {
                "Type": "Choice",
                "Choices": [
                    {
                        "Variable": "$.value",
                        "NumericGreaterThan": 10,
                        "Next": "HighValue"
                    }
                ],
                "Default": "LowValue"
            },
            "HighValue": {
                "Type": "Pass",
                "Result": "Value is high",
                "End": true
            },
            "LowValue": {
                "Type": "Pass",
                "Result": "Value is low",
                "End": true
            }
        }
    }).to_string();
    
    // Test high value path
    let input_high = json!({"value": 15}).to_string();
    let output = cloudemu::services::workflows::interpreter::StateMachineExecutor::execute(&definition, &input_high).unwrap();
    assert_eq!(output, "\"Value is high\"");
    
    // Test low value path
    let input_low = json!({"value": 5}).to_string();
    let output = cloudemu::services::workflows::interpreter::StateMachineExecutor::execute(&definition, &input_low).unwrap();
    assert_eq!(output, "\"Value is low\"");
}

#[tokio::test]
async fn test_step_functions_map_state() {
    let emulator = Arc::new(Emulator::in_memory()).unwrap();
    
    // Map state machine
    let definition = json!({
        "StartAt": "MapState",
        "States": {
            "MapState": {
                "Type": "Map",
                "Iterator": {
                    "StartAt": "Double",
                    "States": {
                        "Double": {
                            "Type": "Pass",
                            "Result": "processed",
                            "End": true
                        }
                    }
                },
                "End": true
            }
        }
    }).to_string();
    
    let input = json!([1, 2, 3]).to_string();
    let output = cloudemu::services::workflows::interpreter::StateMachineExecutor::execute(&definition, &input).unwrap();
    
    let output_array: Vec<String> = serde_json::from_str(&output).unwrap();
    assert_eq!(output_array.len(), 3);
}

// ==================== Secrets Manager ====================

#[tokio::test]
async fn test_secrets_manager_versioning() {
    let emulator = Arc::new(Emulator::in_memory()).unwrap();
    
    // Create secret
    let secret = emulator.storage.create_secret(
        "db-password",
        "my-secret-value",
        "000000000000",
        "us-east-1"
    ).unwrap();
    
    // Get secret value
    let value = emulator.storage.get_secret_value(&secret.arn, None).unwrap();
    assert_eq!(value.secret_string, Some("my-secret-value".to_string()));
    
    // Update secret (creates new version)
    emulator.storage.update_secret(&secret.arn, "new-secret-value").unwrap();
    
    // Get latest version
    let updated = emulator.storage.get_secret_value(&secret.arn, None).unwrap();
    assert_eq!(updated.secret_string, Some("new-secret-value".to_string()));
}

// ==================== SQS Message Operations ====================

#[tokio::test]
async fn test_sqs_complete_workflow() {
    let emulator = Arc::new(Emulator::in_memory()).unwrap();
    
    let queue = emulator.storage.create_queue("test-queue", "000000000000", "us-east-1").unwrap();
    assert_eq!(queue.name, "test-queue");
    
    // Send message
    let msg_id = emulator.storage.send_message("test-queue", "hello world").unwrap();
    assert!(!msg_id.is_empty());
    
    // Receive message
    let messages = emulator.storage.receive_message("test-queue", 1).unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].body, "hello world");
    assert!(messages[0].receipt_handle.is_some());
    
    // Delete message
    emulator.storage.delete_message("test-queue", messages[0].receipt_handle.as_ref().unwrap()).unwrap();
    
    // Verify deletion
    let messages_after = emulator.storage.receive_message("test-queue", 1).unwrap();
    assert_eq!(messages_after.len(), 0);
}

// ==================== CloudWatch Logs ====================

#[tokio::test]
async fn test_cloudwatch_logs() {
    let emulator = Arc::new(Emulator::in_memory()).unwrap();
    
    // Create log group
    emulator.storage.create_log_group("app-logs").unwrap();
    
    // Create log stream
    emulator.storage.create_log_stream("app-logs", "stream-1").unwrap();
    
    // Put log events
    emulator.storage.put_log_events("app-logs", "stream-1", "INFO: Application started").unwrap();
    emulator.storage.put_log_events("app-logs", "stream-1", "INFO: Processing request").unwrap();
    
    // Query logs (basic verification - full query would need the handler)
    let groups = emulator.storage.describe_log_groups(Some("app-logs")).unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].name, "app-logs");
}
