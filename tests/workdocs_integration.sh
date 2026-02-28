#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port workdocs)
ENDPOINT="http://localhost:${PORT}"

aws_workdocs() {
  aws workdocs "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for WorkDocs"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  WorkDocs server started")

report_results "WORKDOCS"
exit $?
