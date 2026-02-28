#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port dynamodb)
ENDPOINT="http://localhost:${PORT}"

aws_ddb() {
  aws dynamodb "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

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

report_results "DynamoDB"
exit $?
