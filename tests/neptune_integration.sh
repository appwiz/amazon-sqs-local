#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port neptune)
ENDPOINT="http://localhost:${PORT}"

aws_neptune() {
  aws neptune "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Neptune"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Neptune server started")

report_results "NEPTUNE"
exit $?
