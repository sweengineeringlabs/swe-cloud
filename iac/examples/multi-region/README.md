# Multi-Region Support Example

This example demonstrates how to use the IAC Facade Layer to deploy redundant infrastructure across two different AWS regions for High Availability (HA) and Disaster Recovery (DR).

## Overview

The deployment includes:
1.  **Primary Region (`us-east-1`):**
    *   Compute Instance (API/Web)
    *   Storage Bucket (Primary Data)
2.  **Secondary Region (`us-west-2`):**
    *   Compute Instance (Standby)
    *   Storage Bucket (Failover Data)

## Usage

1.  Initialize Terraform:
    ```bash
    terraform init
    ```

2.  Plan the deployment:
    ```bash
    terraform plan
    ```

3.  Apply changes:
    ```bash
    terraform apply
    ```

## Key Benefits

*   **Regional Redundancy:** Survive a complete region outage.
*   **Low Latency:** Serve users from the nearest region.
*   **Consistent Configuration:** Same Facade modules ensure identical security and performance profiles in both regions.
