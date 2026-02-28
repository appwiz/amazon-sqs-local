#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port codedeploy)
ENDPOINT="http://localhost:${PORT}"

aws_codedeploy() {
  aws codedeploy "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for CodeDeploy"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  CodeDeploy server started")

report_results "CodeDeploy"
exit $?
