#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port inspector)
ENDPOINT="http://localhost:${PORT}"

aws_inspector() {
  aws inspector "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Inspector"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Inspector server started")

report_results "INSPECTOR"
exit $?
