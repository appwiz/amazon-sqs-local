#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port config)
ENDPOINT="http://localhost:${PORT}"

aws_config() {
  aws configservice "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

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

report_results "CONFIG"
exit $?
