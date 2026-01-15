//! Storage engine - SQLite metadata + filesystem objects

mod engine;
mod schema;
// AWS modules removed (s3, dynamodb, etc)
mod gcs;
mod firestore;
mod pubsub;
mod compute;

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
    PubSubTopicMetadata, PubSubSubscriptionMetadata,
    GcpInstanceMetadata
};


