#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port iamidentitycenter)
ENDPOINT="http://localhost:${PORT}"

aws_iamidentitycenter() {
  aws iamidentitycenter "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for IAMIdentityCenter"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  IAMIdentityCenter server started")

report_results "IAMIDENTITYCENTER"
exit $?
