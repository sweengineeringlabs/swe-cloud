# Glossary

Alphabetized list of terms used in SWE Cloud.

---

## A

**API (Application Programming Interface)**
: The public trait contracts layer in the SEA architecture. Defines service interfaces without implementation.

**ADR (Architecture Decision Record)**
: A document capturing an important architectural decision along with its context and consequences.

---

## C

**CloudEmu**
: Local multi-cloud emulator supporting AWS, Azure, GCP, and Oracle services.

**CloudKit**
: Rust multi-cloud SDK providing a unified interface across cloud providers.

**Control Plane**
: The management layer handling resource creation, configuration, and lifecycle operations.

**Core**
: The business logic implementation layer in the SEA architecture. Contains service handlers and orchestration.

---

## D

**Data Plane**
: The runtime layer handling data operations (read, write, query) on provisioned resources.

---

## F

**Facade**
: The top-level entry point in the SEA architecture. Re-exports from SPI, API, and Core via the SAF module.

**FOCUS (FinOps Open Cost & Usage Specification)**
: Industry standard for normalizing cloud cost and usage data across providers.

---

## I

**IAC (Infrastructure as Code)**
: Terraform-based infrastructure provisioning with multi-cloud facade modules.

---

## S

**SAF (Service Access Facade)**
: The re-export module within a Facade crate that provides a unified public API.

**SEA (SPI-API-Core-Facade)**
: The crate layering architecture pattern used across all Rust projects. Separates extension points (SPI) from public contracts (API), implementations (Core), and entry points (Facade).

**SPI (Service Provider Interface)**
: The foundational types, errors, and trait definitions layer in the SEA architecture.

---

## T

**Triple-Sync**
: The alignment strategy ensuring every service is available across CloudKit (SDK), IAC (Terraform), and CloudEmu (emulator).
