# Cloud Industry: Data Plane Services Reference

The **Data Plane** is the execution and transmission layer. It processes actual user data, executes application code, and moves bits across networks.

## 1. Compute & Execution
| Category | AWS | Azure | GCP |
| :--- | :--- | :--- | :--- |
| **Virtual Machines** | EC2 (Nitro/Xen) | Azure VMs (Hyper-V) | Compute Engine (KVM) |
| **Serverless Functions** | Lambda (Firecracker) | Functions | Cloud Functions |
| **Containers** | ECS / Fargate | Container Instances | Cloud Run / GKE |
| **Batch Processing** | AWS Batch | Azure Batch | Batch |

## 2. Storage & Content Delivery
| Category | AWS | Azure | GCP |
| :--- | :--- | :--- | :--- |
| **Object Storage** | S3 | Blob Storage | Cloud Storage |
| **Block Storage** | EBS | Managed Disks | Persistent Disk |
| **File Systems** | EFS / FSx | Azure Files | Filestore |
| **CDN (Edge)** | CloudFront | Azure Front Door | Cloud CDN |

## 3. Database & Persistence
| Category | AWS | Azure | GCP |
| :--- | :--- | :--- | :--- |
| **NoSQL (Document)** | DynamoDB | Cosmos DB | Firestore |
| **Relational (SQL)** | RDS / Aurora | SQL Database | Cloud SQL / Spanner |
| **In-Memory** | ElastiCache (Redis) | Cache for Redis | Memorystore |
| **Graph** | Neptune | Cosmos DB (Gremlin) | - |

## 4. Networking & Connectivity
| Category | AWS | Azure | GCP |
| :--- | :--- | :--- | :--- |
| **Virtual Network** | VPC | VNet | VPC |
| **Load Balancing** | ALB / NLB | Load Balancer | Cloud Load Balancing |
| **Private Links** | PrivateLink | Private Link | Private Service Connect |
| **Messaging/Queueing** | SQS | Service Bus Queues | Pub/Sub / Cloud Tasks |

## 5. Analytics & AI Data Paths
| Category | AWS | Azure | GCP |
| :--- | :--- | :--- | :--- |
| **Data Warehousing** | Redshift | Synapse Analytics | BigQuery |
| **Stream Processing** | Kinesis | Event Hubs | Dataflow |
| **ML Inference** | SageMaker Runtime | Azure ML Endpoints | Vertex AI Prediction |

---

**Parity in ZeroCloud**: 
ZeroCloud focuses on providing high-performance, native data-plane parity for the primary services in Categories 1, 2, 3, and 4 (identified as the **"Big 6 Primitives"**).
