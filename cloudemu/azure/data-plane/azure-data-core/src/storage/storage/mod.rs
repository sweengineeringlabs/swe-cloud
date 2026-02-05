//! Storage engine - SQLite metadata + filesystem objects

mod engine;
mod schema;
mod blob;
mod cosmos;
mod eventgrid;
mod compute;
mod secrets;
mod dynamodb;
mod lambda;
mod sqs;
mod pricing;
mod identity;
mod dns;
mod monitoring;
pub mod logicapps;
pub mod apimanagement;
pub mod loadbalancer;
pub mod redis;
pub mod acr;
pub mod sql;
pub mod networking;
pub mod containers;

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
    VirtualMachineMetadata,
};

pub use identity::{ServicePrincipal, RoleAssignment};
pub use dns::{DnsZone, RecordSet};
pub use logicapps::LogicApp;
pub use pricing::{Product, OfferTerm};
pub use lambda::CreateFunctionParams;
pub use sql::AzureSqlDatabase;
pub use networking::{VirtualNetwork, Subnet};
pub use containers::ContainerGroup;
