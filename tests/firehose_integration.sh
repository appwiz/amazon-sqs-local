#!/usr/bin/env bash
#
# Integration tests for Firehose service within aws-inmemory-services.
#
set -uo pipefail

PORT=14573
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

# ── helpers ──────────────────────────────────────────────────────────────

aws_firehose() {
  aws firehose "$@" \
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

# ── build & start server ─────────────────────────────────────────────────

echo "Building..."
cargo build --quiet 2>&1

lsof -ti:${PORT} | xargs kill 2>/dev/null || true
sleep 0.5

echo "Starting server with Firehose on port ${PORT}..."
"$BINARY" --firehose-port "$PORT" --s3-port 14001 --sns-port 14002 --sqs-port 14003 --dynamodb-port 14004 --lambda-port 14005 --memorydb-port 14006 --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "ERROR: server failed to start"
  exit 1
fi

# ── Tests ────────────────────────────────────────────────────────────────

echo "Running Firehose integration tests..."

# 1. CreateDeliveryStream
OUT=$(aws_firehose create-delivery-stream \
  --delivery-stream-name mystream)
assert_contains "CreateDeliveryStream" "$OUT" "DeliveryStreamARN"
assert_contains "CreateDeliveryStream arn" "$OUT" "mystream"

# 2. DescribeDeliveryStream
OUT=$(aws_firehose describe-delivery-stream \
  --delivery-stream-name mystream)
assert_contains "DescribeDeliveryStream name" "$OUT" "mystream"
assert_contains "DescribeDeliveryStream status" "$OUT" "ACTIVE"
assert_contains "DescribeDeliveryStream type" "$OUT" "DirectPut"

# 3. ListDeliveryStreams
OUT=$(aws_firehose list-delivery-streams)
assert_contains "ListDeliveryStreams" "$OUT" "mystream"

# 4. PutRecord
OUT=$(aws_firehose put-record \
  --delivery-stream-name mystream \
  --record '{"Data":"SGVsbG8gV29ybGQ="}')
assert_contains "PutRecord" "$OUT" "RecordId"

# 5. PutRecordBatch
OUT=$(aws_firehose put-record-batch \
  --delivery-stream-name mystream \
  --records '{"Data":"UmVjb3JkMQ=="}' '{"Data":"UmVjb3JkMg=="}')
assert_contains "PutRecordBatch count" "$OUT" "FailedPutCount"
assert_contains "PutRecordBatch responses" "$OUT" "RequestResponses"

# 6. TagDeliveryStream
OUT=$(aws_firehose tag-delivery-stream \
  --delivery-stream-name mystream \
  --tags Key=env,Value=test Key=team,Value=data)
assert_contains "TagDeliveryStream" "$OUT" ""

# 7. ListTagsForDeliveryStream
OUT=$(aws_firehose list-tags-for-delivery-stream \
  --delivery-stream-name mystream)
assert_contains "ListTagsForDeliveryStream env" "$OUT" "env"
assert_contains "ListTagsForDeliveryStream team" "$OUT" "data"

# 8. UntagDeliveryStream
OUT=$(aws_firehose untag-delivery-stream \
  --delivery-stream-name mystream \
  --tag-keys env)
assert_contains "UntagDeliveryStream" "$OUT" ""

# Verify tag removed
OUT=$(aws_firehose list-tags-for-delivery-stream \
  --delivery-stream-name mystream)
assert_not_contains "UntagDeliveryStream verify" "$OUT" "env"

# 9. CreateDeliveryStream duplicate error
OUT=$(aws_firehose create-delivery-stream \
  --delivery-stream-name mystream)
assert_contains "CreateDeliveryStream duplicate" "$OUT" "ResourceInUseException"

# 10. DescribeDeliveryStream not found
OUT=$(aws_firehose describe-delivery-stream \
  --delivery-stream-name nonexistent)
assert_contains "DescribeDeliveryStream not found" "$OUT" "ResourceNotFoundException"

# 11. DeleteDeliveryStream
OUT=$(aws_firehose delete-delivery-stream \
  --delivery-stream-name mystream)
assert_contains "DeleteDeliveryStream" "$OUT" ""

# 12. DeleteDeliveryStream not found
OUT=$(aws_firehose delete-delivery-stream \
  --delivery-stream-name mystream)
assert_contains "DeleteDeliveryStream not found" "$OUT" "ResourceNotFoundException"

# ── report ───────────────────────────────────────────────────────────────

echo ""
echo "══════════════════════════════════════════════"
echo "  Firehose Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"
