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
| Multipart Upload | ⬜ | High | Upload large files in parts |
| CreateMultipartUpload | ⬜ | High | Initiate multipart upload |
| UploadPart | ⬜ | High | Upload individual parts |
| CompleteMultipartUpload | ⬜ | High | Finalize multipart upload |
| AbortMultipartUpload | ⬜ | High | Cancel multipart upload |
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

## Phase 4: CORS

| Feature | Status | Priority | Description |
|---------|--------|----------|-------------|
| PutBucketCors | ⬜ | Medium | Set CORS configuration |
| GetBucketCors | ⬜ | Medium | Get CORS configuration |
| DeleteBucketCors | ⬜ | Medium | Remove CORS configuration |
| CORS Preflight Handling | ⬜ | Medium | Handle OPTIONS requests |

---

## Phase 5: Presigned URLs

| Feature | Status | Priority | Description |
|---------|--------|----------|-------------|
| Presigned GET URLs | ⬜ | High | Temporary download links |
| Presigned PUT URLs | ⬜ | High | Temporary upload links |
| URL Expiration | ⬜ | High | Time-limited access |
| Signature Validation | ⬜ | High | Verify AWS Signature V4 |

---

## Phase 6: DynamoDB

| Feature | Status | Priority | Description |
|---------|--------|----------|-------------|
| CreateTable | ⬜ | High | Create DynamoDB tables |
| DeleteTable | ⬜ | High | Delete tables |
| DescribeTable | ⬜ | High | Get table info |
| ListTables | ⬜ | High | List all tables |
| PutItem | ⬜ | High | Insert/update items |
| GetItem | ⬜ | High | Retrieve items |
| DeleteItem | ⬜ | High | Remove items |
| Query | ⬜ | High | Query by partition key |
| Scan | ⬜ | Medium | Full table scan |
| UpdateItem | ⬜ | Medium | Partial updates |
| BatchGetItem | ⬜ | Medium | Batch read |
| BatchWriteItem | ⬜ | Medium | Batch write |
| TransactGetItems | ⬜ | Low | Transactional read |
| TransactWriteItems | ⬜ | Low | Transactional write |
| Global Secondary Indexes | ⬜ | Medium | GSI support |
| Local Secondary Indexes | ⬜ | Low | LSI support |
| DynamoDB Streams | ⬜ | Low | Change data capture |
| Terraform Compatibility | ⬜ | High | aws_dynamodb_table |

---

## Phase 7: SQS

| Feature | Status | Priority | Description |
|---------|--------|----------|-------------|
| CreateQueue | ⬜ | High | Create queues |
| DeleteQueue | ⬜ | High | Delete queues |
| ListQueues | ⬜ | High | List all queues |
| GetQueueUrl | ⬜ | High | Get queue URL by name |
| GetQueueAttributes | ⬜ | Medium | Get queue config |
| SetQueueAttributes | ⬜ | Medium | Set queue config |
| SendMessage | ⬜ | High | Send messages |
| ReceiveMessage | ⬜ | High | Receive messages |
| DeleteMessage | ⬜ | High | Delete processed messages |
| ChangeMessageVisibility | ⬜ | Medium | Extend visibility timeout |
| SendMessageBatch | ⬜ | Medium | Batch send |
| DeleteMessageBatch | ⬜ | Medium | Batch delete |
| PurgeQueue | ⬜ | Medium | Clear all messages |
| Dead Letter Queues | ⬜ | Medium | DLQ support |
| FIFO Queues | ⬜ | Low | Ordered, exactly-once |
| Message Delay | ⬜ | Low | Delayed delivery |
| Visibility Timeout | ⬜ | High | Auto-return to queue |
| Terraform Compatibility | ⬜ | High | aws_sqs_queue |

---

## Phase 8: Additional Services (Future)

| Service | Status | Priority | Description |
|---------|--------|----------|-------------|
| SNS | ⬜ | Low | Pub/Sub messaging |
| Lambda | ⬜ | Low | Serverless functions (Docker) |
| Secrets Manager | ⬜ | Low | Secret storage |
| Parameter Store | ⬜ | Low | Configuration storage |
| EventBridge | ⬜ | Low | Event routing |

---

## Infrastructure & Quality

| Feature | Status | Priority | Description |
|---------|--------|----------|-------------|
| AWS Signature V4 Validation | ⬜ | Low | Authenticate requests |
| Request Logging (CloudTrail-like) | ⬜ | Medium | Audit trail |
| Docker Image | ⬜ | Medium | Containerized deployment |
| GitHub Actions CI | ⬜ | Medium | Automated testing |
| Integration Tests | ⬜ | High | AWS SDK tests |
| Performance Benchmarks | ⬜ | Low | Speed testing |
| Documentation Site | ⬜ | Low | Full API docs |

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2025-12-26 | Initial S3 implementation |

---

## Notes

- All S3 operations should return exact AWS XML/JSON format
- Terraform compatibility is a top priority
- Focus on accuracy over performance initially
- DynamoDB and SQS are major additions after S3 is complete
