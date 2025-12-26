# 12 - Infrastructure Emulation Guide

## Document Information

| Field | Value |
|-------|-------|
| **Version** | 1.0.0 |
| **Status** | Design Complete |
| **Last Updated** | 2025-12-26 |

---

## 1. Emulation Depth Spectrum

```
┌─────────────────────────────────────────────────────────────────┐
│                   Emulation Depth Spectrum                       │
│                                                                  │
│   SHALLOW                                              DEEP      │
│   ──────────────────────────────────────────────────────────    │
│                                                                  │
│   API Mock         Functional          Full Behavior            │
│   ─────────        ──────────          ─────────────            │
│   • Return OK      • Store data        • IAM policies           │
│   • Basic errors   • Real operations   • Billing simulation     │
│   • No state       • Persistence       • Network simulation     │
│                                                                  │
│   ┌──────────┐    ┌──────────┐        ┌──────────┐             │
│   │ CloudEmu │    │LocalStack│        │ Real AWS │             │
│   │ (current)│    │  (Pro)   │        │  (cloud) │             │
│   └──────────┘    └──────────┘        └──────────┘             │
│                                                                  │
│   Easy to build    Moderate effort     Impossible to fully     │
│   Good for tests   Good for dev        replicate locally       │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. What CAN Be Emulated

### ✅ Fully Emulatable

| Service | How It's Emulated |
|---------|-------------------|
| **S3** | HashMap/files for storage, metadata in memory |
| **DynamoDB** | In-memory key-value store, or SQLite |
| **SQS** | In-memory queue (VecDeque) |
| **SNS** | HTTP callbacks to local endpoints |
| **Secrets Manager** | Encrypted key-value store |
| **Parameter Store** | Simple key-value storage |

### ⚠️ Partially Emulatable

| Service | Challenge | Possible Approach |
|---------|-----------|-------------------|
| **Lambda** | Execute user code | Docker containers, WASM |
| **API Gateway** | Routing, auth | Proxy server |
| **Step Functions** | State machines | State machine engine |
| **EventBridge** | Event routing | Pub/sub with rules |
| **Kinesis** | Streaming | In-memory stream buffer |

### ❌ Difficult/Impossible to Fully Emulate

| Service | Why It's Hard |
|---------|---------------|
| **EC2** | Needs real VMs/containers |
| **VPC/Networking** | Requires network simulation |
| **IAM (full)** | Complex policy evaluation |
| **CloudFront** | Global CDN behavior |
| **RDS** | Need real database engine (use Docker) |
| **ElastiCache** | Use real Redis/Memcached locally |

---

## 3. Real S3 Infrastructure vs Emulation

```
┌─────────────────────────────────────────────────────────────────┐
│                    Real S3 Infrastructure                        │
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                     API Layer                            │   │
│   │  • Request validation  • Authentication                 │   │
│   │  • Rate limiting       • Request routing                │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                   Control Plane                          │   │
│   │  • Bucket policies     • Access control lists           │   │
│   │  • Versioning          • Lifecycle rules                │   │
│   │  • Replication         • Event notifications            │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                    Data Plane                            │   │
│   │  • Distributed storage • Data encryption                │   │
│   │  • 11 9's durability   • Cross-region replication       │   │
│   │  • Erasure coding      • Consistent hashing             │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                   Hardware Layer                         │   │
│   │  • Millions of drives  • Multiple data centers          │   │
│   │  • Custom hardware     • Network infrastructure         │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
│   What we CAN emulate:    API Layer + Basic Control Plane       │
│   What we SKIP:           Durability, replication, hardware     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 4. Enhanced CloudEmu Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                Enhanced CloudEmu Architecture                    │
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                   Gateway Layer                          │   │
│   │                                                          │   │
│   │  • AWS Signature V4 validation                          │   │
│   │  • Request throttling (simulate rate limits)            │   │
│   │  • Request logging (CloudTrail-like)                    │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                   IAM Layer                              │   │
│   │                                                          │   │
│   │  • Basic policy evaluation                               │   │
│   │  • User/role simulation                                  │   │
│   │  • Resource-based policies                               │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                 Service Layer                            │   │
│   │                                                          │   │
│   │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐       │   │
│   │  │   S3    │ │DynamoDB │ │   SQS   │ │ Lambda  │       │   │
│   │  │         │ │         │ │         │ │         │       │   │
│   │  │Bucket   │ │ Table   │ │ Queue   │ │Container│       │   │
│   │  │Policies │ │ Indexes │ │ DLQ     │ │ Runtime │       │   │
│   │  │Versions │ │ Streams │ │ Delay   │ │         │       │   │
│   │  └─────────┘ └─────────┘ └─────────┘ └─────────┘       │   │
│   │                                                          │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                 Storage Layer                            │   │
│   │                                                          │   │
│   │  ┌─────────────────┐  ┌─────────────────────────────┐   │   │
│   │  │ In-Memory       │  │ Persistent                   │   │   │
│   │  │ (HashMap)       │  │ (SQLite, RocksDB, Files)    │   │   │
│   │  │                 │  │                              │   │   │
│   │  │ Fast, ephemeral │  │ Survives restarts            │   │   │
│   │  └─────────────────┘  └─────────────────────────────┘   │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 5. Lambda Emulation with Docker

```
┌─────────────────────────────────────────────────────────────────┐
│                   Lambda Emulation                               │
│                                                                  │
│   Invoke Request                                                 │
│        │                                                         │
│        ▼                                                         │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                Lambda Service                            │   │
│   │                                                          │   │
│   │   1. Find function definition                            │   │
│   │   2. Start Docker container with runtime                │   │
│   │   3. Pass event payload                                  │   │
│   │   4. Wait for response                                   │   │
│   │   5. Return result                                        │   │
│   │                                                          │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│                            ▼                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                Docker Container                          │   │
│   │                                                          │   │
│   │   • Lambda Runtime (Python, Node, Rust, etc.)           │   │
│   │   • User's handler code                                  │   │
│   │   • Mocked AWS SDK endpoints                             │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 6. Deep DynamoDB Emulation

```rust
// Deeper DynamoDB emulation
pub struct DynamoDBService {
    tables: RwLock<HashMap<String, Table>>,
}

pub struct Table {
    name: String,
    key_schema: KeySchema,
    items: BTreeMap<CompositeKey, Item>,  // Sorted for queries
    
    // Advanced features
    gsi: Vec<GlobalSecondaryIndex>,     // Secondary indexes
    lsi: Vec<LocalSecondaryIndex>,
    stream: Option<DynamoStream>,       // Change data capture
    ttl_attribute: Option<String>,      // Auto-expiry
}

impl DynamoDBService {
    // Query with filters, projections, pagination
    pub fn query(&self, table: &str, params: QueryParams) -> QueryResult {
        // Key condition evaluation
        // Filter expression evaluation
        // Projection
        // Pagination with LastEvaluatedKey
    }
    
    // Transactions
    pub fn transact_write(&self, items: Vec<TransactItem>) -> Result<()> {
        // All-or-nothing writes across items
    }
}
```

---

## 7. Deep SQS Emulation

```rust
pub struct SQSService {
    queues: RwLock<HashMap<String, Queue>>,
}

pub struct Queue {
    messages: VecDeque<Message>,
    in_flight: HashMap<String, InFlightMessage>,
    dead_letter: Option<String>,  // DLQ URL
    
    // Queue attributes
    visibility_timeout: Duration,
    message_retention: Duration,
    delay_seconds: u32,
    max_receive_count: u32,  // Before DLQ
    
    // FIFO queue support
    is_fifo: bool,
    deduplication_scope: Option<String>,
}

impl SQSService {
    pub fn receive(&self, queue: &str, max: u32) -> Vec<Message> {
        // 1. Get messages from queue
        // 2. Move to in_flight with visibility timeout
        // 3. Track receive count
        // 4. Move to DLQ if max receives exceeded
    }
    
    // Background task for visibility timeout
    pub async fn visibility_timeout_reaper(&self) {
        // Return messages to queue when visibility expires
    }
}
```

---

## 8. Practical Recommendations

| Use Case | Recommendation |
|----------|----------------|
| **Unit tests** | CloudEmu (in-memory, fast) |
| **Integration tests** | CloudEmu + LocalStack |
| **Lambda testing** | AWS SAM Local or LocalStack |
| **Database testing** | Real PostgreSQL/Redis in Docker |
| **Full system test** | Real AWS (test account) |

---

## 9. Feature Implementation Roadmap

| Feature | Priority | Complexity | Status |
|---------|----------|------------|--------|
| **S3 Basic** | High | Low | ✅ Done |
| **S3 Versioning** | Medium | Medium | ⬜ |
| **S3 Lifecycle** | Low | Medium | ⬜ |
| **S3 Bucket Policies** | Medium | High | ⬜ |
| **DynamoDB Basic** | High | Medium | ⬜ |
| **DynamoDB Query** | High | High | ⬜ |
| **DynamoDB GSI/LSI** | Medium | High | ⬜ |
| **DynamoDB Streams** | Low | High | ⬜ |
| **SQS Basic** | High | Medium | ⬜ |
| **SQS DLQ** | Medium | Medium | ⬜ |
| **SQS FIFO** | Medium | Medium | ⬜ |
| **Lambda Docker** | High | High | ⬜ |
| **IAM Policies** | Medium | Very High | ⬜ |
| **SNS Basic** | Medium | Medium | ⬜ |
| **EventBridge** | Low | High | ⬜ |

---

## 10. Related Documents

- [11-cloudemu-architecture.md](11-cloudemu-architecture.md) - CloudEmu base architecture
- [04-provider-integration.md](04-provider-integration.md) - SDK integration
- [09-testing-strategy.md](09-testing-strategy.md) - Testing with emulators
