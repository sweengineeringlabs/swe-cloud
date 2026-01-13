# Provider Implementations

This section documents how each cloud provider implements CloudKit traits.

## Provider Comparison

| Feature | AWS | Azure | GCP | Oracle |
|---------|-----|-------|-----|--------|
| **Object Storage** | S3 | Blob Storage | Cloud Storage | Object Storage |
| **Key-Value Store** | DynamoDB | Cosmos DB | Firestore | NoSQL |
| **Message Queue** | SQS | Service Bus Queue | Cloud Tasks | Streaming |
| **Pub/Sub** | SNS | Event Grid | Pub/Sub | - |
| **Functions** | Lambda | Functions | Cloud Functions | Functions |

## Provider Documentation

- [AWS Provider](aws.md) - Amazon Web Services
- [Azure Provider](azure.md) - Microsoft Azure
- [GCP Provider](gcp.md) - Google Cloud Platform
- [Oracle Provider](oracle.md) - Oracle Cloud Infrastructure

## Choosing a Provider

### AWS

**Best for:**
- Well-established cloud infrastructure
- Widest range of services
- Strong enterprise support
- Extensive documentation and community

**Considerations:**
- Complex pricing model
- Steep learning curve
- Some services are region-specific

### Azure

**Best for:**
- Microsoft ecosystem integration
- Enterprise Windows environments
- Hybrid cloud scenarios
- Strong compliance certifications

**Considerations:**
- Naming conventions can be confusing
- Portal can be slow
- Some SDK maturity gaps

### GCP

**Best for:**
- Big data and ML workloads
- Kubernetes (GKE)
- Developer experience
- Competitive pricing

**Considerations:**
- Smaller service catalog
- Fewer regions than AWS
- Enterprise support tier required for some features

### Oracle Cloud

**Best for:**
- Oracle database workloads
- Cost-effective compute
- Strong South African presence
- Autonomous database features

**Considerations:**
- Smaller community
- Less SDK maturity in Rust
- Fewer third-party integrations

## Service Mapping

### Object Storage

| Operation | AWS S3 | Azure Blob | GCS | OCI |
|-----------|--------|------------|-----|-----|
| `put_object` | PutObject | Upload | Objects.insert | PutObject |
| `get_object` | GetObject | Download | Objects.get | GetObject |
| `delete_object` | DeleteObject | Delete | Objects.delete | DeleteObject |
| `list_objects` | ListObjectsV2 | List | Objects.list | ListObjects |
| `head_object` | HeadObject | GetProperties | Objects.get (metadata) | HeadObject |

### Key-Value Store

| Operation | DynamoDB | Cosmos | Firestore | OCI NoSQL |
|-----------|----------|--------|-----------|-----------|
| `put` | PutItem | Upsert | Set | Put |
| `get` | GetItem | Read | Get | Get |
| `delete` | DeleteItem | Delete | Delete | Delete |
| `query` | Query | Query | Where | Query |

## Feature Flags

Each provider crate has feature flags to minimize binary size:

### cloudkit-aws

```toml
[features]
default = ["s3", "dynamodb", "sqs", "sns", "lambda"]
s3 = []
dynamodb = []
sqs = []
sns = []
lambda = []
```

### cloudkit-azure

```toml
[features]
default = ["blob", "cosmos"]
blob = []
cosmos = []
service-bus = []
functions = []
```

### cloudkit-gcp

```toml
[features]
default = ["gcs", "pubsub"]
gcs = []
pubsub = []
firestore = []
functions = []
```

### cloudkit-oracle

```toml
[features]
default = ["object-storage"]
object-storage = []
nosql = []
streaming = []
functions = []
```

## Cross-Provider Patterns

### Provider Abstraction

```rust
use cloudkit::prelude::*;

enum CloudProvider {
    Aws(cloudkit_aws::AwsClient),
    Azure(cloudkit_azure::AzureClient),
    Gcp(cloudkit_gcp::GcpClient),
    Oracle(cloudkit_oracle::OracleClient),
}

impl CloudProvider {
    fn storage(&self) -> Box<dyn ObjectStorage> {
        match self {
            Self::Aws(client) => Box::new(client.storage()),
            Self::Azure(client) => Box::new(client.storage()),
            Self::Gcp(client) => Box::new(client.storage()),
            Self::Oracle(client) => Box::new(client.storage()),
        }
    }
}
```

### Multi-Cloud Deployment

```rust
async fn replicate_to_all<S: ObjectStorage>(
    storages: &[(&str, &S)],
    bucket: &str,
    key: &str,
    data: &[u8],
) -> Vec<(&str, CloudResult<()>)> {
    let mut results = vec![];
    
    for (name, storage) in storages {
        let result = storage.put_object(bucket, key, data).await;
        results.push((*name, result));
    }
    
    results
}
```

## Testing with Multiple Providers

```rust
#[cfg(test)]
mod tests {
    use cloudkit::prelude::*;
    
    // Test with mock storage
    async fn test_with_storage<S: ObjectStorage>(storage: &S) {
        storage.put_object("test-bucket", "test-key", b"data").await.unwrap();
        let data = storage.get_object("test-bucket", "test-key").await.unwrap();
        assert_eq!(&data[..], b"data");
    }
    
    #[tokio::test]
    #[ignore] // Requires AWS credentials
    async fn test_aws() {
        let aws = cloudkit_aws::AwsBuilder::new().build().await.unwrap();
        test_with_storage(&aws.storage()).await;
    }
    
    #[tokio::test]
    #[ignore] // Requires Azure credentials
    async fn test_azure() {
        let azure = cloudkit_azure::AzureBuilder::new().build().await.unwrap();
        test_with_storage(&azure.storage()).await;
    }
}
```
