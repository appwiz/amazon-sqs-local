#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port secretsmanager)
ENDPOINT="http://localhost:${PORT}"

aws_sm() {
  aws secretsmanager "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

# 1. CreateSecret
OUT=$(aws_sm create-secret \
  --name mydb/credentials \
  --secret-string '{"username":"admin","password":"s3cr3t"}' \
  --description "DB credentials")
assert_contains "CreateSecret" "$OUT" "ARN"
assert_contains "CreateSecret name" "$OUT" "mydb/credentials"
assert_contains "CreateSecret version" "$OUT" "VersionId"

# 2. GetSecretValue
OUT=$(aws_sm get-secret-value --secret-id mydb/credentials)
assert_contains "GetSecretValue" "$OUT" "SecretString"
assert_contains "GetSecretValue content" "$OUT" "admin"
assert_contains "GetSecretValue AWSCURRENT" "$OUT" "AWSCURRENT"

# 3. DescribeSecret
OUT=$(aws_sm describe-secret --secret-id mydb/credentials)
assert_contains "DescribeSecret ARN" "$OUT" "ARN"
assert_contains "DescribeSecret name" "$OUT" "mydb/credentials"
assert_contains "DescribeSecret desc" "$OUT" "DB credentials"

# 4. ListSecrets
OUT=$(aws_sm list-secrets)
assert_contains "ListSecrets" "$OUT" "mydb/credentials"
assert_contains "ListSecrets list" "$OUT" "SecretList"

# 5. PutSecretValue
OUT=$(aws_sm put-secret-value \
  --secret-id mydb/credentials \
  --secret-string '{"username":"admin","password":"newpass"}')
assert_contains "PutSecretValue" "$OUT" "VersionId"

# 6. GetSecretValue after update
OUT=$(aws_sm get-secret-value --secret-id mydb/credentials)
assert_contains "GetSecretValue updated" "$OUT" "newpass"

# 7. UpdateSecret description
OUT=$(aws_sm update-secret --secret-id mydb/credentials --description "Updated DB creds")
assert_contains "UpdateSecret" "$OUT" "ARN"

# 8. TagResource
OUT=$(aws_sm tag-resource --secret-id mydb/credentials \
  --tags Key=env,Value=prod Key=team,Value=platform)
assert_contains "TagResource" "$OUT" ""

# 9. Verify tags via DescribeSecret
OUT=$(aws_sm describe-secret --secret-id mydb/credentials)
assert_contains "TagResource verify" "$OUT" "prod"

# 10. UntagResource
OUT=$(aws_sm untag-resource --secret-id mydb/credentials --tag-keys env)
assert_contains "UntagResource" "$OUT" ""

# 11. ListSecretVersionIds
OUT=$(aws_sm list-secret-version-ids --secret-id mydb/credentials)
assert_contains "ListSecretVersionIds" "$OUT" "Versions"
assert_contains "ListSecretVersionIds ARN" "$OUT" "ARN"

# 12. CreateSecret duplicate
OUT=$(aws_sm create-secret --name mydb/credentials --secret-string "dup")
assert_contains "CreateSecret duplicate" "$OUT" "ResourceExistsException"

# 13. GetSecretValue not found
OUT=$(aws_sm get-secret-value --secret-id nonexistent-secret)
assert_contains "GetSecretValue not found" "$OUT" "ResourceNotFoundException"

# 14. Create another secret
OUT=$(aws_sm create-secret --name api/key --secret-string "api-key-12345")
assert_contains "CreateSecret api/key" "$OUT" "ARN"

# 15. ListSecrets shows both
OUT=$(aws_sm list-secrets)
assert_contains "ListSecrets both" "$OUT" "api/key"

# 16. DeleteSecret
OUT=$(aws_sm delete-secret --secret-id api/key --force-delete-without-recovery)
assert_contains "DeleteSecret" "$OUT" "DeletionDate"

# 17. GetSecretValue after delete
OUT=$(aws_sm get-secret-value --secret-id api/key)
assert_contains "GetSecretValue deleted" "$OUT" "ResourceNotFoundException"

# ── report ───────────────────────────────────────────────────────────────

report_results "Secrets Manager"
exit $?
