#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port fsx)
ENDPOINT="http://localhost:${PORT}"

aws_fsx() {
  aws fsx "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for FSx"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  FSx server started")

report_results "FSX"
exit $?
