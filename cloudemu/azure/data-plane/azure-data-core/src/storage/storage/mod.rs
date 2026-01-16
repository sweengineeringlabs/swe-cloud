//! Storage engine - SQLite metadata + filesystem objects

mod engine;
mod schema;
// AWS modules removed (s3, dynamodb, etc)
mod blob;
mod cosmos;
mod eventgrid;
mod compute;
mod secrets;
mod dynamodb;
mod lambda;
mod sqs;

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
    // Azure
    StorageAccountMetadata, BlobContainerMetadata, BlobMetadata,
    CosmosAccountMetadata, CosmosDatabaseMetadata, CosmosContainerMetadata, CosmosItemMetadata,
    EventGridTopicMetadata, EventGridSubscriptionMetadata,
    VirtualMachineMetadata
};

pub use lambda::CreateFunctionParams;


