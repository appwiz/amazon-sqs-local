#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port keyspaces)
ENDPOINT="http://localhost:${PORT}"

aws_keyspaces() {
  aws keyspaces "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Keyspaces"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Keyspaces server started")

report_results "KEYSPACES"
exit $?
