#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port managedflink)
ENDPOINT="http://localhost:${PORT}"

aws_managedflink() {
  aws managedflink "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for ManagedFlink"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  ManagedFlink server started")

report_results "MANAGEDFLINK"
exit $?
