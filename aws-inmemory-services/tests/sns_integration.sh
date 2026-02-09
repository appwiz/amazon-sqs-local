#!/usr/bin/env bash
#
# Integration tests for in-memory SNS service using the system awscli.
# Exercises all SNS API operations.
#
set -uo pipefail

PORT=19911
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

# ── helpers ──────────────────────────────────────────────────────────────

aws_sns() {
  aws sns "$@" \
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

assert_exit_zero() {
  local label="$1"
  shift
  if "$@" > /dev/null 2>&1; then
    PASS=$((PASS + 1))
    TESTS+=("PASS  $label")
  else
    FAIL=$((FAIL + 1))
    TESTS+=("FAIL  $label  (non-zero exit)")
    echo "FAIL: $label" >&2
  fi
}

json_field() {
  python3 -c "import sys,json; print(json.load(sys.stdin)$1)" 2>/dev/null
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

echo "Starting server on port ${PORT}..."
"$BINARY" --s3-port 19000 --sns-port "$PORT" --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "FATAL: server failed to start" >&2
  exit 1
fi

echo "Server running (pid $SERVER_PID). Running tests..."
echo

TOPIC_ARN="arn:aws:sns:${REGION}:${ACCOUNT}:test-topic"

# ═════════════════════════════════════════════════════════════════════════
# 1. CreateTopic
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sns create-topic --name test-topic)
assert_contains "CreateTopic: returns TopicArn" "$OUT" "$TOPIC_ARN"

# Idempotent re-create
OUT=$(aws_sns create-topic --name test-topic)
assert_contains "CreateTopic: idempotent" "$OUT" "$TOPIC_ARN"

# ═════════════════════════════════════════════════════════════════════════
# 2. CreateTopic (FIFO)
# ═════════════════════════════════════════════════════════════════════════

FIFO_ARN="arn:aws:sns:${REGION}:${ACCOUNT}:test-fifo.fifo"

OUT=$(aws_sns create-topic --name test-fifo.fifo \
  --attributes FifoTopic=true,ContentBasedDeduplication=true)
assert_contains "CreateTopic FIFO: returns TopicArn" "$OUT" "$FIFO_ARN"

# ═════════════════════════════════════════════════════════════════════════
# 3. ListTopics
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sns list-topics)
assert_contains "ListTopics: has standard topic" "$OUT" "test-topic"
assert_contains "ListTopics: has FIFO topic" "$OUT" "test-fifo.fifo"

# ═════════════════════════════════════════════════════════════════════════
# 4. GetTopicAttributes
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sns get-topic-attributes --topic-arn "$TOPIC_ARN")
assert_contains "GetTopicAttributes: TopicArn" "$OUT" "$TOPIC_ARN"
assert_contains "GetTopicAttributes: Owner" "$OUT" "$ACCOUNT"
assert_contains "GetTopicAttributes: DisplayName" "$OUT" "DisplayName"

# ═════════════════════════════════════════════════════════════════════════
# 5. SetTopicAttributes
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "SetTopicAttributes: DisplayName" \
  aws sns set-topic-attributes \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --topic-arn "$TOPIC_ARN" \
    --attribute-name DisplayName \
    --attribute-value "My Test Topic"

OUT=$(aws_sns get-topic-attributes --topic-arn "$TOPIC_ARN")
assert_contains "SetTopicAttributes: DisplayName updated" "$OUT" "My Test Topic"

# ═════════════════════════════════════════════════════════════════════════
# 6. Subscribe
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sns subscribe --topic-arn "$TOPIC_ARN" \
  --protocol email --notification-endpoint "test@example.com" \
  --return-subscription-arn)
assert_contains "Subscribe email: has SubscriptionArn" "$OUT" "SubscriptionArn"
SUB_ARN=$(echo "$OUT" | json_field '["SubscriptionArn"]')

OUT=$(aws_sns subscribe --topic-arn "$TOPIC_ARN" \
  --protocol sqs --notification-endpoint "arn:aws:sqs:${REGION}:${ACCOUNT}:my-queue" \
  --return-subscription-arn)
assert_contains "Subscribe SQS: has SubscriptionArn" "$OUT" "SubscriptionArn"
SQS_SUB_ARN=$(echo "$OUT" | json_field '["SubscriptionArn"]')

# ═════════════════════════════════════════════════════════════════════════
# 7. ListSubscriptions
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sns list-subscriptions)
assert_contains "ListSubscriptions: has email sub" "$OUT" "email"
assert_contains "ListSubscriptions: has sqs sub" "$OUT" "sqs"

# ═════════════════════════════════════════════════════════════════════════
# 8. ListSubscriptionsByTopic
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sns list-subscriptions-by-topic --topic-arn "$TOPIC_ARN")
assert_contains "ListSubscriptionsByTopic: has email" "$OUT" "email"
assert_contains "ListSubscriptionsByTopic: has sqs" "$OUT" "sqs"

# ═════════════════════════════════════════════════════════════════════════
# 9. GetSubscriptionAttributes
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sns get-subscription-attributes --subscription-arn "$SUB_ARN")
assert_contains "GetSubscriptionAttributes: has SubscriptionArn" "$OUT" "SubscriptionArn"
assert_contains "GetSubscriptionAttributes: has Protocol" "$OUT" "Protocol"
assert_contains "GetSubscriptionAttributes: has Endpoint" "$OUT" "test@example.com"

# ═════════════════════════════════════════════════════════════════════════
# 10. SetSubscriptionAttributes
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "SetSubscriptionAttributes: RawMessageDelivery" \
  aws sns set-subscription-attributes \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --subscription-arn "$SUB_ARN" \
    --attribute-name RawMessageDelivery \
    --attribute-value true

OUT=$(aws_sns get-subscription-attributes --subscription-arn "$SUB_ARN")
assert_contains "SetSubscriptionAttributes: RawMessageDelivery=true" "$OUT" '"RawMessageDelivery": "true"'

# ═════════════════════════════════════════════════════════════════════════
# 11. Publish
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sns publish --topic-arn "$TOPIC_ARN" --message "Hello from SNS!")
assert_contains "Publish: has MessageId" "$OUT" "MessageId"

# Publish to FIFO topic
OUT=$(aws_sns publish --topic-arn "$FIFO_ARN" \
  --message "FIFO message" \
  --message-group-id "group-1")
assert_contains "Publish FIFO: has MessageId" "$OUT" "MessageId"
assert_contains "Publish FIFO: has SequenceNumber" "$OUT" "SequenceNumber"

# ═════════════════════════════════════════════════════════════════════════
# 12. PublishBatch
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sns publish-batch --topic-arn "$TOPIC_ARN" \
  --publish-batch-request-entries '[
    {"Id":"msg1","Message":"Batch message 1"},
    {"Id":"msg2","Message":"Batch message 2"},
    {"Id":"msg3","Message":"Batch message 3"}
  ]')
assert_contains "PublishBatch: has Successful" "$OUT" "Successful"
assert_contains "PublishBatch: entry msg1" "$OUT" '"Id": "msg1"'
assert_contains "PublishBatch: entry msg2" "$OUT" '"Id": "msg2"'
assert_contains "PublishBatch: entry msg3" "$OUT" '"Id": "msg3"'

# ═════════════════════════════════════════════════════════════════════════
# 13. TagResource
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "TagResource: succeeds" \
  aws sns tag-resource \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --resource-arn "$TOPIC_ARN" \
    --tags Key=Environment,Value=test Key=Team,Value=backend

# ═════════════════════════════════════════════════════════════════════════
# 14. ListTagsForResource
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sns list-tags-for-resource --resource-arn "$TOPIC_ARN")
assert_contains "ListTagsForResource: Environment" "$OUT" "Environment"
assert_contains "ListTagsForResource: test" "$OUT" "test"
assert_contains "ListTagsForResource: Team" "$OUT" "Team"

# ═════════════════════════════════════════════════════════════════════════
# 15. UntagResource
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "UntagResource: succeeds" \
  aws sns untag-resource \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --resource-arn "$TOPIC_ARN" \
    --tag-keys Environment

OUT=$(aws_sns list-tags-for-resource --resource-arn "$TOPIC_ARN")
assert_not_contains "UntagResource: Environment removed" "$OUT" "Environment"
assert_contains "UntagResource: Team still present" "$OUT" "Team"

# ═════════════════════════════════════════════════════════════════════════
# 16. Unsubscribe
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "Unsubscribe: email" \
  aws sns unsubscribe \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --subscription-arn "$SUB_ARN"

OUT=$(aws_sns list-subscriptions-by-topic --topic-arn "$TOPIC_ARN")
assert_not_contains "Unsubscribe: email removed" "$OUT" "test@example.com"
assert_contains "Unsubscribe: SQS still present" "$OUT" "sqs"

# ═════════════════════════════════════════════════════════════════════════
# 17. DeleteTopic
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "DeleteTopic: standard" \
  aws sns delete-topic \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --topic-arn "$TOPIC_ARN"

assert_exit_zero "DeleteTopic: FIFO" \
  aws sns delete-topic \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --topic-arn "$FIFO_ARN"

OUT=$(aws_sns list-topics)
assert_not_contains "DeleteTopic: all removed" "$OUT" "test-topic"
assert_not_contains "DeleteTopic: FIFO removed" "$OUT" "test-fifo"

# ═════════════════════════════════════════════════════════════════════════
# Summary
# ═════════════════════════════════════════════════════════════════════════

echo
echo "═══════════════════════════════════════════════════"
echo "  SNS Results: ${PASS} passed, ${FAIL} failed"
echo "═══════════════════════════════════════════════════"
echo
for t in "${TESTS[@]}"; do
  echo "  $t"
done
echo

if [ "$FAIL" -gt 0 ]; then
  exit 1
fi
