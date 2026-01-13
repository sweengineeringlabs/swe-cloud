# Data Plane Architecture

The Data Plane (`src/storage/`) represents the massive underlying infrastructure that actually holds the bits. It is responsible for durability, persistence, and low-level data retrieval.

## Storage Engines

Different services utilize different optimized storage engines within the Data Plane:

### S3 Data Plane
A distributed key-value blob store optimized for **throughput**. It handles the physical storage of objects and their metadata.

### DynamoDB Data Plane
A fleet of storage nodes running a custom replication engine (conceptually similar to Paxos) optimized for **low latency**. It manages the high-speed retrieval and storage of structured items.
