//! Cloud service implementations

#[cfg(feature = "s3")]
pub mod s3;

#[cfg(feature = "dynamodb")]
pub mod dynamodb;

#[cfg(feature = "sqs")]
pub mod sqs;

#[cfg(feature = "secretsmanager")]
pub mod secrets;

#[cfg(feature = "eventbridge")]
pub mod events;

#[cfg(feature = "kms")]
pub mod kms;

#[cfg(feature = "cloudwatch")]
pub mod monitoring;

#[cfg(feature = "cognito")]
pub mod identity;

#[cfg(feature = "stepfunctions")]
pub mod workflows;
