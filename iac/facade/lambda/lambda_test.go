package lambda_test

import (
	"testing"
	"strings"

	"github.com/gruntwork-io/terratest/modules/terraform"
	"github.com/stretchr/testify/assert"
)

func TestLambdaFacadeAws(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":      "aws",
			"project_name":  "testproject",
			"environment":   "test",
			"function_name": "test-function",
			"handler":       "index.handler",
			"runtime":       "python3.9",
		},
		BackendConfig: map[string]interface{}{},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)
	
	assert.True(t, strings.Contains(planString, "module.aws_lambda[0].aws_lambda_function.this"), "Plan should create an AWS Lambda function")
	assert.True(t, strings.Contains(planString, "function_name = \"test-function\""), "Plan should have the correct function name")
}
