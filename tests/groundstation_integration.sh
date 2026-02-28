#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port groundstation)
ENDPOINT="http://localhost:${PORT}"

aws_groundstation() {
  aws groundstation "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for GroundStation"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  GroundStation server started")

report_results "GROUNDSTATION"
exit $?
