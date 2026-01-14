# Glossary

Alphabetized list of terms used in IAC (Infrastructure as Code) project.

---

**API Layer** - Layer providing typed interfaces to Core functionality. Exposes cloud operations to Terraform modules.

**Backend** - Terraform concept for storing state remotely (S3, Azure Blob, etc.).

**Core Layer** - Implementation layer containing business logic. Coordinates between API and SPI.

**Data Source** - Terraform construct for reading existing infrastructure without managing it.

**Facade** - High-level Terraform modules for common multi-cloud patterns (databases, messaging, etc.).

**HCL (HashiCorp Configuration Language)** - Declarative language used by Terraform for infrastructure definitions.

**Module** - Reusable Terraform configuration package that encapsulates infrastructure resources.

**Output** - Terraform value exported from a module for use by other configurations.

**Plan** - Terraform execution preview showing what changes will be applied.

**Provider** - Terraform plugin implementing cloud-specific resource management (AWS, Azure, GCP).

**Resource** - Terraform managed infrastructure component (bucket, database, network).

**SEA (Stratified Encapsulation Architecture)** - Layered pattern with SPI, API, Core, and Facade layers.

**SPI (Service Provider Interface)** - Contract defining how providers implement cloud operations.

**State** - Terraform's record of managed infrastructure, mapping configuration to real resources.

**Variable** - Input parameter to a Terraform module allowing customization.

**Workspace** - Isolated Terraform state environment for managing multiple deployments.
