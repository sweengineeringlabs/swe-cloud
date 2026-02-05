# Cloud Planes Guide: Control vs. Data Plane

This guide explains the architectural distinction between the **Control Plane** and the **Data Plane** within ZeroCloud, and how this parity enables valid AWS-equivalent behavior for CloudKit applications.

## 1. Overview: The Two Planes

In a modern cloud system, responsibilities are split into two distinct execution layers:

| Plane | Role | ZeroCloud Implementation |
| :--- | :--- | :--- |
| **Control Plane** | **Management**: Planning, configuration, and resource lifecycle. | REST API Handlers, SQLite Metadata, `zero-cli`. |
| **Data Plane** | **Execution**: Handling actual application data and work. | Process Spawners, Reverse Proxies, FS I/O, Messaging Engines. |

---

## 2. The Control Plane (The Brain)

The Control Plane is the entry point for orchestration. It handles the "administrative" side of the cloud.

### Responsibilities:
- **Resource Lifecycle**: Creating, deleting, or updating buckets, queues, or instances.
- **State Management**: Persisting the "Source of Truth" in the `metadata.db`.
- **Authorization**: Validating IAM policies and user permissions.

### ZeroCloud Implementation:
When you call `zero-cli create-queue "my-queue"`, the Control Plane:
1. Validates authentication.
2. Checks if the queue name is valid.
3. Writes a record to the `queues` table in SQLite.
4. Returns a success response with the Queue URL.

---

## 3. The Data Plane (The Muscle)

The Data Plane is the high-performance path where application data flows. This is where ZeroCloud's architectural parity is most critical.

### Responsibilities:
- **I/O Operations**: Reading and writing bytes to disk or network.
- **Compute Execution**: Running actual application code (Python, Node.js).
- **Traffic Routing**: Forwarding packets between services.
- **Protocol Enforcement**: Managing visibility timeouts, receipt handles, and ACID transactions.

### ZeroCloud Implementation:
When your application sends a message to that queue, it hits the **ZeroQueue Data Plane**:
1. It physically accepts the payload characters.
2. It assigns a unique message ID and receipt handle.
3. It manages the **Visibility Clock** so the message disappears while being processed and reappears if the worker fails.

---

## 4. Service-Specific Plane Breakdown

| Service | Control Plane (Management) | Data Plane (Runtime Context) |
| :--- | :--- | :--- |
| **ZeroStore (S3)** | Creating buckets, setting lifecycle policies. | Streaming bit-streams to host NVMe/SSD. |
| **ZeroDB (NoSQL)** | Defining tables, managing indices. | Executing ACID JSON queries via SQLite. |
| **ZeroFunc (Lambda)** | Registering code, setting memory limits. | Forking OS processes and piping stdin/stdout. |
| **ZeroLB (ALB)** | Registering targets, defining HTTP rules. | Functional Rust Reverse Proxy (Axum/Reqwest). |
| **ZeroNet (VPC)** | Creating virtual bridges, assigning IPs. | Native network bridging and interface listeners. |

---

## 5. Why Parity Matters for CloudKit

ZeroCloud is designed for **Data Plane Parity** rather than just "Mocking." This provides three major benefits:

1. **Behavioral Integrity**: If your app causes a "race condition" or a "deadlock" locally in ZeroDB, it would likely happen in DynamoDB too.
2. **Offline Resilience**: Because the Data Plane resides on your physical hardware, your application remains fully functional without an internet connection.
3. **Valid Integration Testing**: You can test how your app handles SQS "Visibility Timeouts" or ALB "502 Gateway Errors" locally because the Zero interfaces physically implement those behaviors.

---

## Summary

The **Control Plane** builds the environment, while the **Data Plane** runs the workload. By ensuring parity across both planes, ZeroCloud allows CloudKit applications to move from local private infrastructure to the public cloud with zero logic changes.

---

**Related Documentation**:
- [Architecture Hub](./architecture.md)
- [AWS-Zero Parity Manifesto](../2-planning/aws_parity_manifesto.md)
- [Developer Guide](../4-development/developer-guide.md)
