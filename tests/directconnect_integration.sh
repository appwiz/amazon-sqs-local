#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port directconnect)
ENDPOINT="http://localhost:${PORT}"

aws_directconnect() {
  aws directconnect "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for DirectConnect"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  DirectConnect server started")

report_results "DIRECTCONNECT"
exit $?
