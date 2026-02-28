#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port rds)
ENDPOINT="http://localhost:${PORT}"

aws_rds() {
  aws rds "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for RDS"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  RDS server started")

report_results "RDS"
exit $?
