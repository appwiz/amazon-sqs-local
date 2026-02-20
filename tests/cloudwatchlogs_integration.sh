#!/usr/bin/env bash
#
# Integration tests for CloudWatch Logs service within aws-inmemory-services.
#
set -uo pipefail

PORT=19400
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

aws_logs() {
  aws logs "$@" \
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

echo "Starting server with CloudWatch Logs on port ${PORT}..."
"$BINARY" \
  --cloudwatchlogs-port "$PORT" \
  --s3-port 19401 --sns-port 19402 --sqs-port 19403 --dynamodb-port 19404 \
  --lambda-port 19405 --firehose-port 19406 --memorydb-port 19407 \
  --cognito-port 19408 --apigateway-port 19409 --kms-port 19410 \
  --secretsmanager-port 19411 --kinesis-port 19412 --eventbridge-port 19413 \
  --ssm-port 19414 --stepfunctions-port 19415 --ses-port 19416 \
  --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "ERROR: server failed to start"
  exit 1
fi

echo "Running CloudWatch Logs integration tests..."

# 1. CreateLogGroup
OUT=$(aws_logs create-log-group --log-group-name /myapp/service)
assert_contains "CreateLogGroup" "$OUT" ""

# 2. DescribeLogGroups
OUT=$(aws_logs describe-log-groups)
assert_contains "DescribeLogGroups" "$OUT" "logGroups"
assert_contains "DescribeLogGroups name" "$OUT" "/myapp/service"

# 3. DescribeLogGroups with prefix
OUT=$(aws_logs describe-log-groups --log-group-name-prefix /myapp)
assert_contains "DescribeLogGroups prefix" "$OUT" "/myapp/service"

# 4. PutRetentionPolicy
OUT=$(aws_logs put-retention-policy \
  --log-group-name /myapp/service \
  --retention-in-days 7)
assert_contains "PutRetentionPolicy" "$OUT" ""

# 5. DescribeLogGroups shows retention
OUT=$(aws_logs describe-log-groups --log-group-name-prefix /myapp)
assert_contains "DescribeLogGroups retention" "$OUT" "retentionInDays"

# 6. CreateLogStream
OUT=$(aws_logs create-log-stream \
  --log-group-name /myapp/service \
  --log-stream-name stream-2024-01-01)
assert_contains "CreateLogStream" "$OUT" ""

# 7. CreateLogStream second
OUT=$(aws_logs create-log-stream \
  --log-group-name /myapp/service \
  --log-stream-name stream-2024-01-02)
assert_contains "CreateLogStream second" "$OUT" ""

# 8. DescribeLogStreams
OUT=$(aws_logs describe-log-streams \
  --log-group-name /myapp/service)
assert_contains "DescribeLogStreams" "$OUT" "logStreams"
assert_contains "DescribeLogStreams name" "$OUT" "stream-2024-01-01"

# 9. PutLogEvents
NOW_MS=$(date +%s)000
OUT=$(aws_logs put-log-events \
  --log-group-name /myapp/service \
  --log-stream-name stream-2024-01-01 \
  --log-events "[{\"timestamp\":${NOW_MS},\"message\":\"Application started\"},{\"timestamp\":${NOW_MS},\"message\":\"Processing request id=42\"}]")
assert_contains "PutLogEvents" "$OUT" "nextSequenceToken"

# 10. GetLogEvents
OUT=$(aws_logs get-log-events \
  --log-group-name /myapp/service \
  --log-stream-name stream-2024-01-01)
assert_contains "GetLogEvents" "$OUT" "events"
assert_contains "GetLogEvents message" "$OUT" "Application started"
assert_contains "GetLogEvents request" "$OUT" "Processing request"

# 11. PutLogEvents to second stream
OUT=$(aws_logs put-log-events \
  --log-group-name /myapp/service \
  --log-stream-name stream-2024-01-02 \
  --log-events "[{\"timestamp\":${NOW_MS},\"message\":\"Error occurred in batch job\"},{\"timestamp\":${NOW_MS},\"message\":\"Batch job completed\"}]")
assert_contains "PutLogEvents second stream" "$OUT" "nextSequenceToken"

# 12. FilterLogEvents - match all
OUT=$(aws_logs filter-log-events \
  --log-group-name /myapp/service)
assert_contains "FilterLogEvents all" "$OUT" "events"
assert_contains "FilterLogEvents all has messages" "$OUT" "Application started"

# 13. FilterLogEvents - with filter pattern
OUT=$(aws_logs filter-log-events \
  --log-group-name /myapp/service \
  --filter-pattern "Error")
assert_contains "FilterLogEvents pattern" "$OUT" "events"
assert_contains "FilterLogEvents pattern match" "$OUT" "Error occurred"

# 14. TagLogGroup
OUT=$(aws_logs tag-log-group \
  --log-group-name /myapp/service \
  --tags env=prod,team=platform)
assert_contains "TagLogGroup" "$OUT" ""

# 15. ListTagsLogGroup
OUT=$(aws_logs list-tags-log-group \
  --log-group-name /myapp/service)
assert_contains "ListTagsLogGroup" "$OUT" "tags"
assert_contains "ListTagsLogGroup env" "$OUT" "prod"

# 16. UntagLogGroup
OUT=$(aws_logs untag-log-group \
  --log-group-name /myapp/service \
  --tags env)
assert_contains "UntagLogGroup" "$OUT" ""

# 17. Verify tag removed
OUT=$(aws_logs list-tags-log-group \
  --log-group-name /myapp/service)
assert_not_contains "UntagLogGroup verify" "$OUT" '"env"'

# 18. DeleteRetentionPolicy
OUT=$(aws_logs delete-retention-policy \
  --log-group-name /myapp/service)
assert_contains "DeleteRetentionPolicy" "$OUT" ""

# 19. DeleteLogStream
OUT=$(aws_logs delete-log-stream \
  --log-group-name /myapp/service \
  --log-stream-name stream-2024-01-02)
assert_contains "DeleteLogStream" "$OUT" ""

# 20. DescribeLogStreams after delete
OUT=$(aws_logs describe-log-streams \
  --log-group-name /myapp/service)
assert_not_contains "DescribeLogStreams after delete" "$OUT" "stream-2024-01-02"

# 21. CreateLogGroup duplicate
OUT=$(aws_logs create-log-group --log-group-name /myapp/service)
assert_contains "CreateLogGroup duplicate" "$OUT" "ResourceAlreadyExistsException"

# 22. DeleteLogGroup
OUT=$(aws_logs delete-log-group --log-group-name /myapp/service)
assert_contains "DeleteLogGroup" "$OUT" ""

# 23. DescribeLogGroups after delete
OUT=$(aws_logs describe-log-groups)
assert_not_contains "DescribeLogGroups after delete" "$OUT" "/myapp/service"

# ── report ───────────────────────────────────────────────────────────────

echo ""
echo "══════════════════════════════════════════════"
echo "  CloudWatch Logs Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"
