#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port datasync)
ENDPOINT="http://localhost:${PORT}"

aws_datasync() {
  aws datasync "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for DataSync"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  DataSync server started")

report_results "DATASYNC"
exit $?
