//! Main CloudKit entry point.

use cloudkit_spi::CloudConfig;
use cloudkit_spi::{CloudContextBuilder, ProviderType};

/// Main entry point for CloudKit.
///
/// Use this struct to create cloud clients for different providers.
///
/// # Example
///
/// ```rust,ignore
/// use cloudkit::CloudKit;
/// use cloudkit::common::Region;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Create a cloud context for AWS
///     let context = CloudKit::aws()
///         .region(Region::aws_us_east_1())
///         .build()
///         .await?;
///
///     println!("Provider: {}", context.provider());
///
///     Ok(())
/// }
/// ```
///
/// # Provider Crates
///
/// For full provider functionality, use the provider crates directly:
///
/// ```rust,ignore
/// use cloudkit_aws::AwsBuilder;
///
/// let aws = AwsBuilder::new()
///     .region(Region::aws_us_east_1())
///     .build()
///     .await?;
///
/// // Now you can access services
/// aws.storage().put_object("bucket", "key", b"data").await?;
/// ```
pub struct CloudKit;

impl CloudKit {
    /// Create an AWS context builder.
    ///
    /// For full AWS functionality, use `cloudkit-aws` crate directly.
    pub fn aws() -> CloudContextBuilder {
        CloudContextBuilder::new(ProviderType::Aws)
    }

    /// Create an Azure context builder.
    ///
    /// For full Azure functionality, use `cloudkit-azure` crate directly.
    pub fn azure() -> CloudContextBuilder {
        CloudContextBuilder::new(ProviderType::Azure)
    }

    /// Create a GCP context builder.
    ///
    /// For full GCP functionality, use `cloudkit-gcp` crate directly.
    pub fn gcp() -> CloudContextBuilder {
        CloudContextBuilder::new(ProviderType::Gcp)
    }

    /// Create an Oracle Cloud context builder.
    ///
    /// For full Oracle Cloud functionality, use `cloudkit-oracle` crate directly.
    pub fn oracle() -> CloudContextBuilder {
        CloudContextBuilder::new(ProviderType::Oracle)
    }

    /// Create a client from configuration.
    pub fn from_config(provider: ProviderType, config: CloudConfig) -> CloudContextBuilder {
        CloudContextBuilder::new(provider).config(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cloudkit_aws() {
        let context = CloudKit::aws().build().await.unwrap();
        assert_eq!(context.provider(), ProviderType::Aws);
    }

    #[tokio::test]
    async fn test_cloudkit_azure() {
        let context = CloudKit::azure().build().await.unwrap();
        assert_eq!(context.provider(), ProviderType::Azure);
    }

    #[tokio::test]
    async fn test_cloudkit_gcp() {
        let context = CloudKit::gcp().build().await.unwrap();
        assert_eq!(context.provider(), ProviderType::Gcp);
    }

    #[tokio::test]
    async fn test_cloudkit_oracle() {
        let context = CloudKit::oracle().build().await.unwrap();
        assert_eq!(context.provider(), ProviderType::Oracle);
    }
}
