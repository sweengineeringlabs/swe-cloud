# 03 - API Design

## Document Information

| Field | Value |
|-------|-------|
| **Version** | 1.0.0 |
| **Status** | Design Complete |
| **Last Updated** | 2025-12-26 |

---

## 1. Service Traits Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                      CloudKit API Layer                          │
│                                                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                    Service Traits                          │  │
│  │                                                            │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │  │
│  │  │ Object      │  │ KeyValue    │  │ Message     │        │  │
│  │  │ Storage     │  │ Store       │  │ Queue       │        │  │
│  │  │             │  │             │  │             │        │  │
│  │  │ • put       │  │ • get       │  │ • send      │        │  │
│  │  │ • get       │  │ • put       │  │ • receive   │        │  │
│  │  │ • delete    │  │ • delete    │  │ • delete    │        │  │
│  │  │ • list      │  │ • query     │  │ • purge     │        │  │
│  │  │ • copy      │  │ • batch     │  │             │        │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘        │  │
│  │                                                            │  │
│  │  ┌─────────────┐  ┌─────────────┐                         │  │
│  │  │ PubSub      │  │ Functions   │                         │  │
│  │  │             │  │             │                         │  │
│  │  │ • publish   │  │ • invoke    │                         │  │
│  │  │ • subscribe │  │ • invoke    │                         │  │
│  │  │ • unsubscr  │  │   _async    │                         │  │
│  │  │ • list      │  │ • list      │                         │  │
│  │  └─────────────┘  └─────────────┘                         │  │
│  │                                                            │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. ObjectStorage Trait

### Purpose
Abstract blob/object storage across cloud providers.

### Provider Mapping

| CloudKit | AWS | Azure | GCP | Oracle |
|----------|-----|-------|-----|--------|
| ObjectStorage | S3 | Blob Storage | Cloud Storage | Object Storage |
| bucket | bucket | container | bucket | bucket |
| object/key | object | blob | object | object |

### Trait Definition

```rust
#[async_trait]
pub trait ObjectStorage: Send + Sync {
    // Bucket Operations
    async fn list_buckets(&self) -> CloudResult<Vec<BucketMetadata>>;
    async fn create_bucket(&self, bucket: &str) -> CloudResult<()>;
    async fn delete_bucket(&self, bucket: &str) -> CloudResult<()>;
    async fn bucket_exists(&self, bucket: &str) -> CloudResult<bool>;

    // Object Operations
    async fn put_object(&self, bucket: &str, key: &str, data: &[u8]) -> CloudResult<()>;
    async fn put_object_with_options(
        &self, bucket: &str, key: &str, data: &[u8], options: PutOptions
    ) -> CloudResult<()>;
    async fn get_object(&self, bucket: &str, key: &str) -> CloudResult<Bytes>;
    async fn get_object_with_options(
        &self, bucket: &str, key: &str, options: GetOptions
    ) -> CloudResult<Bytes>;
    async fn head_object(&self, bucket: &str, key: &str) -> CloudResult<ObjectMetadata>;
    async fn delete_object(&self, bucket: &str, key: &str) -> CloudResult<()>;
    async fn delete_objects(&self, bucket: &str, keys: &[&str]) -> CloudResult<()>;
    async fn copy_object(
        &self, src_bucket: &str, src_key: &str, dst_bucket: &str, dst_key: &str
    ) -> CloudResult<()>;
    async fn object_exists(&self, bucket: &str, key: &str) -> CloudResult<bool>;
    async fn list_objects(
        &self, bucket: &str, options: ListOptions
    ) -> CloudResult<ListResult<ObjectMetadata>>;

    // Presigned URLs
    async fn presigned_get_url(
        &self, bucket: &str, key: &str, expires_in: Duration
    ) -> CloudResult<String>;
    async fn presigned_put_url(
        &self, bucket: &str, key: &str, expires_in: Duration
    ) -> CloudResult<String>;
}
```

### Operation Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    put_object Flow                               │
│                                                                  │
│   Application                                                    │
│       │                                                          │
│       ▼                                                          │
│   storage.put_object("bucket", "key", data)                     │
│       │                                                          │
│       ▼                                                          │
│   ┌─────────────────────────────────────────┐                   │
│   │           CloudKit Layer                 │                   │
│   │                                          │                   │
│   │  1. Validate parameters                  │                   │
│   │  2. Get credentials from AuthProvider   │                   │
│   │  3. Wrap in OperationExecutor           │                   │
│   └─────────────────────────────────────────┘                   │
│       │                                                          │
│       ▼                                                          │
│   ┌─────────────────────────────────────────┐                   │
│   │         Provider SDK                     │                   │
│   │                                          │                   │
│   │  AWS: client.put_object().send()        │                   │
│   │  Azure: blob_client.upload()             │                   │
│   │  GCP: client.upload_object()             │                   │
│   └─────────────────────────────────────────┘                   │
│       │                                                          │
│       ▼                                                          │
│   ┌─────────────────────────────────────────┐                   │
│   │         Cloud Provider API               │                   │
│   │                                          │                   │
│   │  HTTPS request to cloud endpoint         │                   │
│   └─────────────────────────────────────────┘                   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 3. KeyValueStore Trait

### Purpose
Abstract NoSQL key-value operations.

### Provider Mapping

| CloudKit | AWS | Azure | GCP | Oracle |
|----------|-----|-------|-----|--------|
| KeyValueStore | DynamoDB | Cosmos DB | Firestore | NoSQL |
| table | table | container | collection | table |
| key | partition key | id | document id | key |

### Trait Definition

```rust
#[async_trait]
pub trait KeyValueStore: Send + Sync {
    async fn get<T: DeserializeOwned + Send>(
        &self, table: &str, key: &str
    ) -> CloudResult<Option<T>>;
    
    async fn get_with_options<T: DeserializeOwned + Send>(
        &self, table: &str, key: &str, options: KvGetOptions
    ) -> CloudResult<Option<T>>;
    
    async fn put<T: Serialize + Send + Sync>(
        &self, table: &str, key: &str, item: &T
    ) -> CloudResult<()>;
    
    async fn put_with_options<T: Serialize + Send + Sync>(
        &self, table: &str, key: &str, item: &T, options: KvPutOptions
    ) -> CloudResult<Option<serde_json::Value>>;
    
    async fn delete(&self, table: &str, key: &str) -> CloudResult<()>;
    
    async fn delete_with_condition(
        &self, table: &str, key: &str, condition: Condition
    ) -> CloudResult<bool>;
    
    async fn exists(&self, table: &str, key: &str) -> CloudResult<bool>;
    
    async fn update(
        &self, table: &str, key: &str, updates: HashMap<String, serde_json::Value>
    ) -> CloudResult<()>;
    
    async fn query<T: DeserializeOwned + Send>(
        &self, table: &str, partition_key: &str, options: KvQueryOptions
    ) -> CloudResult<ListResult<T>>;
    
    async fn batch_get<T: DeserializeOwned + Send>(
        &self, table: &str, keys: &[&str]
    ) -> CloudResult<Vec<T>>;
    
    async fn batch_write<T: Serialize + Send + Sync>(
        &self, table: &str, items: &[(&str, &T)]
    ) -> CloudResult<()>;
}
```

---

## 4. MessageQueue Trait

### Purpose
Abstract queue operations for async messaging.

### Provider Mapping

| CloudKit | AWS | Azure | GCP | Oracle |
|----------|-----|-------|-----|--------|
| MessageQueue | SQS | Service Bus Queue | Cloud Tasks | Streaming |
| queue | queue | queue | queue | stream |
| message | message | message | task | message |

### Trait Definition

```rust
#[async_trait]
pub trait MessageQueue: Send + Sync {
    // Queue Management
    async fn create_queue(&self, name: &str) -> CloudResult<String>;
    async fn delete_queue(&self, queue_url: &str) -> CloudResult<()>;
    async fn get_queue_url(&self, name: &str) -> CloudResult<String>;
    async fn list_queues(&self, prefix: Option<&str>) -> CloudResult<Vec<String>>;

    // Message Operations
    async fn send(&self, queue_url: &str, body: &str) -> CloudResult<ResourceId>;
    async fn send_with_options(
        &self, queue_url: &str, body: &str, options: SendOptions
    ) -> CloudResult<ResourceId>;
    async fn send_batch(
        &self, queue_url: &str, messages: &[&str]
    ) -> CloudResult<Vec<ResourceId>>;
    async fn receive(
        &self, queue_url: &str, options: ReceiveOptions
    ) -> CloudResult<Vec<Message>>;
    async fn delete(&self, queue_url: &str, message: &Message) -> CloudResult<()>;
    async fn delete_batch(&self, queue_url: &str, messages: &[&Message]) -> CloudResult<()>;
    async fn change_visibility(
        &self, queue_url: &str, message: &Message, timeout: Duration
    ) -> CloudResult<()>;
    async fn get_queue_depth(&self, queue_url: &str) -> CloudResult<u64>;
    async fn purge(&self, queue_url: &str) -> CloudResult<()>;
}
```

### Message Lifecycle

```
┌─────────────────────────────────────────────────────────────────┐
│                    Message Lifecycle                             │
│                                                                  │
│   Producer                          Queue                        │
│      │                                │                          │
│      │ send("message")               │                          │
│      ├──────────────────────────────►│                          │
│      │                                │                          │
│      │                                │◄─── Message stored       │
│      │                                │                          │
│                                       │     Consumer             │
│                                       │        │                 │
│                                       │        │ receive()       │
│                                       │◄───────┤                 │
│                                       │        │                 │
│                        Message ───────┼───────►│                 │
│                        (invisible)    │        │                 │
│                                       │        │ process...      │
│                                       │        │                 │
│                                       │        │ delete()        │
│                                       │◄───────┤                 │
│                                       │        │                 │
│                        Deleted ───────┼        │                 │
│                                       │        │                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 5. PubSub Trait

### Purpose
Abstract publish-subscribe messaging.

### Provider Mapping

| CloudKit | AWS | Azure | GCP |
|----------|-----|-------|-----|
| PubSub | SNS | Event Grid | Pub/Sub |
| topic | topic | topic | topic |
| subscription | subscription | subscription | subscription |

### Trait Definition

```rust
#[async_trait]
pub trait PubSub: Send + Sync {
    // Topic Management
    async fn create_topic(&self, name: &str) -> CloudResult<String>;
    async fn delete_topic(&self, topic_arn: &str) -> CloudResult<()>;
    async fn list_topics(&self) -> CloudResult<Vec<String>>;
    async fn get_topic_arn(&self, name: &str) -> CloudResult<String>;

    // Subscription Management
    async fn subscribe(
        &self, topic_arn: &str, protocol: &str, endpoint: &str
    ) -> CloudResult<String>;
    async fn subscribe_with_config(
        &self, topic_arn: &str, config: SubscriptionConfig
    ) -> CloudResult<String>;
    async fn unsubscribe(&self, subscription_arn: &str) -> CloudResult<()>;
    async fn list_subscriptions(&self, topic_arn: &str) -> CloudResult<Vec<String>>;

    // Publishing
    async fn publish(&self, topic_arn: &str, message: &[u8]) -> CloudResult<ResourceId>;
    async fn publish_with_attributes(
        &self, topic_arn: &str, message: &[u8], attributes: HashMap<String, String>
    ) -> CloudResult<ResourceId>;
    async fn publish_batch(
        &self, topic_arn: &str, messages: &[&[u8]]
    ) -> CloudResult<Vec<ResourceId>>;
    async fn publish_json<T: Serialize + Send + Sync>(
        &self, topic_arn: &str, message: &T
    ) -> CloudResult<ResourceId>;
}
```

---

## 6. Functions Trait

### Purpose
Abstract serverless function invocation.

### Provider Mapping

| CloudKit | AWS | Azure | GCP | Oracle |
|----------|-----|-------|-----|--------|
| Functions | Lambda | Functions | Cloud Functions | Functions |
| function | function | function | function | function |

### Trait Definition

```rust
#[async_trait]
pub trait Functions: Send + Sync {
    async fn invoke(
        &self, function_name: &str, payload: &[u8]
    ) -> CloudResult<InvokeResult>;
    
    async fn invoke_with_options(
        &self, function_name: &str, payload: &[u8], options: InvokeOptions
    ) -> CloudResult<InvokeResult>;
    
    async fn invoke_json<T, R>(&self, function_name: &str, payload: &T) -> CloudResult<R>
    where
        T: Serialize + Send + Sync,
        R: DeserializeOwned + Send;
    
    async fn invoke_async(&self, function_name: &str, payload: &[u8]) -> CloudResult<()>;
    
    async fn list_functions(&self) -> CloudResult<Vec<String>>;
    
    async fn function_exists(&self, function_name: &str) -> CloudResult<bool>;
}
```

---

## 7. Options and Types

### Common Options

```
┌─────────────────────────────────────────────────────────────────┐
│                     Options Builder Pattern                      │
│                                                                  │
│   PutOptions::new()                                              │
│       .content_type("application/json")                         │
│       .cache_control("max-age=3600")                            │
│       .metadata("key", "value")                                 │
│                                                                  │
│   ListOptions::new()                                             │
│       .prefix("folder/")                                        │
│       .delimiter("/")                                           │
│       .max_results(100)                                         │
│                                                                  │
│   ReceiveOptions::new()                                          │
│       .max_messages(10)                                         │
│       .visibility_timeout(Duration::from_secs(30))              │
│       .wait_time(Duration::from_secs(20))                       │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 8. Related Documents

- [02-architecture.md](02-architecture.md) - Architecture details
- [04-provider-integration.md](04-provider-integration.md) - Provider SDKs
- [05-error-handling.md](05-error-handling.md) - Error types
