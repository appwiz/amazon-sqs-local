#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port macie)
ENDPOINT="http://localhost:${PORT}"

aws_macie() {
  aws macie "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Macie"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Macie server started")

report_results "MACIE"
exit $?
