# ZeroCloud Service Compatibility Matrix

| Category | Service | Status | Native Implementation Detail |
| :--- | :--- | :--- | :--- |
| **Compute** | **EC2** | ✅ **Implemented** | Maps to `ZeroWorkloads` (VMs/Containers) via `ComputeDriver`. |
| | **Lambda** | ⏳ Pending | Needs a function execution runtime. |
| | **ECS** | ⏳ Pending | Could map to `ZeroWorkloads` but needs task definitions. |
| | **ECR** | ⏳ Pending | Needs a container registry. |
| **Storage** | **S3** | ✅ **Implemented** | Maps to local directories via `ZeroStorage` volumes. |
| | **EBS** | ✅ **Implemented** | Maps to `ZeroVolumes` (Block Storage) via `StorageDriver`. |
| | **Glacier** | ⏳ Pending | No archival storage tier yet. |
| **Database** | **DynamoDB** | ✅ **Implemented** | Maps to embedded **SQLite** tables with JSON payloads. |
| | **RDS** | ⏳ Pending | Needs managed SQL server instances. |
| | **ElastiCache** | ⏳ Pending | Needs Redis/Memcached emulation. |
| **Networking** | **VPC** | ✅ **Implemented** | Maps to `ZeroNetworks` (Bridge/NAT) via `NetworkDriver`. |
| | **Route53** | ⏳ Pending | No DNS management yet. |
| | **ELB** | ⏳ Pending | No load balancing capability. |
| | **APIGateway** | ⏳ Pending | No request routing/transformation layer. |
| **Integration**| **SQS** | ⏳ Pending | Missing message queueing. |
| | **SNS** | ⏳ Pending | Missing pub/sub messaging. |
| | **EventBridge**| ⏳ Pending | Missing event bus. |
| | **StepFunctions**| ⏳ Pending | Missing state machine orchestration. |
| **Security** | **IAM** | ⏳ Pending | No identity management (currently open access). |
| | **KMS** | ⏳ Pending | No key management. |
| | **Secrets** | ⏳ Pending | No secrets storage. |
| | **Cognito** | ⏳ Pending | No user pools. |
| **Management** | **CloudWatch** | ⏳ Pending | Basic stats implemented, but no logs/metrics API. |
| | **Pricing** | ⏳ Pending | No billing/cost emulation. |
