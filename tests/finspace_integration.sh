#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port finspace)
ENDPOINT="http://localhost:${PORT}"

aws_finspace() {
  aws finspace "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for FinSpace"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  FinSpace server started")

report_results "FINSPACE"
exit $?
