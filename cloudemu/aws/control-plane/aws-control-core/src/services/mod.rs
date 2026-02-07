//! Cloud service implementations

#[cfg(feature = "s3")]
pub mod s3;

#[cfg(feature = "dynamodb")]
pub mod dynamodb;

#[cfg(feature = "sqs")]
pub mod sqs;

#[cfg(feature = "sns")]
pub mod sns;

#[cfg(feature = "lambda")]
pub mod lambda;

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

#[cfg(feature = "ec2")]
pub mod ec2;

#[cfg(feature = "ecs")]
pub mod ecs;

#[cfg(feature = "rds")]
pub mod rds;

#[cfg(feature = "iam")]
pub mod iam;

#[cfg(feature = "route53")]
pub mod route53;

#[cfg(feature = "pricing")]
pub mod pricing;

#[cfg(feature = "apigateway")]
pub mod apigateway;

#[cfg(feature = "elb")]
pub mod elb;

#[cfg(feature = "elasticache")]
pub mod elasticache;

#[cfg(feature = "ecr")]
pub mod ecr;
