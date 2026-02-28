#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port iotsitewise)
ENDPOINT="http://localhost:${PORT}"

aws_iotsitewise() {
  aws iotsitewise "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for IoTSiteWise"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  IoTSiteWise server started")

report_results "IOTSITEWISE"
exit $?
