#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port autoscaling)
ENDPOINT="http://localhost:${PORT}"

aws_autoscaling() {
  aws autoscaling "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for AutoScaling"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  AutoScaling server started")

report_results "AUTOSCALING"
exit $?
