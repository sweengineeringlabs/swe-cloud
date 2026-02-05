package networking_test

import (
	"testing"
	"strings"

	"github.com/gruntwork-io/terratest/modules/terraform"
	"github.com/stretchr/testify/assert"
)

func TestNetworkingFacadeAws(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":     "aws",
			"project_name": "testproject",
			"environment":  "test",
			"network_name": "test-vpc",
			"metrics": map[string]interface{}{
				"cidr":            "10.0.0.0/16",
				"azs":             []string{"us-east-1a", "us-east-1b"},
				"public_subnets":  []string{"10.0.1.0/24", "10.0.2.0/24"},
				"private_subnets": []string{"10.0.11.0/24", "10.0.12.0/24"},
			},
		},
		BackendConfig: map[string]interface{}{},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)
	
	assert.True(t, strings.Contains(planString, "module.aws_networking[0].aws_vpc.this"), "Plan should create an AWS VPC")
	assert.True(t, strings.Contains(planString, "cidr_block = \"10.0.0.0/16\""), "Plan should have the correct CIDR block")
}

func TestNetworkingFacadeAzure(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":     "azure",
			"project_name": "testproject",
			"environment":  "test",
			"network_name": "test-vnet",
			"metrics": map[string]interface{}{
				"cidr":            "10.1.0.0/16",
				"azs":             []string{"1", "2"},
				"public_subnets":  []string{"10.1.1.0/24"},
				"private_subnets": []string{"10.1.11.0/24"},
			},
			"provider_config": map[string]interface{}{
				"resource_group_name": "test-rg",
				"location":            "eastus",
			},
		},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)

	assert.True(t, strings.Contains(planString, "module.azure_networking[0].azurerm_virtual_network.this"), "Plan should create an Azure VNet")
	assert.True(t, strings.Contains(planString, "address_space = [\"10.1.0.0/16\"]"), "Plan should have the correct address space")
}

func TestNetworkingFacadeGcp(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":     "gcp",
			"project_name": "testproject",
			"environment":  "test",
			"network_name": "test-network",
			"metrics": map[string]interface{}{
				"cidr":            "10.2.0.0/16",
				"azs":             []string{"us-central1-a"},
				"public_subnets":  []string{"10.2.1.0/24"},
				"private_subnets": []string{"10.2.11.0/24"},
			},
			"provider_config": map[string]interface{}{
				"region": "us-central1",
			},
		},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)

	assert.True(t, strings.Contains(planString, "module.gcp_networking[0].google_compute_network.this"), "Plan should create a GCP Network")
	assert.True(t, strings.Contains(planString, "name = \"test-network\""), "Plan should have the correct network name")
}

func TestNetworkingFacadeInvalidCidr(t *testing.T) {
	t.Parallel()

	terraformOptions := &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":     "aws",
			"project_name": "testproject",
			"environment":  "test",
			"network_name": "test-vpc",
			"metrics": map[string]interface{}{
				"cidr":            "999.0.0.0/16", // Invalid CIDR
				"azs":             []string{"us-east-1a"},
				"public_subnets":  []string{"10.0.1.0/24"},
				"private_subnets": []string{"10.0.11.0/24"},
			},
		},
	}

	_, err := terraform.InitAndPlanE(t, terraformOptions)
	assert.Error(t, err, "Plan should fail with an invalid CIDR block")
}
