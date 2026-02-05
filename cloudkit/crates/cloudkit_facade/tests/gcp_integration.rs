use cloudkit::CloudKit;
use cloudkit_spi::{ProviderType, Region};
// use cloudkit_gcp::GcpBuilder; // Assuming explicit builder or manual construct

#[tokio::test]
async fn test_gcp_context_integration() {
    let context = CloudKit::gcp()
        .region(Region::gcp_us_central1())
        .build()
        .await
        .expect("Failed to build context");

    assert_eq!(context.provider(), ProviderType::Gcp);
    assert_eq!(context.region().code(), "us-central1");
}

// TODO: update this once GcpBuilder is fully standardized like AwsBuilder
// #[tokio::test]
// async fn test_gcp_client_integration() {
//     ...
// }
