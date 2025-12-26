# AWS Provider

The AWS provider implements CloudKit traits using the AWS SDK for Rust.

## Installation

```toml
[dependencies]
cloudkit = "0.1"
cloudkit-aws = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Supported Services

| Service | Trait | Feature Flag |
|---------|-------|--------------|
| S3 | `ObjectStorage` | `s3` (default) |
| DynamoDB | `KeyValueStore` | `dynamodb` (default) |
| SQS | `MessageQueue` | `sqs` (default) |
| SNS | `PubSub` | `sns` (default) |
| Lambda | `Functions` | `lambda` (default) |

## Authentication

### Environment Variables (Recommended)

```bash
export AWS_ACCESS_KEY_ID=AKIA...
export AWS_SECRET_ACCESS_KEY=...
export AWS_REGION=us-east-1
```

### AWS Profile

```rust
use cloudkit_aws::AwsBuilder;

let aws = AwsBuilder::new()
    .profile("my-profile")  // Uses ~/.aws/credentials
    .build()
    .await?;
```

### IAM Role (EC2/ECS/Lambda)

When running on AWS infrastructure, credentials are automatically obtained.

```rust
use cloudkit_aws::AwsBuilder;

let aws = AwsBuilder::new()
    .region(Region::aws_us_east_1())
    .build()
    .await?;  // Uses instance role
```

### Custom Credentials

```rust
use cloudkit::prelude::*;
use cloudkit::spi::StaticAuthProvider;
use cloudkit_aws::AwsBuilder;

let creds = Credentials::new("access-key", "secret-key");
let aws = AwsBuilder::new()
    .region(Region::aws_us_east_1())
    // Custom auth provider
    .build()
    .await?;
```

## S3 Object Storage

### Basic Operations

```rust
use cloudkit_aws::AwsBuilder;
use cloudkit::prelude::*;

#[tokio::main]
async fn main() -> CloudResult<()> {
    let aws = AwsBuilder::new()
        .region(Region::aws_us_east_1())
        .build()
        .await?;
    
    let storage = aws.storage();

    // Upload
    storage.put_object("bucket", "file.txt", b"Hello!").await?;

    // Download
    let data = storage.get_object("bucket", "file.txt").await?;

    // Delete
    storage.delete_object("bucket", "file.txt").await?;

    Ok(())
}
```

### Upload with Options

```rust
let options = PutOptions::new()
    .content_type("application/json")
    .cache_control("max-age=86400")
    .metadata("x-custom-header", "value");

storage.put_object_with_options("bucket", "data.json", json_bytes, options).await?;
```

### List Objects

```rust
let options = ListOptions::new()
    .prefix("folder/")
    .max_results(100);

let result = storage.list_objects("bucket", options).await?;

for obj in result.items {
    println!("{}: {} bytes, modified {:?}", obj.key, obj.size, obj.last_modified);
}

// Handle pagination
if result.has_more() {
    let next_options = ListOptions::new()
        .prefix("folder/")
        .continuation_token(result.next_token.0.unwrap());
    
    let more = storage.list_objects("bucket", next_options).await?;
}
```

### Presigned URLs

```rust
use std::time::Duration;

// Download URL (valid for 1 hour)
let download_url = storage
    .presigned_get_url("bucket", "file.txt", Duration::from_secs(3600))
    .await?;

// Upload URL
let upload_url = storage
    .presigned_put_url("bucket", "new-file.txt", Duration::from_secs(3600))
    .await?;
```

## DynamoDB Key-Value Store

### Basic Operations

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    email: String,
}

let kv = aws.kv_store();

// Put item
let user = User {
    id: "user-123".to_string(),
    name: "John Doe".to_string(),
    email: "john@example.com".to_string(),
};
kv.put("users", &user.id, &user).await?;

// Get item
let retrieved: Option<User> = kv.get("users", "user-123").await?;

// Delete item
kv.delete("users", "user-123").await?;
```

### Query with Conditions

```rust
use cloudkit::api::KvQueryOptions;

let options = KvQueryOptions {
    limit: Some(10),
    scan_forward: true,
    ..Default::default()
};

let users: ListResult<User> = kv.query("users", "company-abc", options).await?;
```

## SQS Message Queue

### Send and Receive

```rust
let queue = aws.queue();

// Get queue URL
let queue_url = queue.get_queue_url("my-queue").await?;

// Send message
let msg_id = queue.send(&queue_url, "Hello from CloudKit!").await?;

// Receive messages
let messages = queue.receive(&queue_url, ReceiveOptions::new().max_messages(10)).await?;

for msg in &messages {
    println!("Message: {}", msg.body);
    // Process message...
    
    // Delete after processing
    queue.delete(&queue_url, msg).await?;
}
```

### FIFO Queues

```rust
let options = SendOptions::new()
    .message_group_id("order-processing")
    .deduplication_id("order-123");

queue.send_with_options(&queue_url, "Order data", options).await?;
```

## SNS Pub/Sub

### Publish Messages

```rust
let pubsub = aws.pubsub();

// Get topic ARN
let topic_arn = pubsub.get_topic_arn("my-topic").await?;

// Publish
pubsub.publish(&topic_arn, b"Event occurred!").await?;

// Publish JSON
#[derive(Serialize)]
struct Event { event_type: String, data: String }

let event = Event {
    event_type: "user.created".to_string(),
    data: "user-123".to_string(),
};
pubsub.publish_json(&topic_arn, &event).await?;
```

### Subscribe

```rust
// Email subscription
pubsub.subscribe(&topic_arn, "email", "user@example.com").await?;

// HTTPS webhook
pubsub.subscribe(&topic_arn, "https", "https://api.example.com/webhook").await?;

// SQS queue
pubsub.subscribe(&topic_arn, "sqs", "arn:aws:sqs:us-east-1:123456789012:my-queue").await?;
```

## Lambda Functions

### Invoke Functions

```rust
let functions = aws.functions();

// Synchronous invocation
let result = functions.invoke("my-function", b"{}").await?;

if result.is_success() {
    if let Some(payload) = result.payload {
        println!("Response: {}", String::from_utf8_lossy(&payload));
    }
}
```

### Invoke with JSON

```rust
#[derive(Serialize)]
struct Request { name: String }

#[derive(Deserialize)]
struct Response { greeting: String }

let request = Request { name: "World".to_string() };
let response: Response = functions.invoke_json("hello-function", &request).await?;

println!("{}", response.greeting);
```

### Async Invocation

```rust
// Fire and forget
functions.invoke_async("background-job", b"task-data").await?;
```

## Regions

Available AWS regions:

```rust
use cloudkit::common::Region;

Region::aws_us_east_1()      // US East (N. Virginia)
Region::aws_us_east_2()      // US East (Ohio)
Region::aws_us_west_1()      // US West (N. California)
Region::aws_us_west_2()      // US West (Oregon)
Region::aws_eu_west_1()      // EU (Ireland)
Region::aws_eu_central_1()   // EU (Frankfurt)
Region::aws_af_south_1()     // Africa (Cape Town)
```

## Error Handling

AWS-specific error codes:

```rust
match error {
    CloudError::Provider { provider, code, message } if provider == "aws" => {
        match code.as_str() {
            "NoSuchBucket" => println!("Bucket doesn't exist"),
            "NoSuchKey" => println!("Object doesn't exist"),
            "AccessDenied" => println!("Permission denied"),
            "BucketAlreadyExists" => println!("Bucket name taken"),
            "ServiceException" => println!("AWS service error: {}", message),
            _ => println!("AWS error {}: {}", code, message),
        }
    }
    _ => {}
}
```

## Best Practices

1. **Use Instance Roles** - When running on AWS, use IAM roles instead of credentials
2. **Enable Transfer Acceleration** - For large S3 uploads across regions
3. **Use Batch Operations** - For multiple items, use `delete_objects` and `batch_put`
4. **Configure Retries** - AWS has automatic retry built-in, but configure for your needs
5. **Monitor with CloudWatch** - Use the metrics SPI to send custom metrics
