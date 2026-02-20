#!/usr/bin/env bash
#
# Integration tests for SES v2 service within aws-inmemory-services.
#
set -uo pipefail

PORT=19500
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

aws_ses() {
  aws sesv2 "$@" \
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

echo "Starting server with SES on port ${PORT}..."
"$BINARY" \
  --ses-port "$PORT" \
  --s3-port 19501 --sns-port 19502 --sqs-port 19503 --dynamodb-port 19504 \
  --lambda-port 19505 --firehose-port 19506 --memorydb-port 19507 \
  --cognito-port 19508 --apigateway-port 19509 --kms-port 19510 \
  --secretsmanager-port 19511 --kinesis-port 19512 --eventbridge-port 19513 \
  --ssm-port 19514 --stepfunctions-port 19515 --cloudwatchlogs-port 19516 \
  --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "ERROR: server failed to start"
  exit 1
fi

echo "Running SES v2 integration tests..."

# 1. CreateEmailIdentity (email address)
OUT=$(aws_ses create-email-identity --email-identity sender@example.com)
assert_contains "CreateEmailIdentity email" "$OUT" "IdentityType"
assert_contains "CreateEmailIdentity verified" "$OUT" "VerifiedForSendingStatus"

# 2. CreateEmailIdentity (domain)
OUT=$(aws_ses create-email-identity --email-identity example.com)
assert_contains "CreateEmailIdentity domain" "$OUT" "IdentityType"

# 3. ListEmailIdentities
OUT=$(aws_ses list-email-identities)
assert_contains "ListEmailIdentities" "$OUT" "EmailIdentities"
assert_contains "ListEmailIdentities sender" "$OUT" "sender@example.com"
assert_contains "ListEmailIdentities domain" "$OUT" "example.com"

# 4. GetEmailIdentity
OUT=$(aws_ses get-email-identity --email-identity sender@example.com)
assert_contains "GetEmailIdentity" "$OUT" "sender@example.com"
assert_contains "GetEmailIdentity verified" "$OUT" "VerifiedForSendingStatus"

# 5. SendEmail (simple)
OUT=$(aws_ses send-email \
  --from-email-address sender@example.com \
  --destination '{"ToAddresses":["recipient@example.com"]}' \
  --content '{"Simple":{"Subject":{"Data":"Test Subject"},"Body":{"Text":{"Data":"Hello World"}}}}')
assert_contains "SendEmail" "$OUT" "MessageId"

# 6. SendEmail (with CC and BCC)
OUT=$(aws_ses send-email \
  --from-email-address sender@example.com \
  --destination '{"ToAddresses":["to@example.com"],"CcAddresses":["cc@example.com"]}' \
  --content '{"Simple":{"Subject":{"Data":"CC Test"},"Body":{"Text":{"Data":"CC Body"}}}}')
assert_contains "SendEmail CC" "$OUT" "MessageId"

# 7. CreateEmailIdentity duplicate
OUT=$(aws_ses create-email-identity --email-identity sender@example.com)
assert_contains "CreateEmailIdentity duplicate" "$OUT" "AlreadyExistsException"

# 8. GetEmailIdentity not found
OUT=$(aws_ses get-email-identity --email-identity unknown@nowhere.com)
assert_contains "GetEmailIdentity not found" "$OUT" "NotFoundException"

# 9. DeleteEmailIdentity
OUT=$(aws_ses delete-email-identity --email-identity example.com)
assert_contains "DeleteEmailIdentity" "$OUT" ""

# 10. ListEmailIdentities after delete
OUT=$(aws_ses list-email-identities)
assert_not_contains "ListEmailIdentities after delete" "$OUT" "example.com"
assert_contains "ListEmailIdentities still has sender" "$OUT" "sender@example.com"

# ── report ───────────────────────────────────────────────────────────────

echo ""
echo "══════════════════════════════════════════════"
echo "  SES v2 Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"
