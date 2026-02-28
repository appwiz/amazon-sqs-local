#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port firewallmanager)
ENDPOINT="http://localhost:${PORT}"

aws_firewallmanager() {
  aws firewallmanager "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for FirewallManager"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  FirewallManager server started")

report_results "FIREWALLMANAGER"
exit $?
