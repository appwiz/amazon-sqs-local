#!/usr/bin/env bash
#
# Integration tests for API Gateway service within aws-inmemory-services.
#
set -uo pipefail

PORT=14567
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

# ── helpers ──────────────────────────────────────────────────────────────

aws_apigw() {
  aws apigateway "$@" \
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

echo "Starting server with API Gateway on port ${PORT}..."
"$BINARY" --apigateway-port "$PORT" --s3-port 14101 --sns-port 14102 --sqs-port 14103 \
  --dynamodb-port 14104 --lambda-port 14105 --firehose-port 14106 --memorydb-port 14107 \
  --cognito-port 14108 --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "ERROR: server failed to start"
  exit 1
fi

# ── Tests ────────────────────────────────────────────────────────────────

echo "Running API Gateway integration tests..."

# 1. CreateRestApi
OUT=$(aws_apigw create-rest-api --name MyTestApi --description "Test API")
assert_contains "CreateRestApi name" "$OUT" "MyTestApi"
assert_contains "CreateRestApi description" "$OUT" "Test API"
assert_contains "CreateRestApi id" "$OUT" "id"
API_ID=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['id'])" 2>/dev/null)

# 2. GetRestApi
OUT=$(aws_apigw get-rest-api --rest-api-id "$API_ID")
assert_contains "GetRestApi name" "$OUT" "MyTestApi"
assert_contains "GetRestApi id" "$OUT" "$API_ID"

# 3. GetRestApis
OUT=$(aws_apigw get-rest-apis)
assert_contains "GetRestApis" "$OUT" "MyTestApi"

# 4. Create a second API
OUT2=$(aws_apigw create-rest-api --name SecondApi)
assert_contains "CreateRestApi second" "$OUT2" "SecondApi"

OUT=$(aws_apigw get-rest-apis)
assert_contains "GetRestApis both" "$OUT" "MyTestApi"
assert_contains "GetRestApis second" "$OUT" "SecondApi"

# 5. GetResources (root resource exists by default)
OUT=$(aws_apigw get-resources --rest-api-id "$API_ID")
assert_contains "GetResources root path" "$OUT" '"/"'
ROOT_ID=$(echo "$OUT" | python3 -c "import sys,json; items=json.load(sys.stdin)['items']; print([r for r in items if r['path']=='/'][0]['id'])" 2>/dev/null)

# 6. CreateResource
OUT=$(aws_apigw create-resource \
  --rest-api-id "$API_ID" \
  --parent-id "$ROOT_ID" \
  --path-part "users")
assert_contains "CreateResource pathPart" "$OUT" "users"
assert_contains "CreateResource path" "$OUT" '"/users"'
RESOURCE_ID=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['id'])" 2>/dev/null)

# 7. GetResource
OUT=$(aws_apigw get-resource \
  --rest-api-id "$API_ID" \
  --resource-id "$RESOURCE_ID")
assert_contains "GetResource path" "$OUT" '"/users"'

# 8. GetResources after creation
OUT=$(aws_apigw get-resources --rest-api-id "$API_ID")
assert_contains "GetResources users" "$OUT" "users"

# 9. PutMethod
OUT=$(aws_apigw put-method \
  --rest-api-id "$API_ID" \
  --resource-id "$RESOURCE_ID" \
  --http-method GET \
  --authorization-type NONE)
assert_contains "PutMethod httpMethod" "$OUT" "GET"
assert_contains "PutMethod authType" "$OUT" "NONE"

# 10. GetMethod
OUT=$(aws_apigw get-method \
  --rest-api-id "$API_ID" \
  --resource-id "$RESOURCE_ID" \
  --http-method GET)
assert_contains "GetMethod httpMethod" "$OUT" "GET"

# 11. PutIntegration
OUT=$(aws_apigw put-integration \
  --rest-api-id "$API_ID" \
  --resource-id "$RESOURCE_ID" \
  --http-method GET \
  --type MOCK \
  --request-templates '{"application/json": "{\"statusCode\": 200}"}')
assert_contains "PutIntegration type" "$OUT" "MOCK"

# 12. GetIntegration
OUT=$(aws_apigw get-integration \
  --rest-api-id "$API_ID" \
  --resource-id "$RESOURCE_ID" \
  --http-method GET)
assert_contains "GetIntegration type" "$OUT" "MOCK"

# 13. PutMethodResponse
OUT=$(aws_apigw put-method-response \
  --rest-api-id "$API_ID" \
  --resource-id "$RESOURCE_ID" \
  --http-method GET \
  --status-code 200 \
  --response-models '{"application/json": "Empty"}')
assert_contains "PutMethodResponse statusCode" "$OUT" "200"

# 14. GetMethodResponse
OUT=$(aws_apigw get-method-response \
  --rest-api-id "$API_ID" \
  --resource-id "$RESOURCE_ID" \
  --http-method GET \
  --status-code 200)
assert_contains "GetMethodResponse statusCode" "$OUT" "200"

# 15. PutIntegrationResponse
OUT=$(aws_apigw put-integration-response \
  --rest-api-id "$API_ID" \
  --resource-id "$RESOURCE_ID" \
  --http-method GET \
  --status-code 200 \
  --response-templates '{"application/json": ""}')
assert_contains "PutIntegrationResponse statusCode" "$OUT" "200"

# 16. CreateDeployment
OUT=$(aws_apigw create-deployment \
  --rest-api-id "$API_ID" \
  --stage-name prod \
  --description "Production deployment")
assert_contains "CreateDeployment id" "$OUT" "id"
assert_contains "CreateDeployment description" "$OUT" "Production deployment"
DEPLOYMENT_ID=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['id'])" 2>/dev/null)

# 17. GetDeployment
OUT=$(aws_apigw get-deployment \
  --rest-api-id "$API_ID" \
  --deployment-id "$DEPLOYMENT_ID")
assert_contains "GetDeployment id" "$OUT" "$DEPLOYMENT_ID"

# 18. GetDeployments
OUT=$(aws_apigw get-deployments --rest-api-id "$API_ID")
assert_contains "GetDeployments" "$OUT" "$DEPLOYMENT_ID"

# 19. GetStage (auto-created by deployment with stage-name)
OUT=$(aws_apigw get-stage \
  --rest-api-id "$API_ID" \
  --stage-name prod)
assert_contains "GetStage name" "$OUT" "prod"
assert_contains "GetStage deploymentId" "$OUT" "$DEPLOYMENT_ID"

# 20. GetStages
OUT=$(aws_apigw get-stages --rest-api-id "$API_ID")
assert_contains "GetStages" "$OUT" "prod"

# 21. CreateStage (explicit)
OUT=$(aws_apigw create-stage \
  --rest-api-id "$API_ID" \
  --stage-name staging \
  --deployment-id "$DEPLOYMENT_ID" \
  --description "Staging environment")
assert_contains "CreateStage name" "$OUT" "staging"
assert_contains "CreateStage description" "$OUT" "Staging environment"

# 22. UpdateStage
OUT=$(aws_apigw update-stage \
  --rest-api-id "$API_ID" \
  --stage-name staging \
  --patch-operations op=replace,path=/description,value="Updated staging")
assert_contains "UpdateStage description" "$OUT" "Updated staging"

# 23. Error cases
OUT=$(aws_apigw get-rest-api --rest-api-id "nonexistent")
assert_contains "GetRestApi not found" "$OUT" "NotFoundException"

OUT=$(aws_apigw get-resource \
  --rest-api-id "$API_ID" \
  --resource-id "nonexistent")
assert_contains "GetResource not found" "$OUT" "NotFoundException"

OUT=$(aws_apigw get-stage \
  --rest-api-id "$API_ID" \
  --stage-name nonexistent)
assert_contains "GetStage not found" "$OUT" "NotFoundException"

# 24. DeleteMethod
OUT=$(aws_apigw delete-method \
  --rest-api-id "$API_ID" \
  --resource-id "$RESOURCE_ID" \
  --http-method GET)
assert_contains "DeleteMethod" "$OUT" ""

# 25. DeleteResource
OUT=$(aws_apigw delete-resource \
  --rest-api-id "$API_ID" \
  --resource-id "$RESOURCE_ID")
assert_contains "DeleteResource" "$OUT" ""

OUT=$(aws_apigw get-resources --rest-api-id "$API_ID")
assert_not_contains "DeleteResource verify" "$OUT" '"users"'

# 26. DeleteStage
OUT=$(aws_apigw delete-stage \
  --rest-api-id "$API_ID" \
  --stage-name staging)
assert_contains "DeleteStage" "$OUT" ""

# 27. DeleteRestApi
OUT=$(aws_apigw delete-rest-api --rest-api-id "$API_ID")
assert_contains "DeleteRestApi" "$OUT" ""

OUT=$(aws_apigw get-rest-api --rest-api-id "$API_ID")
assert_contains "DeleteRestApi verify" "$OUT" "NotFoundException"

# ── report ───────────────────────────────────────────────────────────────

echo ""
echo "══════════════════════════════════════════════"
echo "  API Gateway Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"
