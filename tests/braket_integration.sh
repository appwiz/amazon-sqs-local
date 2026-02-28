#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port braket)
ENDPOINT="http://localhost:${PORT}"

aws_braket() {
  aws braket "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Braket"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Braket server started")

report_results "BRAKET"
exit $?
