#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port codecatalyst)
ENDPOINT="http://localhost:${PORT}"

aws_codecatalyst() {
  aws codecatalyst "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for CodeCatalyst"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  CodeCatalyst server started")

report_results "CodeCatalyst"
exit $?
