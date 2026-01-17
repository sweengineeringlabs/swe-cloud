# IAC Documentation Hub

Welcome to the Multi-Cloud IAC Framework documentation. This project uses the **Stratified Encapsulation Architecture (SEA)** to provide a unified interface for infrastructure across AWS, Azure, and GCP.

## Quick Navigation

| Target Audience | Recommended Starting Point |
| :--- | :--- |
| **New Users** | [Installation Guide](6-deployment/installation-guide.md) |
| **Developers** | [Developer Guide](4-development/developer-guide.md) |
| **Architects** | [Architecture Specification](3-design/architecture.md) |
| **Security/QA** | [Testing Strategy](5-testing/testing-strategy.md) |

## Core Documentation

- **[Glossary](glossary.md)**: Definitions of terms and architectural layers.
- **[Architecture Specification](3-design/architecture.md)**: Deep dive into the SEA layers and provider abstraction.
- **[Toolchain Specification](3-design/toolchain.md)**: Details on Go and Terratest integration.
- **[Migration Guide](2-migration/migration-guide.md)**: Documentation for migrating from legacy structures to SEA.

## Service Catalog (Facades)

The following services are implemented as unified facades:

- **[Compute](facade/compute/doc/overview.md)**: EC2, Azure VM, GCP Compute Engine, ZeroCompute.
- **[Storage](facade/storage/doc/overview.md)**: S3, Blob Storage, GCS, ZeroStore.
- **[Database](facade/database/doc/overview.md)**: RDS, Azure SQL, Cloud SQL, ZeroDB.
- **[Networking](facade/networking/doc/overview.md)**: VPC, VNet, GCP Network, ZeroNet.
- **[IAM](facade/iam/doc/overview.md)**: Roles, Identities, Service Accounts, ZeroID.
- **[Monitoring](facade/monitoring/doc/overview.md)**: CloudWatch, Azure Monitor, Cloud Monitoring.
- **[Lambda/Serverless](facade/lambda/doc/overview.md)**: AWS Lambda, ZeroFunc.
- **[Messaging](facade/messaging/doc/overview.md)**: SQS, SNS, ZeroQueue.

## Local Development & Testing

- **[CloudEmu Integration](4-development/cloudemu-integration.md)**: Local cloud emulation for fast, cost-free testing.
- **[Local Testing Example](../examples/local-cloudemu/)**: Complete example using CloudEmu for development.
- **[Zero Integration Example](../examples/zero-integration/)**: Complete example using ZeroCloud.
- **[Integration Tests](../test/integration/)**: Automated Terratest suite (AWS, Azure, GCP, Zero).

## Project Planning

- **[Integration Plan](2-planning/iac-cloudemu-integration-plan.md)**: IAC-CloudEmu integration roadmap.
- **[Backlog](4-development/backlog.md)**: Current task list and roadmap.
- **[Framework Backlog](docs/framework-backlog.md)**: Cross-cutting architectural improvements.

---

**Last Updated**: 2026-01-14  
**Version**: 1.0.0
