#!/usr/bin/env bash
#
# Integration tests for AWS Config service within aws-inmemory-services.
#
set -uo pipefail

PORT=19500
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

aws_config() {
  aws configservice "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

assert_contains() {
  local label="$1" output="$2" expected="$3"
  if echo "$output" | grep -qF "$expected"; then
    PASS=$((PASS + 1))
    TESTS+=("PASS  $label")
  else
    FAIL=$((FAIL + 1))
    TESTS+=("FAIL  $label  (expected '$expected' in output)")
    echo "FAIL: $label" >&2
    echo "  expected: $expected" >&2
    echo "  output:   $output" >&2
  fi
}

assert_not_contains() {
  local label="$1" output="$2" unexpected="$3"
  if echo "$output" | grep -qF "$unexpected"; then
    FAIL=$((FAIL + 1))
    TESTS+=("FAIL  $label  (did not expect '$unexpected' in output)")
    echo "FAIL: $label" >&2
    echo "  unexpected: $unexpected" >&2
    echo "  output:     $output" >&2
  else
    PASS=$((PASS + 1))
    TESTS+=("PASS  $label")
  fi
}

cleanup() {
  if [[ -n "${SERVER_PID:-}" ]]; then
    kill "$SERVER_PID" 2>/dev/null || true
    wait "$SERVER_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

echo "Building..."
cargo build --quiet 2>&1

lsof -ti:${PORT} | xargs kill 2>/dev/null || true
sleep 0.5

echo "Starting server with Config on port ${PORT}..."
"$BINARY" \
  --config-port "$PORT" \
  --s3-port 19501 --sns-port 19502 --sqs-port 19503 --dynamodb-port 19504 \
  --lambda-port 19505 --firehose-port 19506 --memorydb-port 19507 \
  --cognito-port 19508 --apigateway-port 19509 --kms-port 19510 \
  --secretsmanager-port 19511 --kinesis-port 19512 --eventbridge-port 19513 \
  --stepfunctions-port 19514 --ssm-port 19515 --cloudwatchlogs-port 19516 \
  --ses-port 19517 --servicecatalog-port 19518 \
  --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "ERROR: server failed to start"
  exit 1
fi

echo "Running Config integration tests..."

# 1. PutConfigurationRecorder
OUT=$(aws_config put-configuration-recorder \
  --configuration-recorder name=default,roleARN=arn:aws:iam::000000000000:role/config-role \
  --recording-group allSupported=true,includeGlobalResourceTypes=false)
assert_contains "PutConfigurationRecorder" "$OUT" ""

# 2. DescribeConfigurationRecorders
OUT=$(aws_config describe-configuration-recorders)
assert_contains "DescribeConfigurationRecorders" "$OUT" "default"

# 3. PutDeliveryChannel
OUT=$(aws_config put-delivery-channel \
  --delivery-channel name=default,s3BucketName=my-config-bucket)
assert_contains "PutDeliveryChannel" "$OUT" ""

# 4. DescribeDeliveryChannels
OUT=$(aws_config describe-delivery-channels)
assert_contains "DescribeDeliveryChannels" "$OUT" "my-config-bucket"

# 5. StartConfigurationRecorder
OUT=$(aws_config start-configuration-recorder --configuration-recorder-name default)
assert_contains "StartConfigurationRecorder" "$OUT" ""

# 6. DescribeConfigurationRecorderStatus
OUT=$(aws_config describe-configuration-recorder-status)
assert_contains "DescribeConfigurationRecorderStatus" "$OUT" "true"

# 7. StopConfigurationRecorder
OUT=$(aws_config stop-configuration-recorder --configuration-recorder-name default)
assert_contains "StopConfigurationRecorder" "$OUT" ""

# 8. PutConfigRule
OUT=$(aws_config put-config-rule --config-rule '{"ConfigRuleName":"s3-bucket-public-read-prohibited","Source":{"Owner":"AWS","SourceIdentifier":"S3_BUCKET_PUBLIC_READ_PROHIBITED"}}')
assert_contains "PutConfigRule" "$OUT" ""

# 9. DescribeConfigRules
OUT=$(aws_config describe-config-rules)
assert_contains "DescribeConfigRules" "$OUT" "s3-bucket-public-read-prohibited"

# Extract the ConfigRuleArn for later use in tag operations
RULE_ARN=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['ConfigRules'][0]['ConfigRuleArn'])" 2>/dev/null || echo "")

# 10. PutEvaluations
OUT=$(aws_config put-evaluations \
  --evaluations '[{"ComplianceResourceType":"AWS::S3::Bucket","ComplianceResourceId":"my-bucket","ComplianceType":"NON_COMPLIANT","OrderingTimestamp":"2024-01-01T00:00:00Z"}]' \
  --result-token rule-token-1)
assert_contains "PutEvaluations" "$OUT" "FailedEvaluations"

# 11. GetComplianceDetailsByConfigRule
OUT=$(aws_config get-compliance-details-by-config-rule \
  --config-rule-name s3-bucket-public-read-prohibited)
assert_contains "GetComplianceDetailsByConfigRule" "$OUT" "EvaluationResults"

# 12. DescribeComplianceByConfigRule
OUT=$(aws_config describe-compliance-by-config-rule)
assert_contains "DescribeComplianceByConfigRule" "$OUT" "ComplianceByConfigRules"

# 13. DescribeComplianceByResource
OUT=$(aws_config describe-compliance-by-resource \
  --resource-type AWS::S3::Bucket --resource-id my-bucket)
assert_contains "DescribeComplianceByResource" "$OUT" "ComplianceByResources"

# 14. TagResource
OUT=$(aws_config tag-resource \
  --resource-arn "$RULE_ARN" \
  --tags Key=env,Value=prod)
assert_contains "TagResource" "$OUT" ""

# 15. ListTagsForResource
OUT=$(aws_config list-tags-for-resource --resource-arn "$RULE_ARN")
assert_contains "ListTagsForResource" "$OUT" "env"

# 16. UntagResource
OUT=$(aws_config untag-resource \
  --resource-arn "$RULE_ARN" \
  --tag-keys env)
assert_contains "UntagResource" "$OUT" ""

# 17. DeleteConfigRule
OUT=$(aws_config delete-config-rule --config-rule-name s3-bucket-public-read-prohibited)
assert_contains "DeleteConfigRule" "$OUT" ""

# 18. DescribeConfigRules after delete
OUT=$(aws_config describe-config-rules)
assert_not_contains "DescribeConfigRules after delete" "$OUT" "s3-bucket-public-read-prohibited"

# 19. DeleteDeliveryChannel
OUT=$(aws_config delete-delivery-channel --delivery-channel-name default)
assert_contains "DeleteDeliveryChannel" "$OUT" ""

# 20. DeleteConfigurationRecorder
OUT=$(aws_config delete-configuration-recorder --configuration-recorder-name default)
assert_contains "DeleteConfigurationRecorder" "$OUT" ""

# 21. DescribeConfigurationRecorders after delete
OUT=$(aws_config describe-configuration-recorders)
assert_not_contains "DescribeConfigurationRecorders after delete" "$OUT" "default"

# ── report ───────────────────────────────────────────────────────────────

echo ""
echo "══════════════════════════════════════════════"
echo "  Config Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"
