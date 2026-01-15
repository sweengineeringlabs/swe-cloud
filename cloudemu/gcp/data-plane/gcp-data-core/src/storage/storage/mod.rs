//! Storage engine - SQLite metadata + filesystem objects

mod engine;
mod schema;
mod s3;
mod dynamodb;
mod kms;
mod events;
mod secrets;
mod sqs;
mod sns;
mod lambda;
mod monitoring;
mod identity;
mod workflows;
mod gcs;
mod firestore;
mod pubsub;

pub use engine::{
    StorageEngine, BucketMetadata, ObjectMetadata, ListObjectsResult,
    SecretMetadata, SecretValue, KmsKeyMetadata,
    EventBusMetadata, EventRuleMetadata, EventTargetMetadata,
    MetricMetadata, LogGroupMetadata, LogStreamMetadata, LogEventMetadata,
    UserPoolMetadata, UserGroupMetadata, UserMetadata,
    StateMachineMetadata, ExecutionMetadata,
    QueueMetadata, MessageMetadata,
    TableMetadata, ItemMetadata,
    TopicMetadata, SubscriptionMetadata, LambdaMetadata,
    // GCP
    GcsBucketMetadata, GcsObjectMetadata,
    FirestoreDatabaseMetadata, FirestoreDocumentMetadata,
    PubSubTopicMetadata, PubSubSubscriptionMetadata
};

pub use lambda::CreateFunctionParams;
