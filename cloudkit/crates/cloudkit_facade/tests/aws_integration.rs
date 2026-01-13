use cloudkit::CloudKit;
use cloudkit_spi::{ProviderType, Region};
use cloudkit_aws::AwsBuilder;

#[tokio::test]
async fn test_aws_context_integration() {
    // Test generic factory
    let context = CloudKit::aws()
        .region(Region::aws_us_east_1())
        .build()
        .await
        .expect("Failed to build context");

    assert_eq!(context.provider(), ProviderType::Aws);
    assert_eq!(context.region().code(), "us-east-1");
}

#[tokio::test]
async fn test_aws_builder_integration() {
    // Test specific provider builder
    let client = AwsBuilder::new()
        .region(Region::aws_us_west_2())
        .profile("test-profile")
        .build()
        .await
        .expect("Failed to build AWS client");

    assert_eq!(client.context().provider(), ProviderType::Aws);
    assert_eq!(client.context().region().code(), "us-west-2");
    assert_eq!(client.profile(), Some("test-profile"));
    
    // Check we can access services (these calls create internal service clients)
    let _s3 = client.storage();
    let _ddb = client.kv_store();
    let _sqs = client.queue();
}
