#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port organizations)
ENDPOINT="http://localhost:${PORT}"

aws_organizations() {
  aws organizations "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Organizations"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Organizations server started")

report_results "ORGANIZATIONS"
exit $?
