#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port dataexchange)
ENDPOINT="http://localhost:${PORT}"

aws_dataexchange() {
  aws dataexchange "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for DataExchange"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  DataExchange server started")

report_results "DATAEXCHANGE"
exit $?
