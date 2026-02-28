#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port iam)
ENDPOINT="http://localhost:${PORT}"

aws_iam() {
  aws iam "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for IAM"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  IAM server started")

report_results "IAM"
exit $?
