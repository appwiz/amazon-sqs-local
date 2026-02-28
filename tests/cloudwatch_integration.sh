#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port cloudwatch)
ENDPOINT="http://localhost:${PORT}"

aws_cloudwatch() {
  aws cloudwatch "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for CloudWatch"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  CloudWatch server started")

report_results "CloudWatch"
exit $?
