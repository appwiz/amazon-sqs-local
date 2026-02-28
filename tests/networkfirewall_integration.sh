#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port networkfirewall)
ENDPOINT="http://localhost:${PORT}"

aws_networkfirewall() {
  aws networkfirewall "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for NetworkFirewall"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  NetworkFirewall server started")

report_results "NETWORKFIREWALL"
exit $?
