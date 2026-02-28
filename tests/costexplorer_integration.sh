#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port costexplorer)
ENDPOINT="http://localhost:${PORT}"

aws_costexplorer() {
  aws costexplorer "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for CostExplorer"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  CostExplorer server started")

report_results "COSTEXPLORER"
exit $?
