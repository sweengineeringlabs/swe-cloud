# Azure Developer Guide

## WHAT
Guide for Azure module development.

## WHY
Enable contributors to add Azure services.

## HOW

### Adding Services
1. **Control Core**: Implement `AzureProvider` logic in `azure-control-core`.
2. **Data Core**: Add tables to `azure-data-core`.
3. **Register**: Add routing logic in `CloudProviderTrait`.

### Testing
Use `azurite` compatible SDKs or `az` CLI.
