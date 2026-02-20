#!/usr/bin/env bash
#
# Integration tests for Kinesis Data Streams service within aws-inmemory-services.
#
set -uo pipefail

PORT=14568
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

aws_kinesis() {
  aws kinesis "$@" \
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

echo "Starting server with Kinesis on port ${PORT}..."
"$BINARY" \
  --kinesis-port "$PORT" \
  --s3-port 14601 --sns-port 14602 --sqs-port 14603 --dynamodb-port 14604 \
  --lambda-port 14605 --firehose-port 14606 --memorydb-port 14607 \
  --cognito-port 14608 --apigateway-port 14609 --kms-port 14610 \
  --secretsmanager-port 14611 --eventbridge-port 14612 --stepfunctions-port 14613 \
  --ssm-port 14614 --cloudwatchlogs-port 14615 --ses-port 14616 \
  --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "ERROR: server failed to start"
  exit 1
fi

echo "Running Kinesis integration tests..."

# 1. CreateStream
OUT=$(aws_kinesis create-stream --stream-name mystream --shard-count 1)
assert_contains "CreateStream" "$OUT" ""

# 2. DescribeStream
OUT=$(aws_kinesis describe-stream --stream-name mystream)
assert_contains "DescribeStream name" "$OUT" "mystream"
assert_contains "DescribeStream status" "$OUT" "ACTIVE"
assert_contains "DescribeStream shards" "$OUT" "Shards"

# 3. DescribeStreamSummary
OUT=$(aws_kinesis describe-stream-summary --stream-name mystream)
assert_contains "DescribeStreamSummary" "$OUT" "mystream"
assert_contains "DescribeStreamSummary status" "$OUT" "ACTIVE"

# 4. ListStreams
OUT=$(aws_kinesis list-streams)
assert_contains "ListStreams" "$OUT" "mystream"

# 5. ListShards
OUT=$(aws_kinesis list-shards --stream-name mystream)
assert_contains "ListShards" "$OUT" "Shards"
assert_contains "ListShards shard id" "$OUT" "shardId"

# 6. PutRecord
DATA_B64=$(echo -n "Hello Kinesis!" | base64)
OUT=$(aws_kinesis put-record \
  --stream-name mystream \
  --data "$DATA_B64" \
  --partition-key "pk1")
assert_contains "PutRecord ShardId" "$OUT" "ShardId"
assert_contains "PutRecord SequenceNumber" "$OUT" "SequenceNumber"

# 7. PutRecords
OUT=$(aws_kinesis put-records \
  --stream-name mystream \
  --records Data=$(echo -n "Record1" | base64),PartitionKey=pk1 Data=$(echo -n "Record2" | base64),PartitionKey=pk2)
assert_contains "PutRecords" "$OUT" "Records"
assert_contains "PutRecords failed count" "$OUT" "FailedRecordCount"

# 8. GetShardIterator - TRIM_HORIZON
OUT=$(aws_kinesis get-shard-iterator \
  --stream-name mystream \
  --shard-id shardId-000000000000 \
  --shard-iterator-type TRIM_HORIZON)
assert_contains "GetShardIterator" "$OUT" "ShardIterator"
ITERATOR=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['ShardIterator'])" 2>/dev/null || echo "")

# 9. GetRecords
OUT=$(aws_kinesis get-records --shard-iterator "$ITERATOR")
assert_contains "GetRecords" "$OUT" "Records"
assert_contains "GetRecords millis" "$OUT" "MillisBehindLatest"
assert_contains "GetRecords next iterator" "$OUT" "NextShardIterator"

# 10. AddTagsToStream
OUT=$(aws_kinesis add-tags-to-stream --stream-name mystream --tags env=test,team=data)
assert_contains "AddTagsToStream" "$OUT" ""

# 11. ListTagsForStream
OUT=$(aws_kinesis list-tags-for-stream --stream-name mystream)
assert_contains "ListTagsForStream env" "$OUT" "env"
assert_contains "ListTagsForStream data" "$OUT" "data"

# 12. RemoveTagsFromStream
OUT=$(aws_kinesis remove-tags-from-stream --stream-name mystream --tag-keys env)
assert_contains "RemoveTagsFromStream" "$OUT" ""

# 13. Verify tag removed
OUT=$(aws_kinesis list-tags-for-stream --stream-name mystream)
assert_not_contains "RemoveTagsFromStream verify" "$OUT" '"env"'

# 14. IncreaseStreamRetentionPeriod
OUT=$(aws_kinesis increase-stream-retention-period \
  --stream-name mystream \
  --retention-period-hours 48)
assert_contains "IncreaseRetentionPeriod" "$OUT" ""

# 15. DecreaseStreamRetentionPeriod
OUT=$(aws_kinesis decrease-stream-retention-period \
  --stream-name mystream \
  --retention-period-hours 24)
assert_contains "DecreaseRetentionPeriod" "$OUT" ""

# 16. CreateStream duplicate
OUT=$(aws_kinesis create-stream --stream-name mystream --shard-count 1)
assert_contains "CreateStream duplicate" "$OUT" "ResourceInUseException"

# 17. DeleteStream
OUT=$(aws_kinesis delete-stream --stream-name mystream)
assert_contains "DeleteStream" "$OUT" ""

# 18. DescribeStream not found
OUT=$(aws_kinesis describe-stream --stream-name mystream)
assert_contains "DescribeStream not found" "$OUT" "ResourceNotFoundException"

# ── report ───────────────────────────────────────────────────────────────

echo ""
echo "══════════════════════════════════════════════"
echo "  Kinesis Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"
