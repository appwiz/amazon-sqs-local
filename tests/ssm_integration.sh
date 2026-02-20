#!/usr/bin/env bash
#
# Integration tests for SSM Parameter Store service within aws-inmemory-services.
#
set -uo pipefail

PORT=19100
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

aws_ssm() {
  aws ssm "$@" \
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

echo "Starting server with SSM on port ${PORT}..."
"$BINARY" \
  --ssm-port "$PORT" \
  --s3-port 19201 --sns-port 19202 --sqs-port 19203 --dynamodb-port 19204 \
  --lambda-port 19205 --firehose-port 19206 --memorydb-port 19207 \
  --cognito-port 19208 --apigateway-port 19209 --kms-port 19210 \
  --secretsmanager-port 19211 --kinesis-port 19212 --eventbridge-port 19213 \
  --stepfunctions-port 19214 --cloudwatchlogs-port 19215 --ses-port 19216 \
  --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "ERROR: server failed to start"
  exit 1
fi

echo "Running SSM Parameter Store integration tests..."

# 1. PutParameter (String)
OUT=$(aws_ssm put-parameter \
  --name /myapp/db/host \
  --value "localhost" \
  --type String)
assert_contains "PutParameter" "$OUT" "Version"
assert_contains "PutParameter tier" "$OUT" "Tier"

# 2. GetParameter
OUT=$(aws_ssm get-parameter --name /myapp/db/host)
assert_contains "GetParameter" "$OUT" "Parameter"
assert_contains "GetParameter value" "$OUT" "localhost"
assert_contains "GetParameter name" "$OUT" "/myapp/db/host"

# 3. PutParameter (SecureString)
OUT=$(aws_ssm put-parameter \
  --name /myapp/db/password \
  --value "s3cr3t" \
  --type SecureString)
assert_contains "PutParameter SecureString" "$OUT" "Version"

# 4. PutParameter (StringList)
OUT=$(aws_ssm put-parameter \
  --name /myapp/allowed-hosts \
  --value "host1,host2,host3" \
  --type StringList)
assert_contains "PutParameter StringList" "$OUT" "Version"

# 5. GetParameters (multiple)
OUT=$(aws_ssm get-parameters \
  --names /myapp/db/host /myapp/db/password /myapp/does-not-exist)
assert_contains "GetParameters" "$OUT" "Parameters"
assert_contains "GetParameters valid" "$OUT" "localhost"
assert_contains "GetParameters invalid" "$OUT" "InvalidParameters"
assert_contains "GetParameters missing" "$OUT" "does-not-exist"

# 6. GetParametersByPath (recursive)
OUT=$(aws_ssm get-parameters-by-path \
  --path /myapp \
  --recursive)
assert_contains "GetParametersByPath" "$OUT" "Parameters"
assert_contains "GetParametersByPath host" "$OUT" "localhost"
assert_contains "GetParametersByPath password" "$OUT" "s3cr3t"

# 7. GetParametersByPath (non-recursive)
OUT=$(aws_ssm get-parameters-by-path \
  --path /myapp/db)
assert_contains "GetParametersByPath non-recursive" "$OUT" "localhost"

# 8. Overwrite parameter
OUT=$(aws_ssm put-parameter \
  --name /myapp/db/host \
  --value "prod-server.example.com" \
  --type String \
  --overwrite)
assert_contains "PutParameter overwrite" "$OUT" "Version"

# 9. Verify overwrite
OUT=$(aws_ssm get-parameter --name /myapp/db/host)
assert_contains "GetParameter after overwrite" "$OUT" "prod-server.example.com"
assert_not_contains "GetParameter after overwrite old" "$OUT" "localhost"

# 10. DescribeParameters
OUT=$(aws_ssm describe-parameters)
assert_contains "DescribeParameters" "$OUT" "Parameters"
assert_contains "DescribeParameters name" "$OUT" "/myapp/db/host"

# 11. DeleteParameter
OUT=$(aws_ssm delete-parameter --name /myapp/allowed-hosts)
assert_contains "DeleteParameter" "$OUT" ""

# 12. GetParameter after delete
OUT=$(aws_ssm get-parameter --name /myapp/allowed-hosts)
assert_contains "GetParameter after delete" "$OUT" "ParameterNotFound"

# 13. DeleteParameters (batch)
OUT=$(aws_ssm delete-parameters \
  --names /myapp/db/host /myapp/db/password /nonexistent)
assert_contains "DeleteParameters" "$OUT" "DeletedParameters"
assert_contains "DeleteParameters invalid" "$OUT" "InvalidParameters"
assert_contains "DeleteParameters nonexistent" "$OUT" "nonexistent"

# 14. PutParameter no overwrite conflict
OUT=$(aws_ssm put-parameter \
  --name /myapp/db/host \
  --value "value1" \
  --type String)
assert_contains "PutParameter first" "$OUT" "Version"

OUT=$(aws_ssm put-parameter \
  --name /myapp/db/host \
  --value "value2" \
  --type String)
assert_contains "PutParameter no-overwrite conflict" "$OUT" "ParameterAlreadyExists"

# ── report ───────────────────────────────────────────────────────────────

echo ""
echo "══════════════════════════════════════════════"
echo "  SSM Parameter Store Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"
