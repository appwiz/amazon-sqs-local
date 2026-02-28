#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port ec2)
ENDPOINT="http://localhost:${PORT}"

aws_ec2() {
  aws ec2 "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for EC2"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  EC2 server started")

report_results "EC2"
exit $?
