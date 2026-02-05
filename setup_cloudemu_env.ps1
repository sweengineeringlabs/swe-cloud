# Setup CloudEmu Environment Variables
# Usage: . .\setup_cloudemu_env.ps1

Write-Host "Setting up CloudEmu environment variables..."

# 1. AWS Configuration (Port 4566)
$env:AWS_ACCESS_KEY_ID = "test"
$env:AWS_SECRET_ACCESS_KEY = "test"
$env:AWS_DEFAULT_REGION = "us-east-1"
$env:AWS_ENDPOINT_URL = "http://localhost:4566"
Write-Host "AWS Endpoint: $env:AWS_ENDPOINT_URL"

# 2. Azure Configuration (Port 10000)
# Note: Azure SDKs often use connection strings.
$env:AZURE_STORAGE_CONNECTION_STRING = "DefaultEndpointsProtocol=http;AccountName=devstoreaccount1;AccountKey=Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==;BlobEndpoint=http://127.0.0.1:10000/devstoreaccount1;"
$env:AZURE_COSMOS_CONNECTION_STRING = "AccountEndpoint=http://127.0.0.1:10000;AccountKey=C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw=="
# For generic control plane (covers Resource Manager & Retail Prices)
$env:AZURE_ENDPOINT_URL = "http://localhost:10000"
Write-Host "Azure Endpoint: $env:AZURE_ENDPOINT_URL"

# 3. GCP Configuration (Port 4567)
$env:GCP_PROJECT_ID = "local-test"
$env:GOOGLE_CLOUD_PROJECT = "local-test"
# Storage/Firestore/PubSub Emulators
$env:STORAGE_EMULATOR_HOST = "http://localhost:4567"
$env:FIRESTORE_EMULATOR_HOST = "localhost:4567" 
$env:PUBSUB_EMULATOR_HOST = "localhost:4567"
# General URL (covers Cloud Billing & generic APIs)
$env:GCP_ENDPOINT_URL = "http://localhost:4567"
Write-Host "GCP Endpoint: $env:GCP_ENDPOINT_URL"

Write-Host "Environment configured for CloudEmu integration."
