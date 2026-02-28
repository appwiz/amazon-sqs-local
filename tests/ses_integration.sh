#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port ses)
ENDPOINT="http://localhost:${PORT}"

aws_ses() {
  aws sesv2 "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

# 1. CreateEmailIdentity (email address)
OUT=$(aws_ses create-email-identity --email-identity sender@testdomain.org)
assert_contains "CreateEmailIdentity email" "$OUT" "IdentityType"
assert_contains "CreateEmailIdentity verified" "$OUT" "VerifiedForSendingStatus"

# 2. CreateEmailIdentity (domain)
OUT=$(aws_ses create-email-identity --email-identity mydomain.net)
assert_contains "CreateEmailIdentity domain" "$OUT" "IdentityType"

# 3. ListEmailIdentities
OUT=$(aws_ses list-email-identities)
assert_contains "ListEmailIdentities" "$OUT" "EmailIdentities"
assert_contains "ListEmailIdentities sender" "$OUT" "sender@testdomain.org"
assert_contains "ListEmailIdentities domain" "$OUT" "mydomain.net"

# 4. GetEmailIdentity
OUT=$(aws_ses get-email-identity --email-identity sender@testdomain.org)
assert_contains "GetEmailIdentity" "$OUT" "EMAIL_ADDRESS"
assert_contains "GetEmailIdentity verified" "$OUT" "VerifiedForSendingStatus"

# 5. SendEmail (simple)
OUT=$(aws_ses send-email \
  --from-email-address sender@testdomain.org \
  --destination '{"ToAddresses":["recipient@testdomain.org"]}' \
  --content '{"Simple":{"Subject":{"Data":"Test Subject"},"Body":{"Text":{"Data":"Hello World"}}}}')
assert_contains "SendEmail" "$OUT" "MessageId"

# 6. SendEmail (with CC and BCC)
OUT=$(aws_ses send-email \
  --from-email-address sender@testdomain.org \
  --destination '{"ToAddresses":["to@testdomain.org"],"CcAddresses":["cc@testdomain.org"]}' \
  --content '{"Simple":{"Subject":{"Data":"CC Test"},"Body":{"Text":{"Data":"CC Body"}}}}')
assert_contains "SendEmail CC" "$OUT" "MessageId"

# 7. CreateEmailIdentity duplicate
OUT=$(aws_ses create-email-identity --email-identity sender@testdomain.org)
assert_contains "CreateEmailIdentity duplicate" "$OUT" "AlreadyExistsException"

# 8. GetEmailIdentity not found
OUT=$(aws_ses get-email-identity --email-identity unknown@nowhere.com)
assert_contains "GetEmailIdentity not found" "$OUT" "NotFoundException"

# 9. DeleteEmailIdentity
OUT=$(aws_ses delete-email-identity --email-identity mydomain.net)
assert_contains "DeleteEmailIdentity" "$OUT" ""

# 10. ListEmailIdentities after delete
OUT=$(aws_ses list-email-identities)
assert_not_contains "ListEmailIdentities after delete" "$OUT" "mydomain.net"
assert_contains "ListEmailIdentities still has sender" "$OUT" "sender@testdomain.org"

# ── report ───────────────────────────────────────────────────────────────

report_results "SES"
exit $?
