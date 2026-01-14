package iam_test

import (
	"testing"
	"strings"

	"github.com/gruntwork-io/terratest/modules/terraform"
	"github.com/stretchr/testify/assert"
)

func TestIamFacadeAws(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":      "aws",
			"project_name":  "testproject",
			"environment":   "test",
			"identity_name": "test-role",
			"identity_type": "role",
			"principals":    []string{"ec2.amazonaws.com"},
		},
		BackendConfig: map[string]interface{}{},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)
	
	assert.True(t, strings.Contains(planString, "module.aws_iam[0].aws_iam_role.this"), "Plan should create an AWS IAM role")
	assert.True(t, strings.Contains(planString, "name = \"test-role\""), "Plan should have the correct role name")
}

func TestIamFacadeAzure(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":      "azure",
			"project_name":  "testproject",
			"environment":   "test",
			"identity_name": "test-id",
			"identity_type": "user",
			"provider_config": map[string]interface{}{
				"resource_group_name": "test-rg",
				"location":            "eastus",
			},
		},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)

	assert.True(t, strings.Contains(planString, "module.azure_iam[0].azurerm_user_assigned_identity.this"), "Plan should create an Azure User Assigned Identity")
	assert.True(t, strings.Contains(planString, "name = \"test-id\""), "Plan should have the correct identity name")
}

func TestIamFacadeGcp(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":      "gcp",
			"project_name":  "testproject",
			"environment":   "test",
			"identity_name": "test-sa-unique",
			"identity_type": "service_agent",
			"provider_config": map[string]interface{}{
				"project_id": "test-project",
			},
		},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)

	assert.True(t, strings.Contains(planString, "module.gcp_iam[0].google_service_account.this"), "Plan should create a GCP Service Account")
	assert.True(t, strings.Contains(planString, "account_id = \"test-sa-unique\""), "Plan should have the correct account ID")
}

func TestIamFacadeInvalidProvider(t *testing.T) {
	t.Parallel()

	terraformOptions := &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":      "invalid-cloud", // Should fail validation
			"project_name":  "testproject",
			"environment":   "test",
			"identity_name": "test-role",
		},
	}

	_, err := terraform.InitAndPlanE(t, terraformOptions)
	assert.Error(t, err, "Plan should fail with an invalid provider")
}
