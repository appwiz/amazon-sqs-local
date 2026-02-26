#!/usr/bin/env bash
#
# Integration tests for AWS AppSync service within aws-inmemory-services.
#
set -uo pipefail

PORT=19600
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

aws_appsync() {
  aws appsync "$@" \
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

echo "Starting server with AppSync on port ${PORT}..."
"$BINARY" \
  --appsync-port "$PORT" \
  --s3-port 19601 --sns-port 19602 --sqs-port 19603 --dynamodb-port 19604 \
  --lambda-port 19605 --firehose-port 19606 --memorydb-port 19607 \
  --cognito-port 19608 --apigateway-port 19609 --kms-port 19610 \
  --secretsmanager-port 19611 --kinesis-port 19612 --eventbridge-port 19613 \
  --stepfunctions-port 19614 --ssm-port 19615 --cloudwatchlogs-port 19616 \
  --ses-port 19617 --servicecatalog-port 19618 --config-port 19619 \
  --efs-port 19620 \
  --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "ERROR: server failed to start"
  exit 1
fi

echo "Running AppSync integration tests..."

# 1. CreateGraphqlApi
OUT=$(aws_appsync create-graphql-api --name TestApi --authentication-type API_KEY --tags Env=Test)
assert_contains "CreateGraphqlApi" "$OUT" "apiId"
assert_contains "CreateGraphqlApi name" "$OUT" "TestApi"
assert_contains "CreateGraphqlApi auth" "$OUT" "API_KEY"
assert_contains "CreateGraphqlApi arn" "$OUT" "arn:aws:appsync"
assert_contains "CreateGraphqlApi uris" "$OUT" "GRAPHQL"
API_ID=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['graphqlApi']['apiId'])")
API_ARN=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['graphqlApi']['arn'])")

# 2. GetGraphqlApi
OUT=$(aws_appsync get-graphql-api --api-id "$API_ID")
assert_contains "GetGraphqlApi" "$OUT" "$API_ID"
assert_contains "GetGraphqlApi name" "$OUT" "TestApi"

# 3. ListGraphqlApis
OUT=$(aws_appsync list-graphql-apis)
assert_contains "ListGraphqlApis" "$OUT" "graphqlApis"
assert_contains "ListGraphqlApis has API" "$OUT" "$API_ID"

# 4. UpdateGraphqlApi
OUT=$(aws_appsync update-graphql-api --api-id "$API_ID" --name UpdatedApi --authentication-type AWS_IAM)
assert_contains "UpdateGraphqlApi name" "$OUT" "UpdatedApi"
assert_contains "UpdateGraphqlApi auth" "$OUT" "AWS_IAM"

# 5. CreateApiKey
OUT=$(aws_appsync create-api-key --api-id "$API_ID")
assert_contains "CreateApiKey" "$OUT" "apiKey"
KEY_ID=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['apiKey']['id'])")

# 6. ListApiKeys
OUT=$(aws_appsync list-api-keys --api-id "$API_ID")
assert_contains "ListApiKeys" "$OUT" "apiKeys"
assert_contains "ListApiKeys has key" "$OUT" "$KEY_ID"

# 7. UpdateApiKey
OUT=$(aws_appsync update-api-key --api-id "$API_ID" --id "$KEY_ID" --description "Updated key" --expires 1999999999)
assert_contains "UpdateApiKey" "$OUT" "Updated key"
assert_contains "UpdateApiKey expires" "$OUT" "1999999999"

# 8. DeleteApiKey
OUT=$(aws_appsync delete-api-key --api-id "$API_ID" --id "$KEY_ID")
assert_not_contains "DeleteApiKey" "$OUT" "Error"

# 9. ListApiKeys after delete
OUT=$(aws_appsync list-api-keys --api-id "$API_ID")
assert_not_contains "ListApiKeys after delete" "$OUT" "$KEY_ID"

# 10. CreateDataSource
OUT=$(aws_appsync create-data-source --api-id "$API_ID" --name MyDynamoDS --type AMAZON_DYNAMODB)
assert_contains "CreateDataSource" "$OUT" "dataSource"
assert_contains "CreateDataSource name" "$OUT" "MyDynamoDS"
assert_contains "CreateDataSource type" "$OUT" "AMAZON_DYNAMODB"
assert_contains "CreateDataSource arn" "$OUT" "arn:aws:appsync"

# 11. GetDataSource
OUT=$(aws_appsync get-data-source --api-id "$API_ID" --name MyDynamoDS)
assert_contains "GetDataSource" "$OUT" "MyDynamoDS"
assert_contains "GetDataSource type" "$OUT" "AMAZON_DYNAMODB"

# 12. ListDataSources
OUT=$(aws_appsync list-data-sources --api-id "$API_ID")
assert_contains "ListDataSources" "$OUT" "dataSources"
assert_contains "ListDataSources has DS" "$OUT" "MyDynamoDS"

# 13. UpdateDataSource
OUT=$(aws_appsync update-data-source --api-id "$API_ID" --name MyDynamoDS --type NONE)
assert_contains "UpdateDataSource" "$OUT" "NONE"

# 14. CreateDataSource duplicate
OUT=$(aws_appsync create-data-source --api-id "$API_ID" --name MyDynamoDS --type NONE)
assert_contains "CreateDataSource duplicate" "$OUT" "ConcurrentModificationException"

# 15. DeleteDataSource
OUT=$(aws_appsync delete-data-source --api-id "$API_ID" --name MyDynamoDS)
assert_not_contains "DeleteDataSource" "$OUT" "Error"

# 16. ListDataSources after delete
OUT=$(aws_appsync list-data-sources --api-id "$API_ID")
assert_not_contains "ListDataSources after delete" "$OUT" "MyDynamoDS"

# 17. StartSchemaCreation
SCHEMA_DEF=$(echo -n "type Query { hello: String }" | base64)
OUT=$(aws_appsync start-schema-creation --api-id "$API_ID" --definition "$SCHEMA_DEF")
assert_contains "StartSchemaCreation" "$OUT" "SUCCESS"

# 18. GetSchemaCreationStatus
OUT=$(aws_appsync get-schema-creation-status --api-id "$API_ID")
assert_contains "GetSchemaCreationStatus" "$OUT" "SUCCESS"

# 19. TagResource
OUT=$(aws_appsync tag-resource --resource-arn "$API_ARN" --tags Project=MyProject)
assert_not_contains "TagResource" "$OUT" "Error"

# 20. ListTagsForResource
OUT=$(aws_appsync list-tags-for-resource --resource-arn "$API_ARN")
assert_contains "ListTagsForResource" "$OUT" "Project"
assert_contains "ListTagsForResource value" "$OUT" "MyProject"

# 21. UntagResource
OUT=$(aws_appsync untag-resource --resource-arn "$API_ARN" --tag-keys Project)
assert_not_contains "UntagResource" "$OUT" "Error"

# 22. ListTagsForResource after untag
OUT=$(aws_appsync list-tags-for-resource --resource-arn "$API_ARN")
assert_not_contains "ListTagsForResource after untag" "$OUT" "Project"

# 23. DeleteGraphqlApi
OUT=$(aws_appsync delete-graphql-api --api-id "$API_ID")
assert_not_contains "DeleteGraphqlApi" "$OUT" "Error"

# 24. ListGraphqlApis after delete
OUT=$(aws_appsync list-graphql-apis)
assert_not_contains "ListGraphqlApis after delete" "$OUT" "$API_ID"

# 25. GetGraphqlApi not found
OUT=$(aws_appsync get-graphql-api --api-id nonexistent12345)
assert_contains "GetGraphqlApi not found" "$OUT" "NotFoundException"

# 26. DeleteDataSource not found
OUT=$(aws_appsync delete-data-source --api-id nonexistent12345 --name fakeDS)
assert_contains "DeleteDataSource not found" "$OUT" "NotFoundException"

# ── report ───────────────────────────────────────────────────────────────

echo ""
echo "══════════════════════════════════════════════"
echo "  AppSync Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"
