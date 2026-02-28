#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port stepfunctions)
ENDPOINT="http://localhost:${PORT}"

aws_sfn() {
  aws stepfunctions "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

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

report_results "Step Functions"
exit $?
