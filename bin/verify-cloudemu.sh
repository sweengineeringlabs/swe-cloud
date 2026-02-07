#!/usr/bin/env bash
# bin/verify-cloudemu.sh — verify CloudEmu deployment
source "$(cd "$(dirname "$0")/.." && pwd)/lib/common.sh"

ENDPOINT="${CLOUDEMU_ENDPOINT:-http://localhost:4566}"

require_cmd aws || exit 1

echo "${BOLD}CloudEmu Deployment Verification${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# ── Check connectivity ───────────────────────────────────────────────
echo "1. Checking CloudEmu connectivity..."
if curl -s -f "$ENDPOINT/health" > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} CloudEmu is running on $ENDPOINT"
else
    echo -e "${RED}✗${NC} CloudEmu is not responding"
    echo "   Start CloudEmu with: cargo run --release -p cloudemu-server"
    exit 1
fi
echo ""

# ── Check S3 buckets ────────────────────────────────────────────────
echo "2. Verifying S3 buckets..."
BUCKETS=$(aws --endpoint-url=$ENDPOINT s3 ls 2>/dev/null | wc -l)
if [ "$BUCKETS" -gt 0 ]; then
    echo -e "${GREEN}✓${NC} Found $BUCKETS bucket(s):"
    aws --endpoint-url=$ENDPOINT s3 ls | sed 's/^/   /'
else
    echo -e "${RED}✗${NC} No buckets found"
fi
echo ""

# ── Check DynamoDB tables ───────────────────────────────────────────
echo "3. Verifying DynamoDB tables..."
TABLES=$(aws --endpoint-url=$ENDPOINT dynamodb list-tables --output text --query 'TableNames' 2>/dev/null | wc -w)
if [ "$TABLES" -gt 0 ]; then
    echo -e "${GREEN}✓${NC} Found $TABLES table(s):"
    aws --endpoint-url=$ENDPOINT dynamodb list-tables --output text --query 'TableNames' | tr '\t' '\n' | sed 's/^/   /'
else
    echo -e "${RED}✗${NC} No tables found"
fi
echo ""

# ── Check SQS queues ────────────────────────────────────────────────
echo "4. Verifying SQS queues..."
QUEUES=$(aws --endpoint-url=$ENDPOINT sqs list-queues --output text --query 'QueueUrls' 2>/dev/null | wc -w)
if [ "$QUEUES" -gt 0 ]; then
    echo -e "${GREEN}✓${NC} Found $QUEUES queue(s):"
    aws --endpoint-url=$ENDPOINT sqs list-queues --output text --query 'QueueUrls' | tr '\t' '\n' | sed 's/^/   /'
else
    echo -e "${RED}✗${NC} No queues found"
fi
echo ""

# ── Check SNS topics ────────────────────────────────────────────────
echo "5. Verifying SNS topics..."
TOPICS=$(aws --endpoint-url=$ENDPOINT sns list-topics --output text --query 'Topics[*].TopicArn' 2>/dev/null | wc -w)
if [ "$TOPICS" -gt 0 ]; then
    echo -e "${GREEN}✓${NC} Found $TOPICS topic(s):"
    aws --endpoint-url=$ENDPOINT sns list-topics --output text --query 'Topics[*].TopicArn' | tr '\t' '\n' | sed 's/^/   /'
else
    echo -e "${RED}✗${NC} No topics found"
fi
echo ""

# ── Check Lambda functions ──────────────────────────────────────────
echo "6. Verifying Lambda functions..."
FUNCTIONS=$(aws --endpoint-url=$ENDPOINT lambda list-functions --output text --query 'Functions[*].FunctionName' 2>/dev/null | wc -w)
if [ "$FUNCTIONS" -gt 0 ]; then
    echo -e "${GREEN}✓${NC} Found $FUNCTIONS function(s):"
    aws --endpoint-url=$ENDPOINT lambda list-functions --output text --query 'Functions[*].FunctionName' | tr '\t' '\n' | sed 's/^/   /'
else
    echo -e "${RED}✗${NC} No functions found"
fi
echo ""

# ── Summary ──────────────────────────────────────────────────────────
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Verification Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "S3 Buckets:       $BUCKETS"
echo "DynamoDB Tables:  $TABLES"
echo "SQS Queues:       $QUEUES"
echo "SNS Topics:       $TOPICS"
echo "Lambda Functions: $FUNCTIONS"
echo ""

if [ "$BUCKETS" -gt 0 ] && [ "$TABLES" -gt 0 ]; then
    echo -e "${GREEN}✓ CloudEmu deployment verified successfully!${NC}"
    exit 0
else
    echo -e "${RED}✗ Some resources are missing. Run 'terraform apply' first.${NC}"
    exit 1
fi
