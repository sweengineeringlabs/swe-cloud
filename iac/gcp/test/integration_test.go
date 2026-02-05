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
	gcpEndpoint = "http://localhost:4567"
)

// TestGCPIntegration tests the GCP provider integration with CloudEmu
func TestGCPIntegration(t *testing.T) {
	t.Parallel()

	ensureGCPRunning(t)

	timestamp := time.Now().Unix()
	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: "../../examples/gcp-integration",
		Vars: map[string]interface{}{
			"bucket_name": fmt.Sprintf("test-gcp-bucket-%d", timestamp),
			"table_name":  fmt.Sprintf("test-gcp-collection-%d", timestamp),
			"environment": "test",
		},
		NoColor: true,
	})

	defer terraform.Destroy(t, terraformOptions)
	terraform.InitAndApply(t, terraformOptions)

	// 1. Verify Storage (GCS)
	bucketName := terraform.Output(t, terraformOptions, "bucket_name")
	assert.NotEmpty(t, bucketName)

	bucketURL := terraform.Output(t, terraformOptions, "bucket_url")
	assert.NotEmpty(t, bucketURL)

	// 2. Verify NoSQL (Firestore)
	tableName := terraform.Output(t, terraformOptions, "table_name")
	assert.NotEmpty(t, tableName)

	// 3. Verify Networking (VPC)
	vpcID := terraform.Output(t, terraformOptions, "vpc_id")
	assert.NotEmpty(t, vpcID)

	// 4. Verify Identity (Service Account)
	saEmail := terraform.Output(t, terraformOptions, "sa_email")
	assert.NotEmpty(t, saEmail)

	// 5. Verify Compute (Cloud Function)
	functionName := terraform.Output(t, terraformOptions, "function_name")
	assert.NotEmpty(t, functionName)

	// 6. Verify Messaging (Pub/Sub)
	topicARN := terraform.Output(t, terraformOptions, "topic_arn")
	assert.NotEmpty(t, topicARN)

	t.Log("✓ GCP integration test successful")
}

func ensureGCPRunning(t *testing.T) {
	client := &http.Client{Timeout: 2 * time.Second}
	// Check GCS endpoint
	resp, err := client.Get(gcpEndpoint)
	
	if err != nil {
		t.Skip("CloudEmu (GCP) not running. Start with: cd cloudemu && cargo run --release -p cloudemu-server")
	}
	
	t.Log("✓ CloudEmu (GCP) is running")
}
