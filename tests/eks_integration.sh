#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port eks)
ENDPOINT="http://localhost:${PORT}"

aws_eks() {
  aws eks "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for EKS"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  EKS server started")

report_results "EKS"
exit $?
