use zero_sdk::ZeroClient;
use serde_json::json;

// Note: These tests assume a running ZeroCloud Control Plane at localhost:8080
// They are intended as integration tests.

#[tokio::test]
async fn test_store_workflow() {
    let client = ZeroClient::from_env();
    let bucket_name = format!("test-bucket-{}", uuid::Uuid::new_v4());
    
    // Create
    client.store().create_bucket(&bucket_name).await.ok(); // Ignore if exists
    
    // List
    let buckets = client.store().list_buckets().await.unwrap();
    assert!(buckets.contains(&bucket_name));
}

#[tokio::test]
async fn test_db_workflow() {
    let client = ZeroClient::from_env();
    let table_name = "sdk_test_table";
    
    // Create 
    client.db().create_table(table_name, "id").await.ok();
    
    // Put
    let item = json!({"id": "1", "data": "test"});
    client.db().put_item(table_name, "1", item).await.unwrap();
    
    // List
    let tables = client.db().list_tables().await.unwrap();
    assert!(tables.contains(&table_name.to_string()));
}

#[tokio::test]
async fn test_queue_workflow() {
    let client = ZeroClient::from_env();
    let q_name = "sdk_test_queue";
    
    // Create
    client.queue().create_queue(q_name).await.unwrap();
    
    // Send
    let body = "hello sdk";
    let _id = client.queue().send_message(q_name, body).await.unwrap();
    
    // Receive
    let msg = client.queue().receive_message(q_name).await.unwrap().expect("Should have message");
    assert_eq!(msg.body, body);
    
    // Delete
    client.queue().delete_message(q_name, &msg.receipt_handle).await.unwrap();
}

#[tokio::test]
async fn test_iam_workflow() {
    let client = ZeroClient::from_env();
    let user = format!("user-{}", uuid::Uuid::new_v4());
    
    client.iam().create_user(&user).await.unwrap();
    let users = client.iam().list_users().await.unwrap();
    
    let found = users.iter().any(|u| u["UserName"] == user);
    assert!(found);
}
