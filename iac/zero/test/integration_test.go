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
	zeroEndpoint = "http://localhost:8080"
)

// TestZeroIntegration tests the ZeroCloud provider integration in the IAC framework
func TestZeroIntegration(t *testing.T) {
	t.Parallel()

	// Ensure ZeroCloud is running
	ensureZeroRunning(t)

	timestamp := time.Now().Unix()
	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: "../../examples/zero-integration",
		Vars: map[string]interface{}{
			"bucket_name": fmt.Sprintf("test-zero-bucket-%d", timestamp),
			"table_name":  fmt.Sprintf("test-zero-table-%d", timestamp),
			"environment": "test",
		},
		NoColor: true,
	})

	// Clean up resources at the end of the test
	defer terraform.Destroy(t, terraformOptions)

	// Deploy infrastructure
	terraform.InitAndApply(t, terraformOptions)

	// 1. Verify Storage (ZeroStore)
	bucketName := terraform.Output(t, terraformOptions, "bucket_name")
	assert.NotEmpty(t, bucketName)
	
	bucketURL := terraform.Output(t, terraformOptions, "bucket_url")
	assert.Contains(t, bucketURL, fmt.Sprintf("/v1/store/buckets/%s", bucketName))

	// 2. Verify NoSQL (ZeroDB)
	tableName := terraform.Output(t, terraformOptions, "table_name")
	assert.NotEmpty(t, tableName)

	// 3. Verify Networking (ZeroNet)
	vpcID := terraform.Output(t, terraformOptions, "vpc_id")
	assert.NotEmpty(t, vpcID)
	assert.Contains(t, vpcID, "vpc-") // Zero uses AWS-style IDs

	// 4. Verify Identity (ZeroID)
	roleARN := terraform.Output(t, terraformOptions, "role_arn")
	assert.NotEmpty(t, roleARN)
	assert.Contains(t, roleARN, "arn:aws:iam") // Zero uses AWS-style ARNs

	// 5. Verify Compute (ZeroFunc)
	functionARN := terraform.Output(t, terraformOptions, "function_arn")
	assert.NotEmpty(t, functionARN)
	assert.Contains(t, functionARN, "arn:aws:lambda")

	// 6. Verify Messaging (ZeroQueue)
	queueURL := terraform.Output(t, terraformOptions, "queue_url")
	assert.NotEmpty(t, queueURL)
	// ZeroCloud typically runs on localhost:4566 (via cloudemu proxy) or 8080.
	// Since we are using AWS provider redirection, it might look like standard AWS URL or local one.
	// We just check it's not empty for now.

	t.Log("✓ ZeroCloud integration test successful")
}

// Helper Functions

func ensureZeroRunning(t *testing.T) {
	client := &http.Client{Timeout: 2 * time.Second}
	// We check the standard Zero API root or a known service path
	resp, err := client.Get(zeroEndpoint + "/v1/store/buckets")
	
	if err != nil || (resp.StatusCode != 200 && resp.StatusCode != 404) {
		t.Skip("ZeroCloud not running. Start with: cd cloudemu/zero && cargo run")
	}
	
	t.Log("✓ ZeroCloud is running")
}
