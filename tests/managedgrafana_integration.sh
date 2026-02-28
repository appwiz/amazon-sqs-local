#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port managedgrafana)
ENDPOINT="http://localhost:${PORT}"

aws_managedgrafana() {
  aws managedgrafana "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for ManagedGrafana"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  ManagedGrafana server started")

report_results "MANAGEDGRAFANA"
exit $?
