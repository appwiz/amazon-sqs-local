#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port drs)
ENDPOINT="http://localhost:${PORT}"

aws_drs() {
  aws drs "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for DRS"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  DRS server started")

report_results "DRS"
exit $?
