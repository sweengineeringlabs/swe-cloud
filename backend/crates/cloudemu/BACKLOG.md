# CloudEmu Backlog

This document tracks planned features and enhancements for CloudEmu.

## Status Legend

| Symbol | Meaning |
|--------|---------|
| ⬜ | Not started |
| 🔄 | In progress |
| ✅ | Complete |
| ❌ | Blocked/Cancelled |

---

## Phase 1: Core S3 (COMPLETE) ✅

| Feature | Status | Description |
|---------|--------|-------------|
| SQLite Storage Engine | ✅ | Metadata storage with SQLite |
| Filesystem Object Storage | ✅ | Content-addressed object storage |
| CreateBucket | ✅ | Create S3 buckets |
| DeleteBucket | ✅ | Delete empty buckets |
| HeadBucket | ✅ | Check bucket exists |
| ListBuckets | ✅ | List all buckets |
| GetBucketLocation | ✅ | Get bucket region |
| PutBucketVersioning | ✅ | Enable/suspend versioning |
| GetBucketVersioning | ✅ | Get versioning status |
| PutBucketPolicy | ✅ | Set bucket policy (JSON) |
| GetBucketPolicy | ✅ | Get bucket policy |
| DeleteBucketPolicy | ✅ | Remove bucket policy |
| PutObject | ✅ | Upload objects |
| GetObject | ✅ | Download objects |
| HeadObject | ✅ | Get object metadata |
| DeleteObject | ✅ | Delete objects (with delete markers) |
| ListObjectsV2 | ✅ | List objects with pagination |
| CopyObject | ✅ | Copy objects between buckets |
| Terraform Compatibility | ✅ | Works with aws_s3_bucket resources |

---

## Phase 2: Advanced S3

| Feature | Status | Priority | Description |
|---------|--------|----------|-------------|
| Multipart Upload | ✅ | High | Upload large files in parts |
| CreateMultipartUpload | ✅ | High | Initiate multipart upload |
| UploadPart | ✅ | High | Upload individual parts |
| CompleteMultipartUpload | ✅ | High | Finalize multipart upload |
| AbortMultipartUpload | ✅ | High | Cancel multipart upload |
| ListMultipartUploads | ⬜ | Medium | List in-progress uploads |
| ListParts | ⬜ | Medium | List uploaded parts |

---

## Phase 3: Lifecycle Rules

| Feature | Status | Priority | Description |
|---------|--------|----------|-------------|
| PutBucketLifecycleConfiguration | ⬜ | Medium | Set lifecycle rules |
| GetBucketLifecycleConfiguration | ⬜ | Medium | Get lifecycle rules |
| DeleteBucketLifecycle | ⬜ | Medium | Remove lifecycle rules |
| Lifecycle Background Processor | ⬜ | Medium | Auto-expire/transition objects |
| Expiration Rules | ⬜ | Medium | Delete objects after N days |
| Transition Rules | ⬜ | Low | Change storage class |
| NoncurrentVersionExpiration | ⬜ | Low | Delete old versions |

---

## Phase 6: DynamoDB (Core Complete) ✅

| Feature | Status | Priority | Description |
|---------|--------|----------|-------------|
| CreateTable | ✅ | High | Create DynamoDB tables |
| DeleteTable | ✅ | High | Delete tables |
| DescribeTable | ✅ | High | Get table info |
| ListTables | ✅ | High | List all tables |
| PutItem | ✅ | High | Insert/update items |
| GetItem | ✅ | High | Retrieve items |
| DeleteItem | ✅ | High | Remove items |
| Query | ✅ | High | Query by partition key |
| Scan | ✅ | Medium | Full table scan |
| UpdateItem | ⬜ | Medium | Partial updates |
| BatchGetItem | ⬜ | Medium | Batch read |
| BatchWriteItem | ⬜ | Medium | Batch write |
| TransactGetItems | ⬜ | Low | Transactional read |
| TransactWriteItems | ⬜ | Low | Transactional write |
| Global Secondary Indexes | ⬜ | Medium | GSI support |
| Local Secondary Indexes | ⬜ | Low | LSI support |
| DynamoDB Streams | ⬜ | Low | Change data capture |
| Terraform Compatibility | ✅ | High | aws_dynamodb_table |

---

## Phase 7: SQS (Core Complete) ✅

| Feature | Status | Priority | Description |
|---------|--------|----------|-------------|
| CreateQueue | ✅ | High | Create queues |
| DeleteQueue | ✅ | High | Delete queues |
| ListQueues | ✅ | High | List all queues |
| GetQueueUrl | ✅ | High | Get queue URL by name |
| GetQueueAttributes | ✅ | Medium | Get queue config |
| SetQueueAttributes | ✅ | Medium | Set queue config |
| SendMessage | ✅ | High | Send messages |
| ReceiveMessage | ✅ | High | Receive messages |
| DeleteMessage | ✅ | High | Delete processed messages |
| ChangeMessageVisibility | ⬜ | Medium | Extend visibility timeout |
| SendMessageBatch | ⬜ | Medium | Batch send |
| DeleteMessageBatch | ⬜ | Medium | Batch delete |
| PurgeQueue | ⬜ | Medium | Clear all messages |
| Dead Letter Queues | ⬜ | Medium | DLQ support |
| FIFO Queues | ⬜ | Low | Ordered, exactly-once |
| Message Delay | ⬜ | Low | Delayed delivery |
| Visibility Timeout | ⬜ | High | Auto-return to queue |
| Terraform Compatibility | ✅ | High | aws_sqs_queue |

---

## Phase 10: Architecture Refactor (COMPLETE) ✅

| Feature | Status | Priority | Description |
|---------|--------|----------|-------------|
| Data Plane Separation | ✅ | High | Move storage logic to `data-plane` crate |
| Control Plane Separation | ✅ | High | Move service logic to `control-plane` crate |
| Gateway Refactor | ✅ | High | Split Gateway/Ingress/Dispatcher |
| CloudEmu Shell Removal | ✅ | High | Consolidate binary into `control-plane` |
| Error Handling Refactor | ✅ | High | Decouple EmulatorError from Axum |

