package messaging_test

import (
	"testing"
	"strings"

	"github.com/gruntwork-io/terratest/modules/terraform"
	"github.com/stretchr/testify/assert"
)

func TestMessagingFacadeAwsQueue(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":     "aws",
			"project_name": "testproject",
			"environment":  "test",
			"name":         "test-queue",
			"type":         "queue",
		},
		BackendConfig: map[string]interface{}{},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)
	
	assert.True(t, strings.Contains(planString, "module.aws_messaging[0].aws_sqs_queue.this"), "Plan should create an AWS SQS queue")
	assert.True(t, strings.Contains(planString, "name = \"test-queue\""), "Plan should have the correct queue name")
}

func TestMessagingFacadeAwsTopic(t *testing.T) {
	t.Parallel()

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: ".",
		Vars: map[string]interface{}{
			"provider":     "aws",
			"project_name": "testproject",
			"environment":  "test",
			"name":         "test-topic-sns",
			"type":         "topic",
		},
		BackendConfig: map[string]interface{}{},
	})

	planString := terraform.InitAndPlan(t, terraformOptions)
	
	assert.True(t, strings.Contains(planString, "module.aws_messaging[0].aws_sns_topic.this"), "Plan should create an AWS SNS topic")
	assert.True(t, strings.Contains(planString, "name = \"test-topic-sns\""), "Plan should have the correct topic name")
}
