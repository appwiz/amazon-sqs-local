#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port ecr)
ENDPOINT="http://localhost:${PORT}"

aws_ecr() {
  aws ecr "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for ECR"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  ECR server started")

report_results "ECR"
exit $?
