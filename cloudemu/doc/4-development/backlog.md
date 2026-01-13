# CloudEmu Development Backlog

**Last Updated:** 2026-01-13  
**Version:** 0.1.0

This document tracks the development backlog for CloudEmu, organized by priority and implementation phase.

---

## Priority Legend

| Symbol | Priority | Description |
|--------|----------|-------------|
| 🔴 | P0 - Critical | Blockers that must be fixed immediately |
| 🟡 | P1 - High | Important features needed for production readiness |
| 🟢 | P2 - Medium | Nice-to-have improvements |
| 🔵 | P3 - Low | Future enhancements |

## Status Legend

| Symbol | Status | Description |
|--------|--------|-------------|
| ⬜ | Not Started | No work has begun |
| 🔄 | In Progress | Currently being worked on |
| ✅ | Complete | Fully implemented and tested |
| ❌ | Blocked | Cannot proceed due to dependency |
| 🚧 | Partial | Partially implemented |

---

## P0 - Critical Issues (Must Fix)

### 1. 🔴 Fix Test Suite
**Status:** 🔄 In Progress  
**Priority:** P0  
**Estimated Effort:** 2-3 days

**Issues:**
- Tests fail to compile due to file locking errors
- SQLite database conflicts when running multiple tests
- Only 4 integration tests exist

**Tasks:**
- [x] Identify root cause of file locking
- [ ] Ensure all tests use in-memory SQLite
- [ ] Add test cleanup/teardown
- [ ] Expand test coverage to 50+ tests
- [ ] Add CI/CD test automation

**Acceptance Criteria:**
- All tests pass on `cargo test`
- Tests can run in parallel
- Test coverage >60%

---

### 2. 🔴 Address Clippy Warnings
**Status:** 🔄 In Progress  
**Priority:** P0  
**Estimated Effort:** 1 day

**Issues:**
- 15 clippy warnings in codebase
- Functions with too many arguments (8/7)
- Unused functions
- Inefficient iterator usage

**Tasks:**
- [ ] Fix "too many arguments" in `create_function` (use builder pattern)
- [ ] Replace `.last()` with `.next_back()` in dispatch.rs
- [ ] Remove unused `create_bucket_xml` and `delete_objects_xml`
- [ ] Fix unused imports in tests
- [ ] Run `cargo clippy --fix --allow-dirty`

**Acceptance Criteria:**
- `cargo clippy --all-features` runs with 0 warnings
- Code follows Rust best practices

---

### 3. 🔴 Refactor Storage Engine
**Status:** 🔄 In Progress  
**Priority:** P0  
**Estimated Effort:** 3-5 days

**Issues:**
- `storage/engine.rs` is 1,717 lines (monolith)
- All AWS services in one file
- Difficult to maintain and test
- Poor separation of concerns

**Tasks:**
- [ ] Create modular storage architecture
- [ ] Extract S3 storage to `storage/s3.rs`
- [ ] Extract DynamoDB storage to `storage/dynamodb.rs`
- [ ] Extract SQS storage to `storage/sqs.rs`
- [ ] Extract other services to separate files
- [ ] Create storage traits for consistency
- [ ] Update tests to use new modules

**Proposed Structure:**
```
storage/
├── mod.rs              # Main storage interface
├── engine.rs           # Core storage engine (< 300 lines)
├── schema.rs           # Database schema
├── s3.rs               # S3 storage operations
├── dynamodb.rs         # DynamoDB storage
├── sqs.rs              # SQS storage
├── kms.rs              # KMS storage
├── secrets.rs          # Secrets Manager storage
└── ...                 # Other services
```

**Acceptance Criteria:**
- Each storage module < 500 lines
- Clear trait-based interfaces
- All existing tests still pass
- Better code organization

---

### 4. 🔴 Complete DynamoDB Implementation
**Status:** 🔄 In Progress  
**Priority:** P0  
**Estimated Effort:** 4-6 days

**Issues:**
- Missing Query and Scan operations (critical!)
- No UpdateItem or DeleteItem
- No batch operations
- Hardcoded mock responses

**Tasks:**
- [ ] Implement `Query` operation with partition key filtering
- [ ] Implement `Scan` operation with full table scan
- [ ] Implement `UpdateItem` with expression support
- [ ] Implement `DeleteItem`
- [ ] Add `BatchGetItem` and `BatchWriteItem`
- [ ] Support FilterExpression and ProjectionExpression
- [ ] Add proper table metadata retrieval
- [ ] Remove hardcoded timestamps
- [ ] Add comprehensive DynamoDB tests

**Acceptance Criteria:**
- Query and Scan operations work correctly
- All basic CRUD operations implemented
- Compatible with AWS DynamoDB SDK
- 80% test coverage for DynamoDB

---

## P1 - High Priority Features

### 5. 🟡 Implement S3 Multipart Upload
**Status:** ⬜ Not Started  
**Priority:** P1  
**Estimated Effort:** 3-4 days

**Description:**
S3 multipart upload is essential for files >5GB and commonly used in production.

**Tasks:**
- [ ] Implement `CreateMultipartUpload`
- [ ] Implement `UploadPart`
- [ ] Implement `CompleteMultipartUpload`
- [ ] Implement `AbortMultipartUpload`
- [ ] Implement `ListMultipartUploads`
- [ ] Implement `ListParts`
- [ ] Add multipart storage in SQLite
- [ ] Add cleanup for abandoned uploads
- [ ] Add integration tests

**Acceptance Criteria:**
- Can upload files >100MB in parts
- Works with AWS SDK multipart upload
- Properly handles part ETags
- Cleanup on abort works correctly

---

### 6. 🟡 Expand Test Coverage
**Status:** ⬜ Not Started  
**Priority:** P1  
**Estimated Effort:** 5-7 days

**Current Coverage:**
- Integration tests: 4 tests
- Unit tests: 0
- HTTP tests: 0

**Target Coverage:**
- Integration tests: 50+
- Unit tests: 100+
- HTTP tests: 30+
- Overall coverage: >75%

**Tasks:**
- [ ] Add HTTP-level S3 integration tests
- [ ] Add S3 versioning tests
- [ ] Add S3 policy tests
- [ ] Add DynamoDB integration tests
- [ ] Add SQS integration tests
- [ ] Add unit tests for XML generation
- [ ] Add unit tests for storage operations
- [ ] Add error path tests
- [ ] Add concurrent access tests
- [ ] Set up code coverage reporting

---

### 7. 🟡 Remove Hardcoded Values
**Status:** ⬜ Not Started  
**Priority:** P1  
**Estimated Effort:** 1-2 days

**Issues:**
- Hardcoded timestamps (e.g., `1234567890.0`)
- Hardcoded MD5 values ("todo")
- Mock return values in handlers

**Tasks:**
- [ ] Use `chrono::Utc::now()` for timestamps
- [ ] Calculate actual MD5 hashes for messages
- [ ] Remove mock responses from handlers
- [ ] Add proper UUID generation
- [ ] Validate all dynamic values

---

### 8. 🟡 Improve Error Handling
**Status:** ⬜ Not Started  
**Priority:** P1  
**Estimated Effort:** 2-3 days

**Tasks:**
- [ ] Add input validation for all operations
- [ ] Return proper AWS error codes
- [ ] Add error response XML/JSON formatting
- [ ] Handle edge cases (empty strings, null values)
- [ ] Add request ID tracking
- [ ] Improve error messages

---

### 9. 🟡 Complete SQS Features
**Status:** 🚧 Partial  
**Priority:** P1  
**Estimated Effort:** 3-4 days

**Missing Features:**
- GetQueueAttributes / SetQueueAttributes
- ChangeMessageVisibility
- Dead Letter Queues
- FIFO queues
- Message attributes
- Long polling

**Tasks:**
- [ ] Implement queue attributes
- [ ] Implement visibility timeout modification
- [ ] Add DLQ support
- [ ] Add FIFO queue support
- [ ] Add message attributes
- [ ] Implement long polling

---

## P2 - Medium Priority Enhancements

### 10. 🟢 Add Performance Benchmarks
**Status:** ⬜ Not Started  
**Priority:** P2  
**Estimated Effort:** 3-4 days

**Tasks:**
- [ ] Set up Criterion benchmarks
- [ ] Benchmark S3 operations
- [ ] Benchmark DynamoDB operations
- [ ] Benchmark storage engine
- [ ] Profile under load
- [ ] Document performance characteristics
- [ ] Add performance regression tests

---

### 11. 🟢 Security Hardening
**Status:** ⬜ Not Started  
**Priority:** P2  
**Estimated Effort:** 5-7 days

**Tasks:**
- [ ] Add AWS Signature V4 validation (optional)
- [ ] Implement basic authentication
- [ ] Add rate limiting
- [ ] Input sanitization audit
- [ ] Add security documentation
- [ ] Add HTTPS support
- [ ] Security audit

---

### 12. 🟢 S3 Advanced Features
**Status:** ⬜ Not Started  
**Priority:** P2  
**Estimated Effort:** 7-10 days

**Tasks:**
- [ ] Implement CORS configuration
- [ ] Implement lifecycle rules
- [ ] Implement presigned URLs
- [ ] Add range requests
- [ ] Add server-side encryption
- [ ] Add bucket notifications
- [ ] Add replication configuration

---

### 13. 🟢 DynamoDB Advanced Features
**Status:** ⬜ Not Started  
**Priority:** P2  
**Estimated Effort:** 10-14 days

**Tasks:**
- [ ] Implement Global Secondary Indexes (GSI)
- [ ] Implement Local Secondary Indexes (LSI)
- [ ] Add TransactGetItems / TransactWriteItems
- [ ] Implement DynamoDB Streams
- [ ] Add TTL support
- [ ] Implement conditional expressions
- [ ] Add point-in-time recovery

---

### 14. 🟢 Complete Lambda Service
**Status:** 🚧 Partial  
**Priority:** P2  
**Estimated Effort:** 14-21 days

**Tasks:**
- [ ] Implement function code storage
- [ ] Add Docker-based execution
- [ ] Implement environment variables
- [ ] Add layer support
- [ ] Implement async invocation
- [ ] Add event source mappings
- [ ] Integration with SQS, SNS, S3

---

### 15. 🟢 Monitoring & Observability
**Status:** ⬜ Not Started  
**Priority:** P2  
**Estimated Effort:** 3-5 days

**Tasks:**
- [ ] Add structured logging
- [ ] Add metrics collection
- [ ] Add request tracing
- [ ] Add health check endpoint improvements
- [ ] Add CloudWatch metrics emulation
- [ ] Add log aggregation

---

## P3 - Low Priority / Future Enhancements

### 16. 🔵 Additional AWS Services
**Status:** ⬜ Not Started  
**Priority:** P3  

**Services to Add:**
- [ ] ECS (Container Service)
- [ ] RDS (Relational Database)
- [ ] ElastiCache (Redis/Memcached)
- [ ] API Gateway
- [ ] CloudFormation
- [ ] IAM (Identity and Access Management)

---

### 17. 🔵 Developer Experience
**Status:** ⬜ Not Started  
**Priority:** P3  

**Tasks:**
- [ ] Add Docker image
- [ ] Add Docker Compose examples
- [ ] Create CLI tool for management
- [ ] Add web UI for monitoring
- [ ] Improve startup banner
- [ ] Add configuration presets

---

### 18. 🔵 Documentation Improvements
**Status:** ⬜ Not Started  
**Priority:** P3  

**Tasks:**
- [ ] Create API documentation site
- [ ] Add architecture diagrams
- [ ] Add video tutorials
- [ ] Create migration guide from LocalStack
- [ ] Add troubleshooting guide
- [ ] Document performance tuning

---

### 19. 🔵 Cross-Cloud Support
**Status:** ⬜ Not Started  
**Priority:** P3  

**Tasks:**
- [ ] Add Azure Blob Storage emulation
- [ ] Add Google Cloud Storage emulation
- [ ] Multi-cloud abstraction layer
- [ ] Cloud provider compatibility matrix

---

## Completed Items ✅

### ✅ Phase 1: Core S3 (v0.1.0)
- ✅ SQLite Storage Engine
- ✅ Filesystem Object Storage
- ✅ CreateBucket / DeleteBucket / HeadBucket / ListBuckets
- ✅ PutBucketVersioning / GetBucketVersioning
- ✅ PutBucketPolicy / GetBucketPolicy / DeleteBucketPolicy
- ✅ PutObject / GetObject / HeadObject / DeleteObject
- ✅ ListObjectsV2
- ✅ CopyObject
- ✅ Terraform Compatibility
- ✅ AWS SDK Compatibility

---

## Sprint Planning

### Current Sprint (Week of 2026-01-13)

**Focus:** P0 Critical Issues

| Task | Assignee | Status | Due Date |
|------|----------|--------|----------|
| Fix Test Suite | - | 🔄 | 2026-01-15 |
| Address Clippy Warnings | - | 🔄 | 2026-01-14 |
| Refactor Storage Engine | - | 🔄 | 2026-01-17 |
| Complete DynamoDB | - | 🔄 | 2026-01-20 |

### Next Sprint (Week of 2026-01-20)

**Focus:** P1 High Priority Features

| Task | Assignee | Status | Due Date |
|------|----------|--------|----------|
| S3 Multipart Upload | - | ⬜ | 2026-01-24 |
| Expand Test Coverage | - | ⬜ | 2026-01-27 |
| Remove Hardcoded Values | - | ⬜ | 2026-01-22 |
| Improve Error Handling | - | ⬜ | 2026-01-24 |

---

## Progress Tracking

### Velocity Metrics

| Metric | Current | Target |
|--------|---------|--------|
| Test Coverage | ~10% | 75% |
| Clippy Warnings | 15 | 0 |
| Service Completeness | 25% | 80% |
| Documentation | 70% | 90% |
| Performance (req/s) | Unknown | 1000+ |

### Burn Down

**Total P0-P1 Items:** 9  
**Completed:** 0  
**In Progress:** 4  
**Remaining:** 5

**Estimated Completion:** February 2026

---

## Notes

- All tasks should include comprehensive tests
- Focus on production-readiness, not feature count
- Maintain backward compatibility
- Document all breaking changes
- Follow Rust best practices and idioms

---

**Maintained by:** CloudEmu Development Team  
**Review Frequency:** Weekly  
**Next Review:** 2026-01-20
