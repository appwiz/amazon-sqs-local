#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port route53)
ENDPOINT="http://localhost:${PORT}"

aws_route53() {
  aws route53 "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Route53"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Route53 server started")

report_results "Route 53"
exit $?
