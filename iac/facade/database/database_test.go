package database_test

import (
	"testing"
	"strings"

	"github.com/gruntwork-io/terratest/modules/terraform"
	"github.com/stretchr/testify/assert"
)

func TestDatabaseFacadeAws(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":             "aws",
			"project_name":         "testproject",
			"environment":          "test",
			"identifier":           "test-db",
			"master_password":      "password123",
			"allocated_storage_gb": 20,
		},
		BackendConfig: map[string]interface{}{},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)
	
	assert.True(t, strings.Contains(planString, "module.aws_database[0].aws_db_instance.this"), "Plan should create an AWS RDS instance")
	assert.True(t, strings.Contains(planString, "instance_class = \"db.t3.micro\""), "Plan should have the correct instance class for 'small'")
}

func TestDatabaseFacadeAzure(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":             "azure",
			"project_name":         "testproject",
			"environment":          "test",
			"identifier":           "test-db",
			"instance_class":       "medium",
			"master_password":      "password123",
			"allocated_storage_gb": 20,
			"provider_config": map[string]interface{}{
				"resource_group_name": "test-rg",
				"location":            "eastus",
			},
		},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)

	assert.True(t, strings.Contains(planString, "module.azure_database[0].azurerm_mssql_server.this"), "Plan should create an Azure SQL Server")
	assert.True(t, strings.Contains(planString, "sku_name = \"S1\""), "Plan should have the correct SKU name for 'medium'")
}

func TestDatabaseFacadeGcp(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":             "gcp",
			"project_name":         "testproject",
			"environment":          "test",
			"identifier":           "test-db",
			"instance_class":       "large",
			"master_password":      "password123",
			"allocated_storage_gb": 20,
			"provider_config": map[string]interface{}{
				"region": "us-central1",
			},
		},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)

	assert.True(t, strings.Contains(planString, "module.gcp_database[0].google_sql_database_instance.this"), "Plan should create a GCP SQL Instance")
	assert.True(t, strings.Contains(planString, "tier = \"db-n1-standard-1\""), "Plan should have the correct tier for 'large'")
}

func TestDatabaseFacadeInvalidPassword(t *testing.T) {
	t.Parallel()

	terraformOptions := &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":             "aws",
			"project_name":         "testproject",
			"environment":          "test",
			"identifier":           "test-db",
			"master_password":      "short", // Many providers require min length
			"allocated_storage_gb": 20,
		},
	}

	// This might fail either at Terraform validation OR provider validation during plan
	_, err := terraform.InitAndPlanE(t, terraformOptions)
	assert.Error(t, err, "Plan should fail with a weak password")
}
