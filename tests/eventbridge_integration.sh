#!/usr/bin/env bash
#
# Integration tests for EventBridge service within aws-inmemory-services.
#
set -uo pipefail

PORT=19195
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

aws_events() {
  aws events "$@" \
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

echo "Starting server with EventBridge on port ${PORT}..."
"$BINARY" \
  --eventbridge-port "$PORT" \
  --s3-port 19001 --sns-port 19002 --sqs-port 19003 --dynamodb-port 19004 \
  --lambda-port 19005 --firehose-port 19006 --memorydb-port 19007 \
  --cognito-port 19008 --apigateway-port 19009 --kms-port 19010 \
  --secretsmanager-port 19011 --kinesis-port 19012 --stepfunctions-port 19013 \
  --ssm-port 19014 --cloudwatchlogs-port 19015 --ses-port 19016 \
  --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "ERROR: server failed to start"
  exit 1
fi

echo "Running EventBridge integration tests..."

# 1. DescribeEventBus (default)
OUT=$(aws_events describe-event-bus)
assert_contains "DescribeEventBus default" "$OUT" "default"
assert_contains "DescribeEventBus default arn" "$OUT" "arn:aws:events"

# 2. CreateEventBus
OUT=$(aws_events create-event-bus --name mybus)
assert_contains "CreateEventBus" "$OUT" "EventBusArn"
assert_contains "CreateEventBus name" "$OUT" "mybus"

# 3. ListEventBuses
OUT=$(aws_events list-event-buses)
assert_contains "ListEventBuses" "$OUT" "EventBuses"
assert_contains "ListEventBuses default" "$OUT" "default"
assert_contains "ListEventBuses mybus" "$OUT" "mybus"

# 4. DescribeEventBus custom
OUT=$(aws_events describe-event-bus --name mybus)
assert_contains "DescribeEventBus custom" "$OUT" "mybus"

# 5. PutEvents (default bus)
OUT=$(aws_events put-events \
  --entries '[{"Source":"my.service","DetailType":"OrderPlaced","Detail":"{\"orderId\":\"123\"}"}]')
assert_contains "PutEvents" "$OUT" "Entries"
assert_contains "PutEvents FailedEntryCount" "$OUT" "FailedEntryCount"
assert_contains "PutEvents EventId" "$OUT" "EventId"

# 6. PutEvents (custom bus)
OUT=$(aws_events put-events \
  --entries '[{"EventBusName":"mybus","Source":"my.service","DetailType":"Test","Detail":"{}"}]')
assert_contains "PutEvents custom bus" "$OUT" "EventId"

# 7. PutRule
OUT=$(aws_events put-rule \
  --name my-rule \
  --event-pattern '{"source":["my.service"]}' \
  --state ENABLED)
assert_contains "PutRule" "$OUT" "RuleArn"

# 8. DescribeRule
OUT=$(aws_events describe-rule --name my-rule)
assert_contains "DescribeRule" "$OUT" "my-rule"
assert_contains "DescribeRule state" "$OUT" "ENABLED"

# 9. ListRules
OUT=$(aws_events list-rules)
assert_contains "ListRules" "$OUT" "my-rule"

# 10. PutTargets
OUT=$(aws_events put-targets \
  --rule my-rule \
  --targets Id=target1,Arn=arn:aws:lambda:us-east-1:000000000000:function:my-func)
assert_contains "PutTargets" "$OUT" "FailedEntryCount"

# 11. ListTargetsByRule
OUT=$(aws_events list-targets-by-rule --rule my-rule)
assert_contains "ListTargetsByRule" "$OUT" "Targets"
assert_contains "ListTargetsByRule target" "$OUT" "target1"

# 12. RemoveTargets
OUT=$(aws_events remove-targets --rule my-rule --ids target1)
assert_contains "RemoveTargets" "$OUT" "FailedEntryCount"

# 13. ListTargetsByRule after removal
OUT=$(aws_events list-targets-by-rule --rule my-rule)
assert_not_contains "ListTargetsByRule after removal" "$OUT" "target1"

# 14. DeleteRule
OUT=$(aws_events delete-rule --name my-rule)
assert_contains "DeleteRule" "$OUT" ""

# 15. ListRules after delete
OUT=$(aws_events list-rules)
assert_not_contains "ListRules after delete" "$OUT" "my-rule"

# 16. DeleteEventBus
OUT=$(aws_events delete-event-bus --name mybus)
assert_contains "DeleteEventBus" "$OUT" ""

# 17. ListEventBuses after delete
OUT=$(aws_events list-event-buses)
assert_not_contains "ListEventBuses after delete" "$OUT" "mybus"

# ── report ───────────────────────────────────────────────────────────────

echo ""
echo "══════════════════════════════════════════════"
echo "  EventBridge Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"
