# Lambda/Serverless Facade Module

## WHAT: Unified Function-as-a-Service Provisioning

The Lambda facade provides a unified interface for serverless functions. Current support focuses on AWS Lambda, with Azure Functions and GCP Cloud Functions planned.

**Prerequisites**:
- Terraform `1.0.0+`
- Configured Cloud CLI for the target provider.

## WHY: Standardizing Serverless Deployment

### Problems Solved
- **Runtime Normalization**: Ensuring consistent runtime definitions (e.g., `python3.9`) across providers.
- **Trigger Abstraction**: Providing a consistent way to define function handlers and entry points.

## HOW: Usage Example

```hcl
module "process_data" {
  source        = "../../facade/lambda"
  provider_name = "aws"
  function_name = "data-processor"
  handler       = "index.handler"
  runtime       = "python3.11"
}
```

## Examples and Tests
- **Unit Tests**: See `facade/lambda/lambda_test.go` for Terratest plan assertions.

---

**Last Updated**: 2026-01-14
