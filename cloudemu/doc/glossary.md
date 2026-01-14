# CloudEmu Glossary

This glossary defines key terms used throughout the CloudEmu project.

## Core Concepts

**CloudEmu**  
A production-grade local AWS service emulator that provides accurate API responses for development and testing workflows.

**Control Plane**  
The high-level orchestration layer responsible for HTTP routing, request dispatching, and service coordination (`crates/control-plane`).

**Data Plane**  
The low-level persistence layer that handles metadata storage (SQLite), blob storage (filesystem), and configuration (`crates/data-plane`).

**Emulation**  
The process of replicating AWS service behavior locally, including request parsing, validation, and response generation.

**Endpoint**  
The HTTP address where CloudEmu listens for AWS API requests (default: `http://localhost:4566`).

## AWS Service Terms

**S3 (Simple Storage Service)**  
Object storage service for storing and retrieving files (buckets and objects).

**DynamoDB**  
NoSQL key-value and document database service.

**SQS (Simple Queue Service)**  
Message queuing service for decoupling distributed systems.

**SNS (Simple Notification Service)**  
Pub/Sub messaging service for event-driven architectures.

**Lambda**  
Serverless compute service for running code without managing servers.

**Secrets Manager**  
Secure storage for API keys, passwords, and other sensitive data.

**KMS (Key Management Service)**  
Cryptographic key creation, encryption, and signing service.

**EventBridge**  
Event bus for routing application events to targets.

**CloudWatch**  
Monitoring and logging service for metrics and log streams.

**Cognito**  
User authentication and identity management service.

**Step Functions**  
Workflow orchestration service for coordinating distributed applications.

## Architecture Terms

**Dispatcher**  
The routing component that inspects AWS-specific headers (e.g., `x-amz-target`) and directs requests to the appropriate service handler.

**Service Handler**  
The module responsible for implementing a specific AWS service's logic (e.g., `services/s3/`, `services/dynamodb/`).

**Storage Engine**  
The unified persistence backend that stores metadata in SQLite and blobs in the filesystem.

**Terraform Compatibility**  
The ability to use CloudEmu as an AWS endpoint in Terraform configurations for local infrastructure testing.

**SDK Compatibility**  
The ability to use CloudEmu with official AWS SDKs (Rust, Python, JavaScript, etc.) by overriding the endpoint URL.

---

**Last Updated**: 2026-01-14  
**Version**: 1.0
