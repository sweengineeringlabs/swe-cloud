# Triple-Sync Implementation Plan: CloudKit + IAC + CloudEmu

**Status**: 🔄 In Progress
**Objective**: Synchronize all three layers (Application, Infrastructure, and Emulator) so that every service defined in CloudKit can be provisioned via IAC and emulated via CloudEmu.

---

## 1. Synchronization Matrix

| Service | CloudKit API | IAC Facade | AWS (Emu) | Azure (Emu) | GCP (Emu) | Status |
|---------|--------------|------------|-----------|-------------|-----------|--------|
| **Compute** | `compute` | `compute/` | ✅ Full | ⚠️ Infra Only | ⚠️ Infra Only | 🟡 Partial |
| **Networking** | `networking` | `networking/` | ✅ Full | ⚠️ Infra Only | ⚠️ Infra Only | 🟡 Partial |
| **Storage** | `object_storage` | `storage/` | ✅ Full | ⚠️ Infra Only | ⚠️ Infra Only | 🟡 Partial |
| **NoSQL** | `kv_store` | `nosql/` | ✅ Full | ⚠️ Infra Only | ⚠️ Infra Only | 🟡 Partial |
| **Queue** | `message_queue` | `messaging/` | ✅ Full | ⚠️ Infra Only | ⚠️ Infra Only | 🟡 Partial |
| **Pub/Sub** | `pubsub` | `messaging/` | ✅ Full | ⚠️ Infra Only | ⚠️ Infra Only | 🟡 Partial |
| **Functions** | `functions` | `lambda/` | ✅ Full | ⚠️ Infra Only | ⚠️ Infra Only | 🟡 Partial |
| **Secrets** | `secrets` | `secrets/` | ✅ Full | ⚠️ Infra Only | ⚠️ Infra Only | 🟡 Partial |
| **Events** | `events` | `events/` | ✅ Full | ⚠️ Infra Only | ⚠️ Infra Only | 🟡 Partial |
| **KMS** | `encryption` | `encryption/` | ✅ Full | ⚠️ Infra Only | ⚠️ Infra Only | 🟡 Partial |
| **Workflows** | `workflow` | `workflows/` | ✅ Full | ⚠️ Infra Only | ⚠️ Infra Only | 🟡 Partial |

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

### Phase 3: CloudKit SDK Synchronization (The "Code" Tasks) ✅ COMPLETE
Bring the Rust SDK into parity with the IAC layer and the Emulator.

- [x] **Task 3.1: Compute Trait** ✅
- [x] **Task 3.2: Networking Trait** ✅

### Phase 4: Azure & GCP Infra Parity ✅ COMPLETE
Bring the IAC layer for Azure/GCP up to speed with AWS.
- [x] **Task 4.1: Core Modules** (Compute, Storage, NoSQL, etc.) ✅
- [x] **Task 4.2: Facade Updates** (Routing/Conditions) ✅

### Phase 5: Multi-Cloud Emulator Data Plane (The "Runtime" Tasks)
*New Frontier*: Implement the actual emulation logic for Azure/GCP data planes.
- [ ] **Task 5.1: Azure Data Plane** (Blob, Cosmos, EventGrid)
- [ ] **Task 5.2: GCP Data Plane** (GCS, Firestore, PubSub)

### Phase 6: Triple-Sync Validation
- [ ] **Task 6.1**: Create `iac/examples/triple-sync-demo/`
  - Provisions: VPC, Subnet, EC2, S3, DynamoDB, SQS, SNS, Secrets, KMS, and Lambda.
- [ ] **Task 3.2**: Create a CloudKit application in `cloudkit/examples/full_stack_validator/`
  - Runs against the infrastructure provisioned in Task 3.1.
  - Verifies every service works in unison.

---

## 3. Success Criteria
1. `terraform apply` on the demo stack succeeds against CloudEmu.
2. The resource dashboard shows all 10+ services active.
3. The CloudKit validator application reports 100% pass on all service traits.
