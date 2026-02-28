#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port ssm)
ENDPOINT="http://localhost:${PORT}"

aws_ssm() {
  aws ssm "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

# 1. PutParameter (String)
OUT=$(aws_ssm put-parameter \
  --name /myapp/db/host \
  --value "localhost" \
  --type String)
assert_contains "PutParameter" "$OUT" "Version"
assert_contains "PutParameter tier" "$OUT" "Tier"

# 2. GetParameter
OUT=$(aws_ssm get-parameter --name /myapp/db/host)
assert_contains "GetParameter" "$OUT" "Parameter"
assert_contains "GetParameter value" "$OUT" "localhost"
assert_contains "GetParameter name" "$OUT" "/myapp/db/host"

# 3. PutParameter (SecureString)
OUT=$(aws_ssm put-parameter \
  --name /myapp/db/password \
  --value "s3cr3t" \
  --type SecureString)
assert_contains "PutParameter SecureString" "$OUT" "Version"

# 4. PutParameter (StringList)
OUT=$(aws_ssm put-parameter \
  --name /myapp/allowed-hosts \
  --value "host1,host2,host3" \
  --type StringList)
assert_contains "PutParameter StringList" "$OUT" "Version"

# 5. GetParameters (multiple)
OUT=$(aws_ssm get-parameters \
  --names /myapp/db/host /myapp/db/password /myapp/does-not-exist)
assert_contains "GetParameters" "$OUT" "Parameters"
assert_contains "GetParameters valid" "$OUT" "localhost"
assert_contains "GetParameters invalid" "$OUT" "InvalidParameters"
assert_contains "GetParameters missing" "$OUT" "does-not-exist"

# 6. GetParametersByPath (recursive)
OUT=$(aws_ssm get-parameters-by-path \
  --path /myapp \
  --recursive)
assert_contains "GetParametersByPath" "$OUT" "Parameters"
assert_contains "GetParametersByPath host" "$OUT" "localhost"
assert_contains "GetParametersByPath password" "$OUT" "s3cr3t"

# 7. GetParametersByPath (non-recursive)
OUT=$(aws_ssm get-parameters-by-path \
  --path /myapp/db)
assert_contains "GetParametersByPath non-recursive" "$OUT" "localhost"

# 8. Overwrite parameter
OUT=$(aws_ssm put-parameter \
  --name /myapp/db/host \
  --value "prod-server.example.com" \
  --type String \
  --overwrite)
assert_contains "PutParameter overwrite" "$OUT" "Version"

# 9. Verify overwrite
OUT=$(aws_ssm get-parameter --name /myapp/db/host)
assert_contains "GetParameter after overwrite" "$OUT" "prod-server.example.com"
assert_not_contains "GetParameter after overwrite old" "$OUT" "localhost"

# 10. DescribeParameters
OUT=$(aws_ssm describe-parameters)
assert_contains "DescribeParameters" "$OUT" "Parameters"
assert_contains "DescribeParameters name" "$OUT" "/myapp/db/host"

# 11. DeleteParameter
OUT=$(aws_ssm delete-parameter --name /myapp/allowed-hosts)
assert_contains "DeleteParameter" "$OUT" ""

# 12. GetParameter after delete
OUT=$(aws_ssm get-parameter --name /myapp/allowed-hosts)
assert_contains "GetParameter after delete" "$OUT" "ParameterNotFound"

# 13. DeleteParameters (batch)
OUT=$(aws_ssm delete-parameters \
  --names /myapp/db/host /myapp/db/password /nonexistent)
assert_contains "DeleteParameters" "$OUT" "DeletedParameters"
assert_contains "DeleteParameters invalid" "$OUT" "InvalidParameters"
assert_contains "DeleteParameters nonexistent" "$OUT" "nonexistent"

# 14. PutParameter no overwrite conflict
OUT=$(aws_ssm put-parameter \
  --name /myapp/db/host \
  --value "value1" \
  --type String)
assert_contains "PutParameter first" "$OUT" "Version"

OUT=$(aws_ssm put-parameter \
  --name /myapp/db/host \
  --value "value2" \
  --type String)
assert_contains "PutParameter no-overwrite conflict" "$OUT" "ParameterAlreadyExists"

# ── report ───────────────────────────────────────────────────────────────

report_results "SSM"
exit $?
