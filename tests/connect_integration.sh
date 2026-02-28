#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port connect)
ENDPOINT="http://localhost:${PORT}"

aws_connect() {
  aws connect "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Connect"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Connect server started")

report_results "CONNECT"
exit $?
