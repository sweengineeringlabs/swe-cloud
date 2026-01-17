# ZeroCloud Service Architecture Comparison

Detailed technical comparison of ZeroCloud's native services versus their AWS counterparts.

## 1. ZeroCompute vs Amazon EC2

| Feature | Amazon EC2 | ZeroCompute |
| :--- | :--- | :--- |
| **Virtualization** | Nitro Hypervisor (KVM-based) | Docker / Podman / Mock |
| **Isolation** | Hardware-level (VM) | OS-level (Container) or Process |
| **Networking** | VPC (SDN Overlay) | Linux Bridge / NetNS |
| **Storage** | EBS (Network Block Store) | Bind Mounts / Loop Devices |
| **Startup Time** | Seconds to Minutes | Milliseconds |

**Insight**: ZeroCompute trades strict multi-tenant isolation for extreme speed and low resource overhead, making it ideal for dev/test loops.

## 2. ZeroStore vs Amazon S3

| Feature | Amazon S3 | ZeroStore |
| :--- | :--- | :--- |
| **Storage Backend** | Distributed Object System | Local File System (XFS/ext4) |
| **Consistency** | Strong Consistency | Strong Consistency (POSIX) |
| **Metadata** | DynamoDB / Index Service | SQLite (metadata.db) |
| **Scalability** | Exabytes | Terabytes (Local Disk) |
| **API** | REST (XML/JSON) | REST (JSON-only optimized) |

**Insight**: ZeroStore offers valid S3 semantics but operates directly on the local filesystem, allowing direct inspection of "objects" as files.

## 3. ZeroDB vs Amazon DynamoDB

| Feature | Amazon DynamoDB | ZeroDB |
| :--- | :--- | :--- |
| **Storage Engine** | Proprietary B-Tree/LSM | SQLite (JSON Extension) |
| **Partitioning** | Hash Key (Sharding) | Single File (No Sharding) |
| **Schema** | Schemaless | Schemaless (JSON Column) |
| **indexing** | GSI / LSI | SQLite Indices on Generated Columns |
| **Transactions** | ACID (Item/Table) | ACID (SQLite Transaction) |

**Insight**: ZeroDB leverages SQLite's robust JSON support to emulate NoSQL behavior with full ACID guarantees, avoiding the complexity of distributed consensus.

## 4. ZeroFunc vs AWS Lambda

| Feature | AWS Lambda | ZeroFunc |
| :--- | :--- | :--- |
| **Runtime** | Firecracker MicroVM | Process Fork / Ephemeral Container |
| **Triggers** | EventBridge, SQS, Kinesis | Direct API, ZeroQueue (Polling) |
| **Cold Start** | 100ms - 10s | < 50ms (Process Spawn) |
| **State** | Stateless | Stateless (Can mount local FS) |
| **Concurrency** | Thousands | CPU Core Count |

**Insight**: ZeroFunc is a lightweight process spawner. It supports "mounting" local code without zipping/uploading, instant for iterative coding.

## 5. ZeroQueue vs Amazon SQS

| Feature | Amazon SQS | ZeroQueue |
| :--- | :--- | :--- |
| **Persistence** | Distributed Log | SQLite Table |
| **Ordering** | Best-Effort / FIFO | Strict FIFO (Auto-Increment ID) |
| **Visibility** | Timeout-based | Simple Delete-on-Read (MVP) |
| **Polling** | Long Polling | Instant Query |
| **Throughput** | Unlimited | ~10k msg/sec (SQLite Write Limit) |

**Insight**: ZeroQueue is ultra-fast for local IPC. It simplifies the visibility model for easier debugging.

## 6. ZeroID vs AWS IAM

| Feature | AWS IAM | ZeroID |
| :--- | :--- | :--- |
| **Model** | RBAC + ABAC | Simplified RBAC |
| **Language** | Policy JSON | Policy JSON (Subset) |
| **Evaluation** | Complex Logic | Basic Allow/Deny Matcher |
| **principals** | Users, Roles, Groups, Services | Users, Roles, Groups |

**Insight**: ZeroID provides the *structure* of IAM (ARNs, Policies) to satisfy SDK requirements, and now supports RBAC primitives for Roles and Groups.
**IAC Support**: ✅ Fully Supported (Roles, Policies, Policy Attachment).
