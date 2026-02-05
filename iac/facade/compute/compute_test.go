package compute_test

import (
	"testing"
	"strings"

	"github.com/gruntwork-io/terratest/modules/terraform"
	"github.com/stretchr/testify/assert"
)

func TestComputeFacadeAws(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":      "aws",
			"project_name":  "testproject",
			"environment":   "test",
			"instance_name": "test-instance",
			"instance_size": "small",
			"provider_config": map[string]interface{}{
				"ami": "ami-0c55b159cbfafe1f0",
			},
		},
		BackendConfig: map[string]interface{}{},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)
	
	assert.True(t, strings.Contains(planString, "module.aws_compute[0].aws_instance.this"), "Plan should create an AWS EC2 instance")
	assert.True(t, strings.Contains(planString, "instance_type = \"t3.micro\""), "Plan should have the correct instance type for 'small'")
}

func TestComputeFacadeAzure(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":      "azure",
			"project_name":  "testproject",
			"environment":   "test",
			"instance_name": "test-instance",
			"instance_size": "medium",
			"provider_config": map[string]interface{}{
				"resource_group_name": "test-rg",
				"location":            "eastus",
			},
		},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)

	assert.True(t, strings.Contains(planString, "module.azure_compute[0].azurerm_linux_virtual_machine.this"), "Plan should create an Azure VM")
	assert.True(t, strings.Contains(planString, "size = \"Standard_B2s\""), "Plan should have the correct VM size for 'medium'")
}

func TestComputeFacadeGcp(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":      "gcp",
			"project_name":  "testproject",
			"environment":   "test",
			"instance_name": "test-instance",
			"instance_size": "large",
			"provider_config": map[string]interface{}{
				"project_id": "test-project",
				"zone":       "us-central1-a",
			},
		},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)

	assert.True(t, strings.Contains(planString, "module.gcp_compute[0].google_compute_instance.this"), "Plan should create a GCP Compute Instance")
	assert.True(t, strings.Contains(planString, "machine_type = \"n2-standard-2\""), "Plan should have the correct machine type for 'large'")
}

func TestComputeFacadeInvalidName(t *testing.T) {
	t.Parallel()

	terraformOptions := &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":      "aws",
			"project_name":  "testproject",
			"environment":   "test",
			"instance_name": "UPPERCASE_NOT_ALLOWED",
			"instance_size": "small",
			"provider_config": map[string]interface{}{
				"ami": "ami-0c55b159cbfafe1f0",
			},
		},
	}

	_, err := terraform.InitAndPlanE(t, terraformOptions)
	assert.Error(t, err, "Plan should fail with an invalid instance name")
}
