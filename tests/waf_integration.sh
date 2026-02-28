#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port waf)
ENDPOINT="http://localhost:${PORT}"

aws_waf() {
  aws waf "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for WAF"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  WAF server started")

report_results "WAF"
exit $?
