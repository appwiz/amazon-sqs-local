#!/usr/bin/env bash
#
# Integration tests for KMS service within aws-inmemory-services.
#
set -uo pipefail

PORT=17600
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

# ── helpers ──────────────────────────────────────────────────────────────

aws_kms() {
  aws kms "$@" \
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

echo "Starting server with KMS on port ${PORT}..."
"$BINARY" \
  --kms-port "$PORT" \
  --s3-port 17001 --sns-port 17002 --sqs-port 17003 --dynamodb-port 17004 \
  --lambda-port 17005 --firehose-port 17006 --memorydb-port 17007 \
  --cognito-port 17008 --apigateway-port 17009 --secretsmanager-port 17010 \
  --kinesis-port 17011 --eventbridge-port 17012 --stepfunctions-port 17013 \
  --ssm-port 17014 --cloudwatchlogs-port 17015 --ses-port 17016 \
  --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "ERROR: server failed to start"
  exit 1
fi

# ── Tests ────────────────────────────────────────────────────────────────

echo "Running KMS integration tests..."

# 1. CreateKey
OUT=$(aws_kms create-key --description "test key")
assert_contains "CreateKey" "$OUT" "KeyId"
assert_contains "CreateKey ARN" "$OUT" "arn:aws:kms"
assert_contains "CreateKey enabled" "$OUT" "Enabled"
KEY_ID=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['KeyMetadata']['KeyId'])" 2>/dev/null || echo "")

# 2. DescribeKey
OUT=$(aws_kms describe-key --key-id "$KEY_ID")
assert_contains "DescribeKey" "$OUT" "KeyId"
assert_contains "DescribeKey ARN" "$OUT" "arn:aws:kms"

# 3. ListKeys
OUT=$(aws_kms list-keys)
assert_contains "ListKeys" "$OUT" "Keys"
assert_contains "ListKeys has key" "$OUT" "$KEY_ID"

# 4. CreateAlias
OUT=$(aws_kms create-alias --alias-name alias/mykey --target-key-id "$KEY_ID")
assert_contains "CreateAlias" "$OUT" ""

# 5. ListAliases
OUT=$(aws_kms list-aliases)
assert_contains "ListAliases" "$OUT" "alias/mykey"

# 6. Encrypt
PLAINTEXT_B64=$(echo -n "Hello, KMS!" | base64)
OUT=$(aws_kms encrypt --key-id "$KEY_ID" --plaintext "$PLAINTEXT_B64")
assert_contains "Encrypt" "$OUT" "CiphertextBlob"
assert_contains "Encrypt KeyId" "$OUT" "arn:aws:kms"
CIPHERTEXT=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['CiphertextBlob'])" 2>/dev/null || echo "")

# 7. Decrypt
OUT=$(aws_kms decrypt --ciphertext-blob "$CIPHERTEXT")
assert_contains "Decrypt" "$OUT" "Plaintext"
assert_contains "Decrypt KeyId" "$OUT" "arn:aws:kms"

# 8. GenerateDataKey
OUT=$(aws_kms generate-data-key --key-id "$KEY_ID" --key-spec AES_256)
assert_contains "GenerateDataKey" "$OUT" "Plaintext"
assert_contains "GenerateDataKey ciphertext" "$OUT" "CiphertextBlob"

# 9. GenerateDataKeyWithoutPlaintext
OUT=$(aws_kms generate-data-key-without-plaintext --key-id "$KEY_ID" --key-spec AES_256)
assert_contains "GenerateDataKeyWithoutPlaintext" "$OUT" "CiphertextBlob"
assert_not_contains "GenerateDataKeyWithoutPlaintext no plaintext" "$OUT" "Plaintext"

# 10. GenerateRandom
OUT=$(aws_kms generate-random --number-of-bytes 32)
assert_contains "GenerateRandom" "$OUT" "Plaintext"

# 11. TagResource
OUT=$(aws_kms tag-resource --key-id "$KEY_ID" --tags TagKey=env,TagValue=test TagKey=team,TagValue=platform)
assert_contains "TagResource" "$OUT" ""

# 12. ListResourceTags
OUT=$(aws_kms list-resource-tags --key-id "$KEY_ID")
assert_contains "ListResourceTags env" "$OUT" "env"
assert_contains "ListResourceTags platform" "$OUT" "platform"

# 13. UntagResource
OUT=$(aws_kms untag-resource --key-id "$KEY_ID" --tag-keys env)
assert_contains "UntagResource" "$OUT" ""

# Verify tag removed
OUT=$(aws_kms list-resource-tags --key-id "$KEY_ID")
assert_not_contains "UntagResource verify" "$OUT" '"env"'

# 14. GetKeyPolicy
OUT=$(aws_kms get-key-policy --key-id "$KEY_ID" --policy-name default)
assert_contains "GetKeyPolicy" "$OUT" "Policy"

# 15. DisableKey
OUT=$(aws_kms disable-key --key-id "$KEY_ID")
assert_contains "DisableKey" "$OUT" ""

# 16. EnableKey
OUT=$(aws_kms enable-key --key-id "$KEY_ID")
assert_contains "EnableKey" "$OUT" ""

# 17. ScheduleKeyDeletion
OUT=$(aws_kms schedule-key-deletion --key-id "$KEY_ID" --pending-window-in-days 7)
assert_contains "ScheduleKeyDeletion" "$OUT" "DeletionDate"
assert_contains "ScheduleKeyDeletion state" "$OUT" "PendingDeletion"

# 18. CancelKeyDeletion
OUT=$(aws_kms cancel-key-deletion --key-id "$KEY_ID")
assert_contains "CancelKeyDeletion" "$OUT" "KeyId"

# 19. DeleteAlias
OUT=$(aws_kms delete-alias --alias-name alias/mykey)
assert_contains "DeleteAlias" "$OUT" ""

# 20. DescribeKey not found
OUT=$(aws_kms describe-key --key-id "ffffffff-ffff-ffff-ffff-ffffffffffff")
assert_contains "DescribeKey not found" "$OUT" "NotFoundException"

# ── report ───────────────────────────────────────────────────────────────

echo ""
echo "══════════════════════════════════════════════"
echo "  KMS Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"
