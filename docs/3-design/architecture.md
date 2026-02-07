# SWE Cloud Architecture

**Audience**: Architects, Technical Leadership

## Overview

SWE Cloud follows a three-layer architecture with a shared SEA (SPI-API-Core-Facade) pattern across all Rust crates.

## High-Level Architecture

```mermaid
flowchart TB
    subgraph Apps["Applications"]
        UI[SWE Cloud UI<br/>React/TypeScript]
        CC[CloudCost<br/>FinOps CLI]
    end

    subgraph CloudKit["CloudKit SDK"]
        CKF[cloudkit_facade]
        CKA[cloudkit_api]
        CKC[cloudkit_core]
        CKS[cloudkit_spi]
    end

    subgraph CloudEmu["CloudEmu"]
        direction TB
        subgraph Providers["Provider Facades"]
            AWS[AWS Facade]
            AZ[Azure Facade]
            GCP[GCP Facade]
        end
        subgraph Planes["Control + Data Planes"]
            CP[Control Plane<br/>Resource Management]
            DP[Data Plane<br/>Runtime Operations]
        end
    end

    subgraph IAC["Infrastructure as Code"]
        TF[Terraform Facade]
        MOD[Provider Modules]
    end

    UI --> CloudKit
    UI --> CloudEmu
    CC --> CloudKit
    CC --> CloudEmu
    CloudKit --> CloudEmu
    IAC --> CloudEmu
    Providers --> Planes
```

## SEA Layering Pattern

Every provider crate follows this dependency flow:

```
Facade → Core → API → SPI
```

- **SPI**: Types, errors, and foundational traits
- **API**: Public service trait contracts
- **Core**: Business logic and service implementations
- **Facade**: SAF re-exports providing a unified entry point

## Sub-Project Architecture
- [CloudEmu Architecture](../../cloudemu/docs/3-design/architecture.md)
- [CloudKit Architecture](../../cloudkit/docs/3-design/architecture.md)
- [IAC Architecture](../../iac/docs/3-design/ARCHITECTURE.md)
