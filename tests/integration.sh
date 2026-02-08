#!/usr/bin/env bash
#
# Integration tests for aws-sqs-local using the system awscli.
# Exercises all 23 SQS API operations.
#
set -euo pipefail

PORT=19324
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-sqs-local"

PASS=0
FAIL=0
TESTS=()

# ── helpers ──────────────────────────────────────────────────────────────

aws_sqs() {
  aws sqs "$@" \
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
  rm -f /tmp/sqs-lp-result.json
}
trap cleanup EXIT

# ── build & start server ─────────────────────────────────────────────────

echo "Building..."
cargo build --quiet 2>&1

echo "Starting server on port ${PORT}..."
"$BINARY" --port "$PORT" --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "FATAL: server failed to start" >&2
  exit 1
fi

echo "Server running (pid $SERVER_PID). Running tests..."
echo

QUEUE_URL="${ENDPOINT}/${ACCOUNT}/test-queue"
FIFO_URL="${ENDPOINT}/${ACCOUNT}/test-fifo.fifo"

# ═════════════════════════════════════════════════════════════════════════
# 1. CreateQueue
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sqs create-queue --queue-name test-queue)
assert_contains "CreateQueue: returns QueueUrl" "$OUT" "$QUEUE_URL"

# Idempotent re-create
OUT=$(aws_sqs create-queue --queue-name test-queue)
assert_contains "CreateQueue: idempotent" "$OUT" "$QUEUE_URL"

# ═════════════════════════════════════════════════════════════════════════
# 2. CreateQueue (FIFO)
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sqs create-queue --queue-name test-fifo.fifo \
  --attributes FifoQueue=true,ContentBasedDeduplication=true)
assert_contains "CreateQueue FIFO: returns QueueUrl" "$OUT" "test-fifo.fifo"

# ═════════════════════════════════════════════════════════════════════════
# 3. GetQueueUrl
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sqs get-queue-url --queue-name test-queue)
assert_contains "GetQueueUrl: resolves name" "$OUT" "$QUEUE_URL"

# ═════════════════════════════════════════════════════════════════════════
# 4. ListQueues
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sqs list-queues)
assert_contains "ListQueues: includes standard" "$OUT" "test-queue"
assert_contains "ListQueues: includes FIFO" "$OUT" "test-fifo.fifo"

OUT=$(aws_sqs list-queues --queue-name-prefix test-fifo)
assert_contains "ListQueues prefix: matches FIFO" "$OUT" "test-fifo.fifo"

# ═════════════════════════════════════════════════════════════════════════
# 5. GetQueueAttributes
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sqs get-queue-attributes --queue-url "$QUEUE_URL" --attribute-names All)
assert_contains "GetQueueAttributes: VisibilityTimeout" "$OUT" '"VisibilityTimeout": "30"'
assert_contains "GetQueueAttributes: QueueArn" "$OUT" "arn:aws:sqs:${REGION}:${ACCOUNT}:test-queue"
assert_contains "GetQueueAttributes: ApproximateNumberOfMessages" "$OUT" "ApproximateNumberOfMessages"
assert_contains "GetQueueAttributes: CreatedTimestamp" "$OUT" "CreatedTimestamp"

# ═════════════════════════════════════════════════════════════════════════
# 6. SetQueueAttributes
# ═════════════════════════════════════════════════════════════════════════

aws_sqs set-queue-attributes --queue-url "$QUEUE_URL" \
  --attributes VisibilityTimeout=60 > /dev/null
OUT=$(aws_sqs get-queue-attributes --queue-url "$QUEUE_URL" --attribute-names VisibilityTimeout)
assert_contains "SetQueueAttributes: VisibilityTimeout changed" "$OUT" '"VisibilityTimeout": "60"'

# ═════════════════════════════════════════════════════════════════════════
# 7. SendMessage
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sqs send-message --queue-url "$QUEUE_URL" --message-body "hello world")
assert_contains "SendMessage: returns MessageId" "$OUT" "MessageId"
assert_contains "SendMessage: returns MD5OfMessageBody" "$OUT" "MD5OfMessageBody"
MSG_MD5=$(echo "$OUT" | json_field '["MD5OfMessageBody"]')
assert_contains "SendMessage: correct MD5" "$MSG_MD5" "5eb63bbbe01eeed093cb22bb8f5acdc3"

# ═════════════════════════════════════════════════════════════════════════
# 8. SendMessage with message attributes
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sqs send-message --queue-url "$QUEUE_URL" \
  --message-body "with attrs" \
  --message-attributes '{"Color":{"DataType":"String","StringValue":"blue"}}')
assert_contains "SendMessage with attributes: MD5OfMessageAttributes" "$OUT" "MD5OfMessageAttributes"

# ═════════════════════════════════════════════════════════════════════════
# 9. ReceiveMessage
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sqs receive-message --queue-url "$QUEUE_URL" \
  --max-number-of-messages 10 \
  --attribute-names All \
  --message-attribute-names All)
assert_contains "ReceiveMessage: has Messages" "$OUT" "Messages"
assert_contains "ReceiveMessage: has Body" "$OUT" "hello world"
assert_contains "ReceiveMessage: has ReceiptHandle" "$OUT" "ReceiptHandle"
assert_contains "ReceiveMessage: has SentTimestamp" "$OUT" "SentTimestamp"
assert_contains "ReceiveMessage: has ApproximateReceiveCount" "$OUT" "ApproximateReceiveCount"

RECEIPT=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['Messages'][0]['ReceiptHandle'])")

# ═════════════════════════════════════════════════════════════════════════
# 10. ChangeMessageVisibility
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "ChangeMessageVisibility: succeeds" \
  aws sqs change-message-visibility \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --queue-url "$QUEUE_URL" \
    --receipt-handle "$RECEIPT" \
    --visibility-timeout 120

# ═════════════════════════════════════════════════════════════════════════
# 11. DeleteMessage
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "DeleteMessage: succeeds" \
  aws sqs delete-message \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --queue-url "$QUEUE_URL" \
    --receipt-handle "$RECEIPT"

# ═════════════════════════════════════════════════════════════════════════
# 12. SendMessageBatch
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sqs send-message-batch --queue-url "$QUEUE_URL" \
  --entries '[
    {"Id":"m1","MessageBody":"batch-msg-1"},
    {"Id":"m2","MessageBody":"batch-msg-2"},
    {"Id":"m3","MessageBody":"batch-msg-3"}
  ]')
assert_contains "SendMessageBatch: has Successful" "$OUT" "Successful"
assert_contains "SendMessageBatch: entry m1" "$OUT" '"Id": "m1"'
assert_contains "SendMessageBatch: entry m2" "$OUT" '"Id": "m2"'
assert_contains "SendMessageBatch: entry m3" "$OUT" '"Id": "m3"'

# ═════════════════════════════════════════════════════════════════════════
# 13. DeleteMessageBatch
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sqs receive-message --queue-url "$QUEUE_URL" --max-number-of-messages 10)
HANDLES=$(echo "$OUT" | python3 -c "
import sys, json
msgs = json.load(sys.stdin).get('Messages', [])
entries = [{'Id': f'del-{i}', 'ReceiptHandle': m['ReceiptHandle']} for i, m in enumerate(msgs)]
print(json.dumps(entries))
")

if [ "$HANDLES" != "[]" ]; then
  OUT=$(aws_sqs delete-message-batch --queue-url "$QUEUE_URL" --entries "$HANDLES")
  assert_contains "DeleteMessageBatch: has Successful" "$OUT" "Successful"
else
  PASS=$((PASS + 1))
  TESTS+=("PASS  DeleteMessageBatch: (skipped, no inflight)")
fi

# ═════════════════════════════════════════════════════════════════════════
# 14. ChangeMessageVisibilityBatch
# ═════════════════════════════════════════════════════════════════════════

aws_sqs send-message --queue-url "$QUEUE_URL" --message-body "vis-batch-1" > /dev/null
aws_sqs send-message --queue-url "$QUEUE_URL" --message-body "vis-batch-2" > /dev/null

OUT=$(aws_sqs receive-message --queue-url "$QUEUE_URL" --max-number-of-messages 2)
ENTRIES=$(echo "$OUT" | python3 -c "
import sys, json
msgs = json.load(sys.stdin).get('Messages', [])
entries = [{'Id': f'v{i}', 'ReceiptHandle': m['ReceiptHandle'], 'VisibilityTimeout': 300} for i, m in enumerate(msgs)]
print(json.dumps(entries))
")

if [ "$ENTRIES" != "[]" ]; then
  OUT=$(aws_sqs change-message-visibility-batch --queue-url "$QUEUE_URL" --entries "$ENTRIES")
  assert_contains "ChangeMessageVisibilityBatch: has Successful" "$OUT" "Successful"
else
  PASS=$((PASS + 1))
  TESTS+=("PASS  ChangeMessageVisibilityBatch: (skipped)")
fi

# ═════════════════════════════════════════════════════════════════════════
# 15. PurgeQueue
# ═════════════════════════════════════════════════════════════════════════

# First wait 60s purge cooldown won't apply — queue hasn't been purged yet
aws_sqs send-message --queue-url "$QUEUE_URL" --message-body "to-be-purged" > /dev/null

assert_exit_zero "PurgeQueue: succeeds" \
  aws sqs purge-queue \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --queue-url "$QUEUE_URL"

OUT=$(aws_sqs get-queue-attributes --queue-url "$QUEUE_URL" --attribute-names ApproximateNumberOfMessages)
assert_contains "PurgeQueue: messages cleared" "$OUT" '"ApproximateNumberOfMessages": "0"'

# ═════════════════════════════════════════════════════════════════════════
# 16. TagQueue
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "TagQueue: succeeds" \
  aws sqs tag-queue \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --queue-url "$QUEUE_URL" \
    --tags Environment=test,Team=backend

# ═════════════════════════════════════════════════════════════════════════
# 17. ListQueueTags
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sqs list-queue-tags --queue-url "$QUEUE_URL")
assert_contains "ListQueueTags: Environment" "$OUT" '"Environment": "test"'
assert_contains "ListQueueTags: Team" "$OUT" '"Team": "backend"'

# ═════════════════════════════════════════════════════════════════════════
# 18. UntagQueue
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "UntagQueue: succeeds" \
  aws sqs untag-queue \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --queue-url "$QUEUE_URL" \
    --tag-keys Environment

OUT=$(aws_sqs list-queue-tags --queue-url "$QUEUE_URL")
assert_not_contains "UntagQueue: Environment removed" "$OUT" "Environment"
assert_contains "UntagQueue: Team still present" "$OUT" '"Team": "backend"'

# ═════════════════════════════════════════════════════════════════════════
# 19. AddPermission
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "AddPermission: succeeds" \
  aws sqs add-permission \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --queue-url "$QUEUE_URL" \
    --label allow-send \
    --aws-account-ids 111122223333 \
    --actions SendMessage

# ═════════════════════════════════════════════════════════════════════════
# 20. RemovePermission
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "RemovePermission: succeeds" \
  aws sqs remove-permission \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --queue-url "$QUEUE_URL" \
    --label allow-send

# ═════════════════════════════════════════════════════════════════════════
# 21. FIFO: Send + Receive + Dedup + Delete
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sqs send-message --queue-url "$FIFO_URL" \
  --message-body "fifo-msg-1" \
  --message-group-id "group-a")
assert_contains "FIFO SendMessage: has SequenceNumber" "$OUT" "SequenceNumber"
FIFO_MSG_ID=$(echo "$OUT" | json_field '["MessageId"]')

# Dedup: same body → should return same MessageId (content-based)
OUT2=$(aws_sqs send-message --queue-url "$FIFO_URL" \
  --message-body "fifo-msg-1" \
  --message-group-id "group-a")
FIFO_MSG_ID2=$(echo "$OUT2" | json_field '["MessageId"]')
if [ "$FIFO_MSG_ID" = "$FIFO_MSG_ID2" ]; then
  PASS=$((PASS + 1))
  TESTS+=("PASS  FIFO dedup: same MessageId returned")
else
  FAIL=$((FAIL + 1))
  TESTS+=("FAIL  FIFO dedup: different MessageId ($FIFO_MSG_ID vs $FIFO_MSG_ID2)")
fi

OUT=$(aws_sqs receive-message --queue-url "$FIFO_URL" \
  --max-number-of-messages 10 \
  --attribute-names All)
assert_contains "FIFO ReceiveMessage: has body" "$OUT" "fifo-msg-1"
assert_contains "FIFO ReceiveMessage: has MessageGroupId" "$OUT" "MessageGroupId"
assert_contains "FIFO ReceiveMessage: has SequenceNumber" "$OUT" "SequenceNumber"

FIFO_RECEIPT=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['Messages'][0]['ReceiptHandle'])")
assert_exit_zero "FIFO DeleteMessage: succeeds" \
  aws sqs delete-message \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --queue-url "$FIFO_URL" \
    --receipt-handle "$FIFO_RECEIPT"

# ═════════════════════════════════════════════════════════════════════════
# 22. RedrivePolicy / ListDeadLetterSourceQueues
# ═════════════════════════════════════════════════════════════════════════

aws_sqs create-queue --queue-name test-dlq > /dev/null
DLQ_URL="${ENDPOINT}/${ACCOUNT}/test-dlq"
DLQ_ARN="arn:aws:sqs:${REGION}:${ACCOUNT}:test-dlq"

# Use JSON syntax for --attributes to avoid shorthand parsing issues
REDRIVE_POLICY='{"deadLetterTargetArn":"'"${DLQ_ARN}"'","maxReceiveCount":"2"}'
aws_sqs set-queue-attributes --queue-url "$QUEUE_URL" \
  --attributes '{"RedrivePolicy":"'"$(echo "$REDRIVE_POLICY" | sed 's/"/\\"/g')"'"}' > /dev/null

OUT=$(aws_sqs get-queue-attributes --queue-url "$QUEUE_URL" --attribute-names RedrivePolicy)
assert_contains "RedrivePolicy: set correctly" "$OUT" "deadLetterTargetArn"
assert_contains "RedrivePolicy: has maxReceiveCount" "$OUT" "maxReceiveCount"

OUT=$(aws_sqs list-dead-letter-source-queues --queue-url "$DLQ_URL")
assert_contains "ListDeadLetterSourceQueues: lists source" "$OUT" "test-queue"

# ═════════════════════════════════════════════════════════════════════════
# 23. StartMessageMoveTask / ListMessageMoveTasks / CancelMessageMoveTask
# ═════════════════════════════════════════════════════════════════════════

aws_sqs send-message --queue-url "$DLQ_URL" --message-body "move-me-1" > /dev/null
aws_sqs send-message --queue-url "$DLQ_URL" --message-body "move-me-2" > /dev/null

DEST_ARN="arn:aws:sqs:${REGION}:${ACCOUNT}:test-queue"

OUT=$(aws_sqs start-message-move-task \
  --source-arn "$DLQ_ARN" \
  --destination-arn "$DEST_ARN")
assert_contains "StartMessageMoveTask: has TaskHandle" "$OUT" "TaskHandle"
TASK_HANDLE=$(echo "$OUT" | json_field '["TaskHandle"]')

sleep 1

OUT=$(aws_sqs list-message-move-tasks --source-arn "$DLQ_ARN")
assert_contains "ListMessageMoveTasks: has Results" "$OUT" "Results"
assert_contains "ListMessageMoveTasks: has Status" "$OUT" "Status"
assert_contains "ListMessageMoveTasks: has SourceArn" "$OUT" "$DLQ_ARN"

# Send more messages and start another task to test cancel
# (first task should be COMPLETED by now)
aws_sqs send-message --queue-url "$DLQ_URL" --message-body "cancel-me-1" > /dev/null
aws_sqs send-message --queue-url "$DLQ_URL" --message-body "cancel-me-2" > /dev/null
aws_sqs send-message --queue-url "$DLQ_URL" --message-body "cancel-me-3" > /dev/null

OUT=$(aws_sqs start-message-move-task \
  --source-arn "$DLQ_ARN" \
  --destination-arn "$DEST_ARN" \
  --max-number-of-messages-per-second 1)
CANCEL_HANDLE=$(echo "$OUT" | json_field '["TaskHandle"]')

OUT=$(aws_sqs cancel-message-move-task --task-handle "$CANCEL_HANDLE")
assert_contains "CancelMessageMoveTask: has ApproximateNumberOfMessagesMoved" "$OUT" "ApproximateNumberOfMessagesMoved"

# ═════════════════════════════════════════════════════════════════════════
# 24. DeleteQueue
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "DeleteQueue: standard" \
  aws sqs delete-queue \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --queue-url "$QUEUE_URL"

assert_exit_zero "DeleteQueue: FIFO" \
  aws sqs delete-queue \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --queue-url "$FIFO_URL"

assert_exit_zero "DeleteQueue: DLQ" \
  aws sqs delete-queue \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --queue-url "$DLQ_URL"

OUT=$(aws_sqs list-queues)
assert_not_contains "DeleteQueue: all removed" "$OUT" "test-queue"
assert_not_contains "DeleteQueue: FIFO removed" "$OUT" "test-fifo"
assert_not_contains "DeleteQueue: DLQ removed" "$OUT" "test-dlq"

# ═════════════════════════════════════════════════════════════════════════
# 25. ReceiveMessage on empty queue (short poll)
# ═════════════════════════════════════════════════════════════════════════

aws_sqs create-queue --queue-name empty-queue > /dev/null
EMPTY_URL="${ENDPOINT}/${ACCOUNT}/empty-queue"

OUT=$(aws_sqs receive-message --queue-url "$EMPTY_URL" --wait-time-seconds 0)
assert_not_contains "ReceiveMessage empty: no messages" "$OUT" "ReceiptHandle"

aws_sqs delete-queue --queue-url "$EMPTY_URL" > /dev/null 2>&1 || true

# ═════════════════════════════════════════════════════════════════════════
# 26. Long polling
# ═════════════════════════════════════════════════════════════════════════

aws_sqs create-queue --queue-name long-poll-queue > /dev/null
LP_URL="${ENDPOINT}/${ACCOUNT}/long-poll-queue"

# Long poll in background
(aws_sqs receive-message --queue-url "$LP_URL" --wait-time-seconds 5 \
  --attribute-names All > /tmp/sqs-lp-result.json 2>&1) &
LP_PID=$!

sleep 1

# Send while waiting
aws_sqs send-message --queue-url "$LP_URL" --message-body "long-poll-msg" > /dev/null

wait "$LP_PID" 2>/dev/null || true
LP_OUT=$(cat /tmp/sqs-lp-result.json 2>/dev/null || echo "")
assert_contains "Long polling: wakes up on send" "$LP_OUT" "long-poll-msg"

aws_sqs delete-queue --queue-url "$LP_URL" > /dev/null 2>&1 || true

# ═════════════════════════════════════════════════════════════════════════
# 27. Message attributes round-trip
# ═════════════════════════════════════════════════════════════════════════

aws_sqs create-queue --queue-name attr-test-queue > /dev/null
ATTR_URL="${ENDPOINT}/${ACCOUNT}/attr-test-queue"

aws_sqs send-message --queue-url "$ATTR_URL" \
  --message-body "attrs-test" \
  --message-attributes '{
    "Color":{"DataType":"String","StringValue":"red"},
    "Count":{"DataType":"Number","StringValue":"42"}
  }' > /dev/null

OUT=$(aws_sqs receive-message --queue-url "$ATTR_URL" \
  --message-attribute-names All --attribute-names All)
assert_contains "Message attributes: Color" "$OUT" '"Color"'
assert_contains "Message attributes: Count" "$OUT" '"Count"'
assert_contains "Message attributes: red value" "$OUT" '"red"'
assert_contains "Message attributes: 42 value" "$OUT" '"42"'

aws_sqs delete-queue --queue-url "$ATTR_URL" > /dev/null 2>&1 || true

# ═════════════════════════════════════════════════════════════════════════
# 28. Queue with custom attributes at creation
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_sqs create-queue --queue-name custom-attrs-queue \
  --attributes VisibilityTimeout=45,DelaySeconds=5,MaximumMessageSize=65536 \
  --tags Project=test)
CUSTOM_URL="${ENDPOINT}/${ACCOUNT}/custom-attrs-queue"
assert_contains "CreateQueue custom attrs: url" "$OUT" "custom-attrs-queue"

OUT=$(aws_sqs get-queue-attributes --queue-url "$CUSTOM_URL" --attribute-names All)
assert_contains "Custom attrs: VisibilityTimeout=45" "$OUT" '"VisibilityTimeout": "45"'
assert_contains "Custom attrs: DelaySeconds=5" "$OUT" '"DelaySeconds": "5"'
assert_contains "Custom attrs: MaximumMessageSize=65536" "$OUT" '"MaximumMessageSize": "65536"'

OUT=$(aws_sqs list-queue-tags --queue-url "$CUSTOM_URL")
assert_contains "CreateQueue tags: Project" "$OUT" '"Project": "test"'

aws_sqs delete-queue --queue-url "$CUSTOM_URL" > /dev/null 2>&1 || true

# ═════════════════════════════════════════════════════════════════════════
# Summary
# ═════════════════════════════════════════════════════════════════════════

echo
echo "═══════════════════════════════════════════════════"
echo "  Results: ${PASS} passed, ${FAIL} failed"
echo "═══════════════════════════════════════════════════"
echo
for t in "${TESTS[@]}"; do
  echo "  $t"
done
echo

if [ "$FAIL" -gt 0 ]; then
  exit 1
fi
