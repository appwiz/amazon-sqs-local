#!/usr/bin/env bash
#
# Integration tests for Step Functions service within aws-inmemory-services.
#
set -uo pipefail

PORT=18083
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

aws_sfn() {
  aws stepfunctions "$@" \
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

echo "Starting server with Step Functions on port ${PORT}..."
"$BINARY" \
  --stepfunctions-port "$PORT" \
  --s3-port 18001 --sns-port 18002 --sqs-port 18003 --dynamodb-port 18004 \
  --lambda-port 18005 --firehose-port 18006 --memorydb-port 18007 \
  --cognito-port 18008 --apigateway-port 18009 --kms-port 18010 \
  --secretsmanager-port 18011 --kinesis-port 18012 --eventbridge-port 18013 \
  --ssm-port 18014 --cloudwatchlogs-port 18015 --ses-port 18016 \
  --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "ERROR: server failed to start"
  exit 1
fi

echo "Running Step Functions integration tests..."

DEFINITION='{"Comment":"Test SM","StartAt":"Pass","States":{"Pass":{"Type":"Pass","End":true}}}'
ROLE_ARN="arn:aws:iam::000000000000:role/StepFunctionsRole"

# 1. CreateStateMachine
OUT=$(aws_sfn create-state-machine \
  --name my-state-machine \
  --definition "$DEFINITION" \
  --role-arn "$ROLE_ARN")
assert_contains "CreateStateMachine" "$OUT" "stateMachineArn"
assert_contains "CreateStateMachine ARN" "$OUT" "arn:aws:states"
SM_ARN=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['stateMachineArn'])" 2>/dev/null || echo "")

# 2. DescribeStateMachine
OUT=$(aws_sfn describe-state-machine --state-machine-arn "$SM_ARN")
assert_contains "DescribeStateMachine" "$OUT" "my-state-machine"
assert_contains "DescribeStateMachine def" "$OUT" "Pass"
assert_contains "DescribeStateMachine status" "$OUT" "ACTIVE"

# 3. ListStateMachines
OUT=$(aws_sfn list-state-machines)
assert_contains "ListStateMachines" "$OUT" "my-state-machine"

# 4. StartExecution
OUT=$(aws_sfn start-execution \
  --state-machine-arn "$SM_ARN" \
  --name my-execution \
  --input '{"key":"value"}')
assert_contains "StartExecution" "$OUT" "executionArn"
assert_contains "StartExecution date" "$OUT" "startDate"
EXEC_ARN=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['executionArn'])" 2>/dev/null || echo "")

# 5. DescribeExecution
OUT=$(aws_sfn describe-execution --execution-arn "$EXEC_ARN")
assert_contains "DescribeExecution" "$OUT" "my-execution"
assert_contains "DescribeExecution status" "$OUT" "RUNNING"
assert_contains "DescribeExecution input" "$OUT" "key"

# 6. ListExecutions
OUT=$(aws_sfn list-executions --state-machine-arn "$SM_ARN")
assert_contains "ListExecutions" "$OUT" "my-execution"
assert_contains "ListExecutions status" "$OUT" "RUNNING"

# 7. GetExecutionHistory
OUT=$(aws_sfn get-execution-history --execution-arn "$EXEC_ARN")
assert_contains "GetExecutionHistory" "$OUT" "events"
assert_contains "GetExecutionHistory type" "$OUT" "ExecutionStarted"

# 8. StopExecution
OUT=$(aws_sfn stop-execution --execution-arn "$EXEC_ARN")
assert_contains "StopExecution" "$OUT" "stopDate"

# 9. DescribeExecution after stop
OUT=$(aws_sfn describe-execution --execution-arn "$EXEC_ARN")
assert_contains "DescribeExecution after stop" "$OUT" "ABORTED"

# 10. TagResource
OUT=$(aws_sfn tag-resource \
  --resource-arn "$SM_ARN" \
  --tags key=env,value=test key=team,value=platform)
assert_contains "TagResource" "$OUT" ""

# 11. ListTagsForResource
OUT=$(aws_sfn list-tags-for-resource --resource-arn "$SM_ARN")
assert_contains "ListTagsForResource" "$OUT" "tags"
assert_contains "ListTagsForResource env" "$OUT" "env"

# 12. UntagResource
OUT=$(aws_sfn untag-resource --resource-arn "$SM_ARN" --tag-keys env)
assert_contains "UntagResource" "$OUT" ""

# 13. CreateStateMachine duplicate
OUT=$(aws_sfn create-state-machine \
  --name my-state-machine \
  --definition "$DEFINITION" \
  --role-arn "$ROLE_ARN")
assert_contains "CreateStateMachine duplicate" "$OUT" "StateMachineAlreadyExists"

# 14. StartExecution duplicate name
OUT=$(aws_sfn start-execution \
  --state-machine-arn "$SM_ARN" \
  --name my-execution \
  --input '{}')
assert_contains "StartExecution duplicate" "$OUT" "ExecutionAlreadyExists"

# 15. DeleteStateMachine
OUT=$(aws_sfn delete-state-machine --state-machine-arn "$SM_ARN")
assert_contains "DeleteStateMachine" "$OUT" ""

# 16. DescribeStateMachine not found
OUT=$(aws_sfn describe-state-machine --state-machine-arn "$SM_ARN")
assert_contains "DescribeStateMachine not found" "$OUT" "StateMachineDoesNotExist"

# ── report ───────────────────────────────────────────────────────────────

echo ""
echo "══════════════════════════════════════════════"
echo "  Step Functions Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"
