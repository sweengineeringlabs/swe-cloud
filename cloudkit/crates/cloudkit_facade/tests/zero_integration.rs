use cloudkit::CloudKit;
use cloudkit_spi::{ProviderType, Region};
use cloudkit_zero::ZeroBuilder;

#[tokio::test]
async fn test_zero_context_integration() {
    // Test generic factory
    let context = CloudKit::zero()
        .region(Region::zero_local())
        .build()
        .await
        .expect("Failed to build context");

    assert_eq!(context.provider(), ProviderType::Zero);
    assert_eq!(context.region().code(), "local");
}

#[tokio::test]
async fn test_zero_builder_integration() {
    // Test specific provider builder
    let client = ZeroBuilder::new()
        .region(Region::zero_default())
        .endpoint("http://localhost:8080")
        .build()
        .await
        .expect("Failed to build Zero client");

    assert_eq!(client.context().provider(), ProviderType::Zero);
    assert_eq!(client.context().region().code(), "default");
    
    // Check we can access services
    let _store = client.storage();
    let _db = client.kv_store();
    let _queue = client.queue();
    let _func = client.functions();
    let _iam = client.identity();
}

#[tokio::test]
async fn test_zero_cloudemu_integration() {
    // Test factory via cloudemu method
    let context = CloudKit::cloudemu(ProviderType::Zero)
        .build()
        .await
        .expect("Failed to build context");

    assert_eq!(context.provider(), ProviderType::Zero);
}
