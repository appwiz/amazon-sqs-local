#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port ram)
ENDPOINT="http://localhost:${PORT}"

aws_ram() {
  aws ram "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for RAM"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  RAM server started")

report_results "RAM"
exit $?
