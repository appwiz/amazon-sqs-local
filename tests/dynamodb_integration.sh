#!/usr/bin/env bash
#
# Integration tests for DynamoDB service within aws-inmemory-services.
#
set -uo pipefail

PORT=18000
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

# ── helpers ──────────────────────────────────────────────────────────────

aws_ddb() {
  aws dynamodb "$@" \
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

echo "Starting server with DynamoDB on port ${PORT}..."
"$BINARY" --dynamodb-port "$PORT" --s3-port 18001 --sns-port 18002 --sqs-port 18003 --lambda-port 18004 --firehose-port 18005 --memorydb-port 18006 --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "ERROR: server failed to start"
  exit 1
fi

# ── Tests ────────────────────────────────────────────────────────────────

echo "Running DynamoDB integration tests..."

# 1. CreateTable
OUT=$(aws_ddb create-table \
  --table-name TestTable \
  --attribute-definitions AttributeName=pk,AttributeType=S \
  --key-schema AttributeName=pk,KeyType=HASH \
  --billing-mode PAY_PER_REQUEST)
assert_contains "CreateTable" "$OUT" "TestTable"
assert_contains "CreateTable status" "$OUT" "ACTIVE"

# 2. CreateTable with sort key
OUT=$(aws_ddb create-table \
  --table-name CompositeTable \
  --attribute-definitions AttributeName=pk,AttributeType=S AttributeName=sk,AttributeType=S \
  --key-schema AttributeName=pk,KeyType=HASH AttributeName=sk,KeyType=RANGE \
  --billing-mode PAY_PER_REQUEST)
assert_contains "CreateTable with sort key" "$OUT" "CompositeTable"

# 3. ListTables
OUT=$(aws_ddb list-tables)
assert_contains "ListTables TestTable" "$OUT" "TestTable"
assert_contains "ListTables CompositeTable" "$OUT" "CompositeTable"

# 4. DescribeTable
OUT=$(aws_ddb describe-table --table-name TestTable)
assert_contains "DescribeTable" "$OUT" "TestTable"
assert_contains "DescribeTable PAY_PER_REQUEST" "$OUT" "PAY_PER_REQUEST"

# 5. PutItem
OUT=$(aws_ddb put-item \
  --table-name TestTable \
  --item '{"pk":{"S":"key1"},"data":{"S":"value1"},"num":{"N":"42"}}')
assert_contains "PutItem" "$OUT" ""

# 6. GetItem
OUT=$(aws_ddb get-item \
  --table-name TestTable \
  --key '{"pk":{"S":"key1"}}')
assert_contains "GetItem data" "$OUT" "value1"
assert_contains "GetItem num" "$OUT" "42"

# 7. PutItem (overwrite)
OUT=$(aws_ddb put-item \
  --table-name TestTable \
  --item '{"pk":{"S":"key1"},"data":{"S":"updated_value"}}')
OUT=$(aws_ddb get-item \
  --table-name TestTable \
  --key '{"pk":{"S":"key1"}}')
assert_contains "PutItem overwrite" "$OUT" "updated_value"

# 8. PutItem second item
aws_ddb put-item \
  --table-name TestTable \
  --item '{"pk":{"S":"key2"},"data":{"S":"value2"}}' > /dev/null

# 9. Scan
OUT=$(aws_ddb scan --table-name TestTable)
assert_contains "Scan key1" "$OUT" "key1"
assert_contains "Scan key2" "$OUT" "key2"

# 10. Query
OUT=$(aws_ddb query \
  --table-name TestTable \
  --key-condition-expression "pk = :pk" \
  --expression-attribute-values '{":pk":{"S":"key1"}}')
assert_contains "Query" "$OUT" "key1"
assert_not_contains "Query excludes key2" "$OUT" "key2"

# 11. UpdateItem
OUT=$(aws_ddb update-item \
  --table-name TestTable \
  --key '{"pk":{"S":"key1"}}' \
  --update-expression "SET #d = :d" \
  --expression-attribute-names '{"#d":"data"}' \
  --expression-attribute-values '{":d":{"S":"new_value"}}' \
  --return-values ALL_NEW)
assert_contains "UpdateItem" "$OUT" "new_value"

# 12. DeleteItem
OUT=$(aws_ddb delete-item \
  --table-name TestTable \
  --key '{"pk":{"S":"key2"}}' \
  --return-values ALL_OLD)
assert_contains "DeleteItem" "$OUT" "value2"

# Verify deleted
OUT=$(aws_ddb get-item \
  --table-name TestTable \
  --key '{"pk":{"S":"key2"}}')
assert_not_contains "DeleteItem verify" "$OUT" "key2"

# 13. BatchWriteItem
OUT=$(aws_ddb batch-write-item \
  --request-items '{"TestTable":[{"PutRequest":{"Item":{"pk":{"S":"batch1"},"data":{"S":"b1"}}}},{"PutRequest":{"Item":{"pk":{"S":"batch2"},"data":{"S":"b2"}}}}]}')
assert_contains "BatchWriteItem" "$OUT" "UnprocessedItems"

# 14. BatchGetItem
OUT=$(aws_ddb batch-get-item \
  --request-items '{"TestTable":{"Keys":[{"pk":{"S":"batch1"}},{"pk":{"S":"batch2"}}]}}')
assert_contains "BatchGetItem b1" "$OUT" "b1"
assert_contains "BatchGetItem b2" "$OUT" "b2"

# 15. PutItem to composite table
aws_ddb put-item \
  --table-name CompositeTable \
  --item '{"pk":{"S":"user1"},"sk":{"S":"order#001"},"total":{"N":"100"}}' > /dev/null
aws_ddb put-item \
  --table-name CompositeTable \
  --item '{"pk":{"S":"user1"},"sk":{"S":"order#002"},"total":{"N":"200"}}' > /dev/null
aws_ddb put-item \
  --table-name CompositeTable \
  --item '{"pk":{"S":"user2"},"sk":{"S":"order#001"},"total":{"N":"50"}}' > /dev/null

# 16. Query composite table
OUT=$(aws_ddb query \
  --table-name CompositeTable \
  --key-condition-expression "pk = :pk" \
  --expression-attribute-values '{":pk":{"S":"user1"}}')
assert_contains "Query composite user1" "$OUT" "order#001"
assert_contains "Query composite count" "$OUT" '"Count": 2'

# 17. TagResource
OUT=$(aws_ddb tag-resource \
  --resource-arn "arn:aws:dynamodb:${REGION}:${ACCOUNT}:table/TestTable" \
  --tags Key=env,Value=test)
assert_contains "TagResource" "$OUT" ""

# 18. ListTagsOfResource
OUT=$(aws_ddb list-tags-of-resource \
  --resource-arn "arn:aws:dynamodb:${REGION}:${ACCOUNT}:table/TestTable")
assert_contains "ListTagsOfResource" "$OUT" "env"

# 19. UntagResource
OUT=$(aws_ddb untag-resource \
  --resource-arn "arn:aws:dynamodb:${REGION}:${ACCOUNT}:table/TestTable" \
  --tag-keys env)
assert_contains "UntagResource" "$OUT" ""

# 20. CreateTable duplicate error
OUT=$(aws_ddb create-table \
  --table-name TestTable \
  --attribute-definitions AttributeName=pk,AttributeType=S \
  --key-schema AttributeName=pk,KeyType=HASH \
  --billing-mode PAY_PER_REQUEST)
assert_contains "CreateTable duplicate error" "$OUT" "ResourceInUseException"

# 21. DescribeTable not found
OUT=$(aws_ddb describe-table --table-name NonExistent)
assert_contains "DescribeTable not found" "$OUT" "ResourceNotFoundException"

# 22. DeleteTable
OUT=$(aws_ddb delete-table --table-name CompositeTable)
assert_contains "DeleteTable" "$OUT" "DELETING"

# 23. UpdateTable
OUT=$(aws_ddb update-table \
  --table-name TestTable \
  --billing-mode PROVISIONED \
  --provisioned-throughput ReadCapacityUnits=5,WriteCapacityUnits=5)
assert_contains "UpdateTable" "$OUT" "TestTable"

# Final cleanup
aws_ddb delete-table --table-name TestTable > /dev/null 2>&1

# ── report ───────────────────────────────────────────────────────────────

echo ""
echo "══════════════════════════════════════════════"
echo "  DynamoDB Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"
