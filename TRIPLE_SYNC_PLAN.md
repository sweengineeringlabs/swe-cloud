# Triple-Sync Implementation Plan: CloudKit + IAC + CloudEmu

**Status**: 🔄 In Progress  
**Objective**: Synchronize all three layers (Application, Infrastructure, and Emulator) so that every service defined in CloudKit can be provisioned via IAC and emulated via CloudEmu.

---

## 1. Synchronization Matrix

| Service | CloudKit API | IAC Facade | CloudEmu Engine | Alignment |
|---------|--------------|------------|-----------------|-----------|
| **Compute** | - | `compute/` | EC2 | 🟡 Control Plane Only |
| **Networking** | - | `networking/` | VPC | 🟡 Control Plane Only |
| **Storage** | `object_storage` | `storage/` | S3 | ✅ Full |
| **NoSQL** | `kv_store` | `nosql/` | DynamoDB | ✅ Full |
| **Queue** | `message_queue` | `messaging/` | SQS | ✅ Full |
| **Pub/Sub** | `pubsub` | `messaging/` | SNS | ✅ Full |
| **Functions** | `functions` | `lambda/` | Lambda | ✅ Full (Local Exec) |
| **Secrets** | `secrets` | `secrets/` | SecretsMgr | ✅ Sync |
| **Events** | `events` | `events/` | EventBridge | ✅ Sync |
| **KMS** | `encryption` | `encryption/` | KMS | ✅ Sync |
| **Workflows** | `workflow` | `workflows/` | StepFunctions | ✅ Sync |

---

## 2. Implementation Roadmap

### Phase 1: High-Priority IAC Bridge (The "Sync" Tasks)
Currently, CloudKit and CloudEmu are "ahead" of IAC for several services. We need to create the missing Facade modules in the `iac/` project.

- [x] **Task 1.1: Secrets Facade** ✅
- [x] **Task 1.2: Events Facade** ✅
- [x] **Task 1.3: Encryption (KMS) Facade** ✅
- [x] **Task 1.4: Workflow Facade** ✅

### Phase 2: Configuration Standardization
Ensure environment variable naming conventions match CloudKit's expectations.

- [ ] **Task 2.1**: Update IAC facades to output standardized connection strings.
- [ ] **Task 2.2**: Ensure CloudEmu returns standard ARNs that CloudKit can parse.

### Phase 3: Triple-Sync Validation
- [ ] **Task 3.1**: Create `iac/examples/triple-sync-demo/`
  - Provisions: VPC, Subnet, EC2, S3, DynamoDB, SQS, SNS, Secrets, KMS, and Lambda.
- [ ] **Task 3.2**: Create a CloudKit application in `cloudkit/examples/full_stack_validator/`
  - Runs against the infrastructure provisioned in Task 3.1.
  - Verifies every service works in unison.

---

## 3. Success Criteria
1. `terraform apply` on the demo stack succeeds against CloudEmu.
2. The resource dashboard shows all 10+ services active.
3. The CloudKit validator application reports 100% pass on all service traits.
