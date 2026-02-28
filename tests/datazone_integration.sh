#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port datazone)
ENDPOINT="http://localhost:${PORT}"

aws_datazone() {
  aws datazone "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for DataZone"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  DataZone server started")

report_results "DATAZONE"
exit $?
