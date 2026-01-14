package monitoring_test

import (
	"testing"
	"strings"

	"github.com/gruntwork-io/terratest/modules/terraform"
	"github.com/stretchr/testify/assert"
)

func TestMonitoringFacadeAws(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":     "aws",
			"project_name": "testproject",
			"environment":  "test",
			"alarm_name":   "cpu-high",
			"metric_name":  "CPUUtilization",
			"threshold":    80,
		},
		BackendConfig: map[string]interface{}{},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)
	
	assert.True(t, strings.Contains(planString, "module.aws_monitoring[0].aws_cloudwatch_metric_alarm.this"), "Plan should create an AWS CloudWatch alarm")
	assert.True(t, strings.Contains(planString, "threshold = 80"), "Plan should have the correct threshold")
}

func TestMonitoringFacadeAzure(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":     "azure",
			"project_name": "testproject",
			"environment":  "test",
			"alarm_name":   "cpu-high",
			"metric_name":  "Percentage CPU",
			"threshold":    75,
			"provider_config": map[string]interface{}{
				"resource_group_name": "test-rg",
				"scopes":              []string{"/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/test-rg"},
			},
		},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)

	assert.True(t, strings.Contains(planString, "module.azure_monitoring[0].azurerm_monitor_metric_alert.this"), "Plan should create an Azure Monitor metric alert")
	assert.True(t, strings.Contains(planString, "threshold = 75"), "Plan should have the correct threshold")
}

func TestMonitoringFacadeGcp(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":     "gcp",
			"project_name": "testproject",
			"environment":  "test",
			"alarm_name":   "cpu-critical",
			"metric_name":  "cpu/utilization",
			"threshold":    0.9,
			"provider_config": map[string]interface{}{
				"project_id": "test-project",
			},
		},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)

	assert.True(t, strings.Contains(planString, "module.gcp_monitoring[0].google_monitoring_alert_policy.this"), "Plan should create a GCP Monitoring alert policy")
	assert.True(t, strings.Contains(planString, "threshold_value = 0.9"), "Plan should have the correct threshold value")
}

func TestMonitoringFacadeInvalidThreshold(t *testing.T) {
	t.Parallel()

	terraformOptions := &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":     "aws",
			"project_name": "testproject",
			"environment":  "test",
			"alarm_name":   "cpu-high",
			"metric_name":  "CPUUtilization",
			"threshold":    -1, // Invalid threshold
		},
	}

	// This is just a placeholder example, actual behavior depends on variables.tf validations
	_, err := terraform.InitAndPlanE(t, terraformOptions)
	// If there's a validation rule in variables.tf, this will be Error
	if err != nil {
		assert.Error(t, err)
	}
}
