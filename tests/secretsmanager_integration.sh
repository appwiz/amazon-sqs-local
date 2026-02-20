#!/usr/bin/env bash
#
# Integration tests for Secrets Manager service within aws-inmemory-services.
#
set -uo pipefail

PORT=17700
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

aws_sm() {
  aws secretsmanager "$@" \
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

echo "Starting server with Secrets Manager on port ${PORT}..."
"$BINARY" \
  --secretsmanager-port "$PORT" \
  --s3-port 17101 --sns-port 17102 --sqs-port 17103 --dynamodb-port 17104 \
  --lambda-port 17105 --firehose-port 17106 --memorydb-port 17107 \
  --cognito-port 17108 --apigateway-port 17109 --kms-port 17110 \
  --kinesis-port 17111 --eventbridge-port 17112 --stepfunctions-port 17113 \
  --ssm-port 17114 --cloudwatchlogs-port 17115 --ses-port 17116 \
  --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "ERROR: server failed to start"
  exit 1
fi

echo "Running Secrets Manager integration tests..."

# 1. CreateSecret
OUT=$(aws_sm create-secret \
  --name mydb/credentials \
  --secret-string '{"username":"admin","password":"s3cr3t"}' \
  --description "DB credentials")
assert_contains "CreateSecret" "$OUT" "ARN"
assert_contains "CreateSecret name" "$OUT" "mydb/credentials"
assert_contains "CreateSecret version" "$OUT" "VersionId"

# 2. GetSecretValue
OUT=$(aws_sm get-secret-value --secret-id mydb/credentials)
assert_contains "GetSecretValue" "$OUT" "SecretString"
assert_contains "GetSecretValue content" "$OUT" "admin"
assert_contains "GetSecretValue AWSCURRENT" "$OUT" "AWSCURRENT"

# 3. DescribeSecret
OUT=$(aws_sm describe-secret --secret-id mydb/credentials)
assert_contains "DescribeSecret ARN" "$OUT" "ARN"
assert_contains "DescribeSecret name" "$OUT" "mydb/credentials"
assert_contains "DescribeSecret desc" "$OUT" "DB credentials"

# 4. ListSecrets
OUT=$(aws_sm list-secrets)
assert_contains "ListSecrets" "$OUT" "mydb/credentials"
assert_contains "ListSecrets list" "$OUT" "SecretList"

# 5. PutSecretValue
OUT=$(aws_sm put-secret-value \
  --secret-id mydb/credentials \
  --secret-string '{"username":"admin","password":"newpass"}')
assert_contains "PutSecretValue" "$OUT" "VersionId"

# 6. GetSecretValue after update
OUT=$(aws_sm get-secret-value --secret-id mydb/credentials)
assert_contains "GetSecretValue updated" "$OUT" "newpass"

# 7. UpdateSecret description
OUT=$(aws_sm update-secret --secret-id mydb/credentials --description "Updated DB creds")
assert_contains "UpdateSecret" "$OUT" "ARN"

# 8. TagResource
OUT=$(aws_sm tag-resource --secret-id mydb/credentials \
  --tags Key=env,Value=prod Key=team,Value=platform)
assert_contains "TagResource" "$OUT" ""

# 9. Verify tags via DescribeSecret
OUT=$(aws_sm describe-secret --secret-id mydb/credentials)
assert_contains "TagResource verify" "$OUT" "prod"

# 10. UntagResource
OUT=$(aws_sm untag-resource --secret-id mydb/credentials --tag-keys env)
assert_contains "UntagResource" "$OUT" ""

# 11. ListSecretVersionIds
OUT=$(aws_sm list-secret-version-ids --secret-id mydb/credentials)
assert_contains "ListSecretVersionIds" "$OUT" "Versions"
assert_contains "ListSecretVersionIds ARN" "$OUT" "ARN"

# 12. CreateSecret duplicate
OUT=$(aws_sm create-secret --name mydb/credentials --secret-string "dup")
assert_contains "CreateSecret duplicate" "$OUT" "ResourceExistsException"

# 13. GetSecretValue not found
OUT=$(aws_sm get-secret-value --secret-id nonexistent-secret)
assert_contains "GetSecretValue not found" "$OUT" "ResourceNotFoundException"

# 14. Create another secret
OUT=$(aws_sm create-secret --name api/key --secret-string "api-key-12345")
assert_contains "CreateSecret api/key" "$OUT" "ARN"

# 15. ListSecrets shows both
OUT=$(aws_sm list-secrets)
assert_contains "ListSecrets both" "$OUT" "api/key"

# 16. DeleteSecret
OUT=$(aws_sm delete-secret --secret-id api/key --force-delete-without-recovery)
assert_contains "DeleteSecret" "$OUT" "DeletionDate"

# 17. GetSecretValue after delete
OUT=$(aws_sm get-secret-value --secret-id api/key)
assert_contains "GetSecretValue deleted" "$OUT" "ResourceNotFoundException"

# ── report ───────────────────────────────────────────────────────────────

echo ""
echo "══════════════════════════════════════════════"
echo "  Secrets Manager Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"
