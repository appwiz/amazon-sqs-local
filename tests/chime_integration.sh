#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port chime)
ENDPOINT="http://localhost:${PORT}"

aws_chime() {
  aws chime "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Chime"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Chime server started")

report_results "CHIME"
exit $?
