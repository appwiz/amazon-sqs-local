#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port licensemanager)
ENDPOINT="http://localhost:${PORT}"

aws_licensemanager() {
  aws licensemanager "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for LicenseManager"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  LicenseManager server started")

report_results "LICENSEMANAGER"
exit $?
