use cloudemu::{Emulator, gateway::create_router};
use cloudkit::prelude::*;
use cloudkit_aws::AwsBuilder;


use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::test]
async fn test_sdk_emulator_s3_integration() {
    // 1. Setup Emulator on a random port
    let emulator = Arc::new(Emulator::in_memory().unwrap());
    let app = create_router(emulator.clone());
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let endpoint = format!("http://{}", addr);

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // 2. Setup SDK to point to the local emulator
    let client = AwsBuilder::new()
        .region(Region::aws_us_east_1())
        .config(CloudConfig::builder()
            .endpoint(endpoint)
            .build()
            .unwrap())
        .build()
        .await
        .expect("Failed to build AWS client");

    let storage = client.storage();

    // 3. Test SDK -> Emulator flow
    let bucket = "test-bucket";
    let key = "test-key";
    let content = b"Hello from CloudKit SDK to CloudEmu!";

    // Create bucket
    storage.create_bucket(bucket).await.expect("Failed to create bucket");

    // Put object
    storage.put_object(bucket, key, content).await.expect("Failed to put object");

    // Get object
    let result = storage.get_object(bucket, key).await.expect("Failed to get object");
    assert_eq!(&result[..], content);

    // List objects
    let objects_result = storage.list_objects(bucket, ListOptions::default()).await.expect("Failed to list objects");
    assert_eq!(objects_result.items.len(), 1);
    assert_eq!(objects_result.items[0].key, key);

    // Delete object
    storage.delete_object(bucket, key).await.expect("Failed to delete object");

    // Verify deletion
    let objects_after = storage.list_objects(bucket, ListOptions::default()).await.expect("Failed to list objects after delete");
    assert!(objects_after.items.is_empty());

    // Delete bucket
    storage.delete_bucket(bucket).await.expect("Failed to delete bucket");
}
