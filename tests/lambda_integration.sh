#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port lambda)
ENDPOINT="http://localhost:${PORT}"

aws_lambda() {
  aws lambda "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

trap 'rm -f /tmp/lambda-test-output.json /tmp/lambda-dummy.zip /tmp/lambda-dummy-file.txt' EXIT

ensure_server

# Create a dummy zip file for function deployments
echo "dummy" > /tmp/lambda-dummy-file.txt
(cd /tmp && zip -q lambda-dummy.zip lambda-dummy-file.txt)

# ── Tests ────────────────────────────────────────────────────────────────

echo "Running Lambda integration tests..."

# 1. CreateFunction
OUT=$(aws_lambda create-function \
  --function-name my-func \
  --runtime python3.12 \
  --role "arn:aws:iam::${ACCOUNT}:role/test-role" \
  --handler index.handler \
  --zip-file fileb:///tmp/lambda-dummy.zip)
assert_contains "CreateFunction name" "$OUT" "my-func"
assert_contains "CreateFunction runtime" "$OUT" "python3.12"
assert_contains "CreateFunction state" "$OUT" "Active"

# 2. ListFunctions
OUT=$(aws_lambda list-functions)
assert_contains "ListFunctions" "$OUT" "my-func"

# 3. GetFunction
OUT=$(aws_lambda get-function --function-name my-func)
assert_contains "GetFunction config" "$OUT" "my-func"
assert_contains "GetFunction code" "$OUT" "S3"

# 4. UpdateFunctionConfiguration
OUT=$(aws_lambda update-function-configuration \
  --function-name my-func \
  --timeout 30 \
  --memory-size 256)
assert_contains "UpdateFunctionConfiguration timeout" "$OUT" "30"
assert_contains "UpdateFunctionConfiguration memory" "$OUT" "256"

# 5. Invoke
OUT=$(aws_lambda invoke \
  --function-name my-func \
  /tmp/lambda-test-output.json)
assert_contains "Invoke status" "$OUT" "200"

# 6. PublishVersion
OUT=$(aws_lambda publish-version --function-name my-func)
assert_contains "PublishVersion" "$OUT" '"Version": "1"'

# 7. ListVersionsByFunction
OUT=$(aws_lambda list-versions-by-function --function-name my-func)
assert_contains "ListVersionsByFunction LATEST" "$OUT" "\$LATEST"
assert_contains "ListVersionsByFunction v1" "$OUT" '"Version": "1"'

# 8. CreateAlias
OUT=$(aws_lambda create-alias \
  --function-name my-func \
  --name prod \
  --function-version 1)
assert_contains "CreateAlias name" "$OUT" "prod"
assert_contains "CreateAlias version" "$OUT" '"FunctionVersion": "1"'

# 9. GetAlias
OUT=$(aws_lambda get-alias \
  --function-name my-func \
  --name prod)
assert_contains "GetAlias" "$OUT" "prod"

# 10. ListAliases
OUT=$(aws_lambda list-aliases --function-name my-func)
assert_contains "ListAliases" "$OUT" "prod"

# 11. DeleteAlias
OUT=$(aws_lambda delete-alias \
  --function-name my-func \
  --name prod)
assert_contains "DeleteAlias" "$OUT" ""

# 12. AddPermission
OUT=$(aws_lambda add-permission \
  --function-name my-func \
  --statement-id s3-invoke \
  --action lambda:InvokeFunction \
  --principal s3.amazonaws.com)
assert_contains "AddPermission" "$OUT" "s3-invoke"

# 13. GetPolicy
OUT=$(aws_lambda get-policy --function-name my-func)
assert_contains "GetPolicy" "$OUT" "s3-invoke"

# 14. RemovePermission
OUT=$(aws_lambda remove-permission \
  --function-name my-func \
  --statement-id s3-invoke)
assert_contains "RemovePermission" "$OUT" ""

# 15. TagResource
OUT=$(aws_lambda tag-resource \
  --resource "arn:aws:lambda:${REGION}:${ACCOUNT}:function:my-func" \
  --tags env=test,team=backend)
assert_contains "TagResource" "$OUT" ""

# 16. ListTags
OUT=$(aws_lambda list-tags \
  --resource "arn:aws:lambda:${REGION}:${ACCOUNT}:function:my-func")
assert_contains "ListTags env" "$OUT" "env"
assert_contains "ListTags team" "$OUT" "backend"

# 17. UntagResource
OUT=$(aws_lambda untag-resource \
  --resource "arn:aws:lambda:${REGION}:${ACCOUNT}:function:my-func" \
  --tag-keys env)
assert_contains "UntagResource" "$OUT" ""

# 18. CreateFunction duplicate
OUT=$(aws_lambda create-function \
  --function-name my-func \
  --runtime python3.12 \
  --role "arn:aws:iam::${ACCOUNT}:role/test-role" \
  --handler index.handler \
  --zip-file fileb:///tmp/lambda-dummy.zip)
assert_contains "CreateFunction duplicate" "$OUT" "ResourceConflictException"

# 19. GetFunction not found
OUT=$(aws_lambda get-function --function-name nonexistent)
assert_contains "GetFunction not found" "$OUT" "ResourceNotFoundException"

# 20. DeleteFunction
OUT=$(aws_lambda delete-function --function-name my-func)
assert_contains "DeleteFunction" "$OUT" ""

# 21. DeleteFunction not found
OUT=$(aws_lambda delete-function --function-name my-func)
assert_contains "DeleteFunction not found" "$OUT" "ResourceNotFoundException"

# ── report ───────────────────────────────────────────────────────────────

report_results "Lambda"
exit $?
