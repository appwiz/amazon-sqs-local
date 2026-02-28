#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port managedprometheus)
ENDPOINT="http://localhost:${PORT}"

aws_managedprometheus() {
  aws managedprometheus "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for ManagedPrometheus"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  ManagedPrometheus server started")

report_results "MANAGEDPROMETHEUS"
exit $?
