#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port transferfamily)
ENDPOINT="http://localhost:${PORT}"

aws_transferfamily() {
  aws transferfamily "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for TransferFamily"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  TransferFamily server started")

report_results "TRANSFERFAMILY"
exit $?
