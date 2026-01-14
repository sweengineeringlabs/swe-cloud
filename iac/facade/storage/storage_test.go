package storage_test

import (
	"testing"
	"strings"

	"github.com/gruntwork-io/terratest/modules/terraform"
	"github.com/stretchr/testify/assert"
)

// TestStorageFacadeAws verifies the Storage Facade creates an S3 bucket
func TestStorageFacadeAws(t *testing.T) {
	t.Parallel()

	// 1. Configure Terraform options
	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		// Path to the Terraform module we want to test.
        // Since the test is now colocated, we use the current directory.
		TerraformDir: ".",

		// Variables to pass to our module using -var options
		Vars: map[string]interface{}{
			"provider":      "aws",
			"project_name":  "testproject",
			"environment":   "test",
			"bucket_name":   "unit-test-bucket",
			"storage_class": "STANDARD",
		},
		
		// Disable backend to avoid remote state locking during tests
		BackendConfig: map[string]interface{}{},
	})

	// 2. Defer destroy (cleanup) - though for Unit Tests we might skip 'apply'
	// cleanup is only needed if we actually provision resources.
	// defer terraform.Destroy(t, terraformOptions)

	// 3. Run 'terraform init' and 'terraform plan'
	// We use Plan (not Apply) for Unit Testing to avoid costs/cloud deps.
	planString := terraform.InitAndPlan(t, terraformOptions)
	
	// 4. Validate the Plan Outcome
	// We expect 1 resource to be added (the S3 bucket)
	// Output looks like: "Plan: 1 to add, 0 to change, 0 to destroy."
	
	// Check that we are creating the correct resource
	assert.True(t, strings.Contains(planString, "module.aws_storage[0].aws_s3_bucket.this"), "Plan should create an AWS S3 bucket")
	assert.True(t, strings.Contains(planString, "1 to add"), "Plan should propose adding 1 resource")
}

// TestStorageFacadeAzure verifies provider switching works
func TestStorageFacadeAzure(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":      "azure",
			"project_name":  "testproject",
			"environment":   "test",
			"bucket_name":   "unittestbucket", 
			"provider_config": map[string]interface{}{
				"resource_group_name": "test-rg",
				"location": "eastus",
			},
		},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)

	// Validate Azure switching logic
	assert.True(t, strings.Contains(planString, "module.azure_storage[0].azurerm_storage_account.this"), "Plan should create an Azure Storage Account")
}
