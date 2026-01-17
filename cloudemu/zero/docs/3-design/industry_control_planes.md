# Cloud Industry: Control Plane Services Reference

The **Control Plane** is the management layer of the cloud. It is responsible for provisioning, configuring, and governing resources. It does not handle application payload data.

## 1. Governance & Orchestration
| Category | AWS | Azure | GCP |
| :--- | :--- | :--- | :--- |
| **Infrastructure as Code** | CloudFormation / CDK | ARM Templates / Bicep | Cloud Deployment Manager |
| **Resource Management** | Organizations | Management Groups | Resource Manager |
| **Policy Enforcement** | Control Tower / Config | Azure Policy | Cloud Policy Intelligence |
| **Configuration Tracking** | AWS Config | Azure Resource Graph | Cloud Asset Inventory |

## 2. Security & Identity Management
| Category | AWS | Azure | GCP |
| :--- | :--- | :--- | :--- |
| **Identity (IAM)** | IAM | Microsoft Entra ID (AD) | IAM |
| **Key Management** | KMS | Key Vault | Cloud KMS |
| **Secret Management** | Secrets Manager | Key Vault Secrets | Secret Manager |
| **Identity Governance** | Permissions Boundary | Azure AD PIM | IAM Recommender |

## 3. Monitoring & Observability (Management)
| Category | AWS | Azure | GCP |
| :--- | :--- | :--- | :--- |
| **API Auditing** | CloudTrail | Activity Logs | Cloud Audit Logs |
| **Alarm Logic** | CloudWatch Alarms | Azure Monitor Alerts | Cloud Monitoring Alerts |
| **Compliance** | Artifact / Audit Manager | Blueprints / Compliance | Security Command Center |

## 4. Application Orchestration
| Category | AWS | Azure | GCP |
| :--- | :--- | :--- | :--- |
| **Workflow Engines** | Step Functions | Logic Apps | Cloud Workflows |
| **Event Orchestration** | EventBridge | Event Grid | Eventarc |
| **API Management** | API Gateway | API Management (APIM) | Apigee / API Gateway |

## 5. Developer & Delivery Tooling
| Category | AWS | Azure | GCP |
| :--- | :--- | :--- | :--- |
| **CI/CD Orchestration** | CodePipeline | Azure DevOps Pipelines | Cloud Build |
| **Git Repositories** | CodeCommit | Azure Repos | Cloud Source Repositories |
| **Cloud IDEs** | Cloud9 | Dev Box / Codespaces | Cloud Workstations |

---

**Relationship to ZeroCloud**: 
The **Zero Control Plane** (via `zero-control-facade`) implements a subset of these patterns (primarily IAM, Resource Management, and API Management) to provide a localized, high-performance orchestration engine.
