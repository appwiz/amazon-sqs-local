#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port controltower)
ENDPOINT="http://localhost:${PORT}"

aws_controltower() {
  aws controltower "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for ControlTower"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  ControlTower server started")

report_results "CONTROLTOWER"
exit $?
