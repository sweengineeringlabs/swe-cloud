package test

import (
	"fmt"
	"net/http"
	"testing"
	"time"

	"github.com/gruntwork-io/terratest/modules/terraform"
	"github.com/stretchr/testify/assert"
)

const (
	azureEndpoint = "http://localhost:10000"
)

// TestAzureIntegration tests the Azure provider integration with CloudEmu
func TestAzureIntegration(t *testing.T) {
	t.Parallel()

	ensureAzureRunning(t)

	timestamp := time.Now().Unix()
	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: "../../examples/azure-integration",
		Vars: map[string]interface{}{
			"bucket_name": fmt.Sprintf("test-azure-container-%d", timestamp),
			"table_name":  fmt.Sprintf("test-azure-cosmos-%d", timestamp),
			"environment": "test",
		},
		NoColor: true,
	})

	defer terraform.Destroy(t, terraformOptions)
	terraform.InitAndApply(t, terraformOptions)

	// 1. Verify Storage (Azure Blob)
	bucketName := terraform.Output(t, terraformOptions, "bucket_name")
	assert.NotEmpty(t, bucketName)

	bucketURL := terraform.Output(t, terraformOptions, "bucket_url")
	assert.Contains(t, bucketURL, bucketName)

	// 2. Verify NoSQL (Cosmos DB)
	tableName := terraform.Output(t, terraformOptions, "table_name")
	assert.NotEmpty(t, tableName)

	// 3. Verify Networking (VNet)
	vnetID := terraform.Output(t, terraformOptions, "vnet_id")
	assert.NotEmpty(t, vnetID)

	// 4. Verify Identity (Managed Identity)
	identityID := terraform.Output(t, terraformOptions, "identity_id")
	assert.NotEmpty(t, identityID)

	// 5. Verify Compute (Function)
	functionName := terraform.Output(t, terraformOptions, "function_name")
	assert.NotEmpty(t, functionName)

	// 6. Verify Messaging (Service Bus Queue)
	queueURL := terraform.Output(t, terraformOptions, "queue_url")
	assert.NotEmpty(t, queueURL)

	t.Log("✓ Azure integration test successful")
}

func ensureAzureRunning(t *testing.T) {
	client := &http.Client{Timeout: 2 * time.Second}
	// Check Azure Blob endpoint
	resp, err := client.Get(azureEndpoint + "/devstoreaccount1")
	
	if err != nil || (resp.StatusCode != 200 && resp.StatusCode != 400 && resp.StatusCode != 404) {
		t.Skip("CloudEmu (Azure) not running. Start with: cd cloudemu && cargo run --release -p cloudemu-server")
	}
	
	t.Log("✓ CloudEmu (Azure) is running")
}
