package test

import (
	"fmt"
	"net/http"
	"os"
	"os/exec"
	"testing"
	"time"

	"github.com/gruntwork-io/terratest/modules/terraform"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

const (
	cloudEmuEndpoint = "http://localhost:4566"
	healthCheckPath  = "/health"
)

// TestCloudEmuStorageFacade tests the storage facade with CloudEmu
func TestCloudEmuStorageFacade(t *testing.T) {
	t.Parallel()

	// Ensure CloudEmu is running
	ensureCloudEmuRunning(t)

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: "../../examples/local-cloudemu",
		Vars: map[string]interface{}{
			"bucket_name":   fmt.Sprintf("test-bucket-%d", time.Now().Unix()),
			"environment":   "test",
		},
		NoColor: true,
	})

	// Clean up resources
	defer terraform.Destroy(t, terraformOptions)

	// Deploy infrastructure
	terraform.InitAndApply(t, terraformOptions)

	// Verify outputs
	bucketName := terraform.Output(t, terraformOptions, "bucket_name")
	assert.NotEmpty(t, bucketName)

	bucketARN := terraform.Output(t, terraformOptions, "bucket_arn")
	assert.Contains(t, bucketARN, bucketName)

	// Verify bucket exists in CloudEmu
	verifyS3BucketExists(t, bucketName)

	// Test S3 operations
	testS3Upload(t, bucketName)
	testS3Download(t, bucketName)
}

// TestCloudEmuDatabaseFacade tests the database facade with CloudEmu
func TestCloudEmuDatabaseFacade(t *testing.T) {
	t.Parallel()

	ensureCloudEmuRunning(t)

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: "../../examples/local-cloudemu",
		Vars: map[string]interface{}{
			"database_name": fmt.Sprintf("test-table-%d", time.Now().Unix()),
			"environment":   "test",
		},
		NoColor: true,
	})

	defer terraform.Destroy(t, terraformOptions)
	terraform.InitAndApply(t, terraformOptions)

	tableName := terraform.Output(t, terraformOptions, "table_name")
	assert.NotEmpty(t, tableName)

	// Verify table exists
	verifyDynamoDBTableExists(t, tableName)

	// Test DynamoDB operations
	testDynamoDBPutItem(t, tableName)
	testDynamoDBGetItem(t, tableName)
}

// TestCloudEmuMessagingFacade tests the messaging facade with CloudEmu
func TestCloudEmuMessagingFacade(t *testing.T) {
	t.Parallel()

	ensureCloudEmuRunning(t)

	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: "../../examples/local-cloudemu",
		Vars: map[string]interface{}{
			"queue_name":  fmt.Sprintf("test-queue-%d", time.Now().Unix()),
			"topic_name":  fmt.Sprintf("test-topic-%d", time.Now().Unix()),
			"environment": "test",
		},
		NoColor: true,
	})

	defer terraform.Destroy(t, terraformOptions)
	terraform.InitAndApply(t, terraformOptions)

	queueURL := terraform.Output(t, terraformOptions, "queue_url")
	assert.NotEmpty(t, queueURL)

	topicARN := terraform.Output(t, terraformOptions, "topic_arn")
	assert.NotEmpty(t, topicARN)

	// Test SQS operations
	testSQSSendMessage(t, queueURL)
	testSQSReceiveMessage(t, queueURL)

	// Test SNS operations
	testSNSPublish(t, topicARN)
}

// TestCloudEmuFullStack tests deploying all services together
func TestCloudEmuFullStack(t *testing.T) {
	t.Parallel()

	ensureCloudEmuRunning(t)

	timestamp := time.Now().Unix()
	terraformOptions := terraform.WithDefaultRetryableErrors(t, &terraform.Options{
		TerraformDir: "../../examples/local-cloudemu",
		Vars: map[string]interface{}{
			"bucket_name":   fmt.Sprintf("fullstack-bucket-%d", timestamp),
			"database_name": fmt.Sprintf("fullstack-table-%d", timestamp),
			"queue_name":    fmt.Sprintf("fullstack-queue-%d", timestamp),
			"topic_name":    fmt.Sprintf("fullstack-topic-%d", timestamp),
			"function_name": fmt.Sprintf("fullstack-fn-%d", timestamp),
			"environment":   "test",
		},
		NoColor: true,
	})

	defer terraform.Destroy(t, terraformOptions)
	terraform.InitAndApply(t, terraformOptions)

	// Verify all resources created
	bucketName := terraform.Output(t, terraformOptions, "bucket_name")
	tableName := terraform.Output(t, terraformOptions, "table_name")
	queueURL := terraform.Output(t, terraformOptions, "queue_url")
	topicARN := terraform.Output(t, terraformOptions, "topic_arn")
	functionName := terraform.Output(t, terraformOptions, "function_name")

	assert.NotEmpty(t, bucketName)
	assert.NotEmpty(t, tableName)
	assert.NotEmpty(t, queueURL)
	assert.NotEmpty(t, topicARN)
	assert.NotEmpty(t, functionName)

	// Verify resources exist in CloudEmu
	verifyS3BucketExists(t, bucketName)
	verifyDynamoDBTableExists(t, tableName)
	verifySQSQueueExists(t, queueURL)
	verifySNSTopicExists(t, topicARN)
	verifyLambdaFunctionExists(t, functionName)

	t.Log("✓ Full stack deployment successful")
}

// Helper Functions

func ensureCloudEmuRunning(t *testing.T) {
	client := &http.Client{Timeout: 2 * time.Second}
	resp, err := client.Get(cloudEmuEndpoint + healthCheckPath)
	
	if err != nil || resp.StatusCode != 200 {
		t.Skip("CloudEmu not running. Start with: cd cloudemu && cargo run --release -p cloudemu-server")
	}
	
	t.Log("✓ CloudEmu is running")
}

func awsCommand(args ...string) *exec.Cmd {
	cmdArgs := append([]string{"--endpoint-url", cloudEmuEndpoint}, args...)
	return exec.Command("aws", cmdArgs...)
}

func verifyS3BucketExists(t *testing.T, bucketName string) {
	cmd := awsCommand("s3", "ls", "s3://"+bucketName)
	output, err := cmd.CombinedOutput()
	require.NoError(t, err, "Bucket %s should exist. Output: %s", bucketName, string(output))
	t.Logf("✓ S3 bucket %s exists", bucketName)
}

func verifyDynamoDBTableExists(t *testing.T, tableName string) {
	cmd := awsCommand("dynamodb", "describe-table", "--table-name", tableName)
	output, err := cmd.CombinedOutput()
	require.NoError(t, err, "Table %s should exist. Output: %s", tableName, string(output))
	t.Logf("✓ DynamoDB table %s exists", tableName)
}

func verifySQSQueueExists(t *testing.T, queueURL string) {
	cmd := awsCommand("sqs", "get-queue-attributes", "--queue-url", queueURL, "--attribute-names", "All")
	output, err := cmd.CombinedOutput()
	require.NoError(t, err, "Queue %s should exist. Output: %s", queueURL, string(output))
	t.Logf("✓ SQS queue exists at %s", queueURL)
}

func verifySNSTopicExists(t *testing.T, topicARN string) {
	cmd := awsCommand("sns", "get-topic-attributes", "--topic-arn", topicARN)
	output, err := cmd.CombinedOutput()
	require.NoError(t, err, "Topic %s should exist. Output: %s", topicARN, string(output))
	t.Logf("✓ SNS topic exists: %s", topicARN)
}

func verifyLambdaFunctionExists(t *testing.T, functionName string) {
	cmd := awsCommand("lambda", "get-function", "--function-name", functionName)
	output, err := cmd.CombinedOutput()
	require.NoError(t, err, "Function %s should exist. Output: %s", functionName, string(output))
	t.Logf("✓ Lambda function %s exists", functionName)
}

func testS3Upload(t *testing.T, bucketName string) {
	// Create test file
	testFile := "/tmp/cloudemu-test.txt"
	err := os.WriteFile(testFile, []byte("Hello from Terratest!"), 0644)
	require.NoError(t, err)
	defer os.Remove(testFile)

	// Upload to S3
	cmd := awsCommand("s3", "cp", testFile, fmt.Sprintf("s3://%s/test.txt", bucketName))
	output, err := cmd.CombinedOutput()
	require.NoError(t, err, "Failed to upload to S3: %s", string(output))
	t.Logf("✓ Uploaded file to S3 bucket %s", bucketName)
}

func testS3Download(t *testing.T, bucketName string) {
	downloadFile := "/tmp/cloudemu-download.txt"
	defer os.Remove(downloadFile)

	cmd := awsCommand("s3", "cp", fmt.Sprintf("s3://%s/test.txt", bucketName), downloadFile)
	output, err := cmd.CombinedOutput()
	require.NoError(t, err, "Failed to download from S3: %s", string(output))

	content, err := os.ReadFile(downloadFile)
	require.NoError(t, err)
	assert.Equal(t, "Hello from Terratest!", string(content))
	t.Logf("✓ Downloaded and verified file from S3")
}

func testDynamoDBPutItem(t *testing.T, tableName string) {
	item := `{"id": {"S": "test-id-1"}, "name": {"S": "Test Item"}}`
	cmd := awsCommand("dynamodb", "put-item", "--table-name", tableName, "--item", item)
	output, err := cmd.CombinedOutput()
	require.NoError(t, err, "Failed to put item: %s", string(output))
	t.Logf("✓ Put item to DynamoDB table %s", tableName)
}

func testDynamoDBGetItem(t *testing.T, tableName string) {
	key := `{"id": {"S": "test-id-1"}}`
	cmd := awsCommand("dynamodb", "get-item", "--table-name", tableName, "--key", key)
	output, err := cmd.CombinedOutput()
	require.NoError(t, err, "Failed to get item: %s", string(output))
	assert.Contains(t, string(output), "Test Item")
	t.Logf("✓ Got item from DynamoDB table %s", tableName)
}

func testSQSSendMessage(t *testing.T, queueURL string) {
	cmd := awsCommand("sqs", "send-message", "--queue-url", queueURL, "--message-body", "Test message from Terratest")
	output, err := cmd.CombinedOutput()
	require.NoError(t, err, "Failed to send message: %s", string(output))
	t.Logf("✓ Sent message to SQS queue")
}

func testSQSReceiveMessage(t *testing.T, queueURL string) {
	cmd := awsCommand("sqs", "receive-message", "--queue-url", queueURL)
	output, err := cmd.CombinedOutput()
	require.NoError(t, err, "Failed to receive message: %s", string(output))
	assert.Contains(t, string(output), "Test message")
	t.Logf("✓ Received message from SQS queue")
}

func testSNSPublish(t *testing.T, topicARN string) {
	cmd := awsCommand("sns", "publish", "--topic-arn", topicARN, "--message", "Test message from Terratest")
	output, err := cmd.CombinedOutput()
	require.NoError(t, err, "Failed to publish to SNS: %s", string(output))
	t.Logf("✓ Published message to SNS topic")
}
