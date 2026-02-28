#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port proton)
ENDPOINT="http://localhost:${PORT}"

aws_proton() {
  aws proton "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Proton"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Proton server started")

report_results "PROTON"
exit $?
