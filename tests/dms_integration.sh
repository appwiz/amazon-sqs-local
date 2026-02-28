#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port dms)
ENDPOINT="http://localhost:${PORT}"

aws_dms() {
  aws dms "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for DMS"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  DMS server started")

report_results "DMS"
exit $?
