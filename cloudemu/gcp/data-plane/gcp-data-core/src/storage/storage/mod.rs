//! Storage engine - SQLite metadata + filesystem objects

mod engine;
mod schema;
// AWS modules removed (s3, dynamodb, etc)
mod gcs;
mod firestore;
mod pubsub;
mod compute;
mod sql;
mod secrets;
mod lambda;
mod pricing;
mod iam;
mod dns;
mod monitoring;
mod workflows;
mod networking;
mod run;
mod kms;

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
    GcpInstanceMetadata,
};

pub use iam::ServiceAccount;
pub use dns::ManagedZone;
pub use workflows::Workflow;
pub use sql::GcpSqlInstance;

pub use pricing::{Product, OfferTerm};

pub use lambda::CreateFunctionParams;


