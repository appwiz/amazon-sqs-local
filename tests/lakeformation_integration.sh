#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port lakeformation)
ENDPOINT="http://localhost:${PORT}"

aws_lakeformation() {
  aws lakeformation "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for LakeFormation"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  LakeFormation server started")

report_results "LAKEFORMATION"
exit $?
