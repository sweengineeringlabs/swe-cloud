# IAC Development Backlog

**Last Updated:** 2026-01-14  
**Project:** Multi-Cloud Infrastructure as Code (SEA Architecture)  
**Overall Completion:** ~25%

---

## Overview

This backlog tracks the remaining implementation work for the IAC project. The architecture is **fully designed** and **structure is complete**, but many provider modules remain unimplemented.

**Status Legend:**
- ✅ Complete
- 🟡 Partial / In Progress
- ❌ Not Started
- 🔸 Blocked / Needs Decision

---

## Phase 1: Core Layer Implementation

### AWS Core Modules (`iac_core/aws/src/`)

| Module | Status | Priority | Estimated Effort | Notes |
|--------|--------|----------|------------------|-------|
| **compute/** | ✅ | P0 | - | EC2 instance module complete |
| **storage/** | ✅ | P0 | - | S3 bucket module complete |
| **database/** | ❌ | P1 | 2-3 hours | RDS instance module needed |
| **networking/** | ❌ | P1 | 3-4 hours | VPC, Subnet, Route Table modules |
| **iam/** | ❌ | P2 | 2-3 hours | IAM roles, policies, instance profiles |
| **messaging/** | ❌ | P2 | 2 hours | SQS, SNS modules |
| **lambda/** | ❌ | P3 | 2 hours | Lambda function module |
| **monitoring/** | ❌ | P3 | 2 hours | CloudWatch, alarms |

**Acceptance Criteria:**
- Each module has `main.tf` and `variables.tf`
- Clean, documented Terraform code
- Follows EC2/S3 pattern
- Outputs match API contract

### Azure Core Modules (`iac_core/azure/src/`)

| Module | Status | Priority | Estimated Effort | Notes |
|--------|--------|----------|------------------|-------|
| **compute/** | ❌ | P1 | 3 hours | VM, VMSS modules |
| **storage/** | ❌ | P1 | 2 hours | Blob storage module |
| **database/** | ❌ | P2 | 2 hours | Cosmos DB, SQL Database |
| **networking/** | ❌ | P2 | 3 hours | VNet, Subnet, NSG |
| **iam/** | ❌ | P3 | 2 hours | Managed identities, RBAC |

**Acceptance Criteria:**
- Full parity with AWS modules
- Azure-specific best practices
- Resource group management

### GCP Core Modules (`iac_core/gcp/src/`)

| Module | Status | Priority | Estimated Effort | Notes |
|--------|--------|----------|------------------|-------|
| **compute/** | ❌ | P1 | 3 hours | Compute Engine instances |
| **storage/** | ❌ | P1 | 2 hours | Cloud Storage buckets |
| **database/** | ❌ | P2 | 2 hours | Cloud SQL, Firestore |
| **networking/** | ❌ | P2 | 3 hours | VPC, Subnets, Firewall rules |
| **iam/** | ❌ | P3 | 2 hours | Service accounts, IAM bindings |

**Acceptance Criteria:**
- Full parity with AWS/Azure
- GCP-specific best practices
- Project/folder management

---

## Phase 2: Facade Layer Updates

### Update Facade to New Structure

| Task | Status | Priority | Estimated Effort | Notes |
|------|--------|----------|------------------|-------|
| Update compute facade paths | ❌ | P0 | 1 hour | Change `../../core/compute` → `../../iac_core/aws/src/compute` |
| Update storage facade paths | ❌ | P0 | 1 hour | Change paths to new structure |
| Add provider selection logic | ❌ | P1 | 2 hours | Route to correct provider module |
| Update facade variables | ❌ | P1 | 1 hour | Align with new core modules |
| Add database facade | ❌ | P2 | 2 hours | New facade for database resources |
| Add networking facade | ❌ | P2 | 2 hours | New facade for network resources |

**Acceptance Criteria:**
- Facades route to `iac_core/{provider}/src/{resource}`
- Provider selection works correctly
- Backwards compatible where possible

---

## Phase 3: Examples & Documentation

### Examples

| Example | Status | Priority | Estimated Effort | Notes |
|---------|--------|----------|------------------|-------|
| **web-app** | 🟡 | P0 | 1 hour | Update paths to new structure |
| **data-pipeline** | ❌ | P2 | 3 hours | Storage + Database + Lambda example |
| **multi-region** | ❌ | P3 | 2 hours | Multi-region deployment example |
| **multi-cloud** | ❌ | P3 | 3 hours | AWS + Azure + GCP in one setup |

**Acceptance Criteria:**
- All examples work with `terraform apply`
- Clear README with usage instructions
- Cost estimates included

### Documentation

| Document | Status | Priority | Estimated Effort | Notes |
|----------|--------|----------|------------------|-------|
| PROGRESS.md | ✅ | P0 | - | Complete |
| package-strategy.md ADR | ✅ | P0 | - | Complete and up-to-date |
| Module README per resource | 🟡 | P1 | 3 hours | Need READMEs for each module |
| Migration guide | ❌ | P2 | 2 hours | How to migrate from old structure |
| Testing guide | ❌ | P2 | 2 hours | Terratest setup guide |
| Contributing guide | ❌ | P3 | 1 hour | How to add new providers/modules |

---

## Phase 4: API Layer Refinement

### API Contracts

| Contract | Status | Priority | Estimated Effort | Notes |
|----------|--------|----------|------------------|-------|
| Compute API | ✅ | P0 | - | Complete |
| Storage API | ✅ | P0 | - | Complete |
| Database API | ❌ | P1 | 2 hours | Define database contract |
| Networking API | ❌ | P1 | 2 hours | Define network contract |
| IAM API | ❌ | P2 | 1 hour | Define IAM contract |

**Acceptance Criteria:**
- Input/output contracts defined
- Validation rules comprehensive
- Provider-agnostic

---

## Phase 5: SPI Layer Expansion

### Provider Integration

| Provider | Status | Priority | Estimated Effort | Notes |
|----------|--------|----------|------------------|-------|
| AWS SPI | ✅ | P0 | - | Complete (backend, provider config) |
| Azure SPI | ❌ | P1 | 2 hours | Azure backend, provider config |
| GCP SPI | ❌ | P1 | 2 hours | GCS backend, provider config |
| Oracle SPI | ❌ | P3 | 2 hours | OCI backend, provider config |

**Acceptance Criteria:**
- Remote state backend configured
- Provider authentication setup
- Default tags/labels configured

---

## Phase 6: Testing Infrastructure

### Test Coverage

| Test Type | Status | Priority | Estimated Effort | Notes |
|-----------|--------|----------|------------------|-------|
| Validation tests | ❌ | P1 | 3 hours | Input validation with `terraform validate` |
| Unit tests (Terratest) | ❌ | P2 | 5 hours | Test individual modules |
| Integration tests | ❌ | P2 | 5 hours | Test full deployments |
| Contract tests | ❌ | P3 | 3 hours | Verify API contracts |
| Multi-cloud tests | ❌ | P3 | 4 hours | Test provider switching |

**Acceptance Criteria:**
- Automated test suite
- CI/CD integration ready
- Test coverage >70%

---

## Priority Breakdown

### P0 - Critical (Blocking)
- [ ] Update facade paths to new structure
- [ ] Update web-app example
- [ ] Fix any broken references

**Estimated Effort:** ~3 hours

### P1 - High Priority
- [ ] Complete AWS database module
- [ ] Complete AWS networking module
- [ ] Implement Azure compute/storage
- [ ] Implement GCP compute/storage
- [ ] Define database & networking API contracts

**Estimated Effort:** ~20 hours

### P2 - Medium Priority
- [ ] Complete remaining AWS modules (IAM, messaging)
- [ ] Complete remaining Azure modules
- [ ] Complete remaining GCP modules
- [ ] Add database/networking facades
- [ ] Create data-pipeline example

**Estimated Effort:** ~25 hours

### P3 - Low Priority
- [ ] Lambda, monitoring modules
- [ ] Oracle provider
- [ ] Multi-cloud examples
- [ ] Advanced testing

**Estimated Effort:** ~20 hours

---

## Milestones

### Milestone 1: Core AWS Complete (Target: Week 1)
- ✅ Compute & Storage implemented
- [ ] Database implemented
- [ ] Networking implemented
- [ ] Facades updated
- [ ] Examples working

**Completion:** 40% → 70%

### Milestone 2: Multi-Cloud Foundation (Target: Week 2-3)
- [ ] Azure compute & storage
- [ ] GCP compute & storage
- [ ] Provider switching works
- [ ] Multi-cloud example

**Completion:** 70% → 85%

### Milestone 3: Production Ready (Target: Week 4-5)
- [ ] All core modules complete
- [ ] Full test coverage
- [ ] Complete documentation
- [ ] Migration guide

**Completion:** 85% → 100%

---

## Dependencies

### External Dependencies
- Terraform >= 1.0
- AWS provider ~> 5.0
- Azure provider ~> 3.0
- GCP provider ~> 5.0

### Internal Dependencies
- Common layer ✅ Complete
- API contracts ✅ Compute & Storage complete
- SPI layer 🟡 AWS only

---

## Risks & Blockers

| Risk | Impact | Mitigation | Status |
|------|--------|------------|--------|
| Provider API changes | High | Pin provider versions | ✅ Mitigated |
| Complex module interactions | Medium | Comprehensive testing | 🟡 In Progress |
| Documentation drift | Low | Auto-generate where possible | ❌ Not Started |

---

## Quick Start (For Contributors)

### To Add a New AWS Module:

1. Create directory: `iac_core/aws/src/{resource_type}/`
2. Add `main.tf` with Terraform resources
3. Add `variables.tf` with inputs
4. Add `outputs.tf` (optional) with outputs
5. Follow existing compute/storage pattern
6. Update this backlog

### To Add a New Provider Module:

1. Create directory: `iac_core/{provider}/src/{resource_type}/`
2. Implement module following AWS pattern
3. Add SPI configuration in `iac_spi/{provider}/`
4. Update facade to support new provider
5. Add example showcasing new provider

---

## Notes

- **Architecture is solid** - No structural changes needed
- **Focus on implementation** - Just need to fill in the modules
- **Follow patterns** - AWS compute/storage are good templates
- **Test as you go** - Don't wait until the end

---

**Total Estimated Effort:** ~70 hours  
**Current Progress:** ~25%  
**Target Completion:** 4-5 weeks (part-time)
