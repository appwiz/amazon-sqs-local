#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port cognito)
ENDPOINT="http://localhost:${PORT}"

aws_cognito() {
  aws cognito-idp "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

# ── Tests ────────────────────────────────────────────────────────────────

echo "Running Cognito integration tests..."

# 1. CreateUserPool
OUT=$(aws_cognito create-user-pool --pool-name MyTestPool)
assert_contains "CreateUserPool name" "$OUT" "MyTestPool"
assert_contains "CreateUserPool status" "$OUT" "Active"
assert_contains "CreateUserPool arn" "$OUT" "arn:aws:cognito-idp"
POOL_ID=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['UserPool']['Id'])" 2>/dev/null)

# 2. DescribeUserPool
OUT=$(aws_cognito describe-user-pool --user-pool-id "$POOL_ID")
assert_contains "DescribeUserPool name" "$OUT" "MyTestPool"
assert_contains "DescribeUserPool id" "$OUT" "$POOL_ID"

# 3. ListUserPools
OUT=$(aws_cognito list-user-pools --max-results 10)
assert_contains "ListUserPools" "$OUT" "MyTestPool"

# 4. CreateUserPool (second pool for list test)
OUT2=$(aws_cognito create-user-pool --pool-name SecondPool)
assert_contains "CreateUserPool second" "$OUT2" "SecondPool"

# 5. ListUserPools shows both
OUT=$(aws_cognito list-user-pools --max-results 10)
assert_contains "ListUserPools both pools" "$OUT" "MyTestPool"
assert_contains "ListUserPools second pool" "$OUT" "SecondPool"

# 6. AdminCreateUser
OUT=$(aws_cognito admin-create-user \
  --user-pool-id "$POOL_ID" \
  --username johndoe \
  --user-attributes Name=email,Value=john@example.com Name=name,Value="John Doe" \
  --temporary-password "Temp1234!")
assert_contains "AdminCreateUser username" "$OUT" "johndoe"
assert_contains "AdminCreateUser status" "$OUT" "FORCE_CHANGE_PASSWORD"
assert_contains "AdminCreateUser email" "$OUT" "john@example.com"

# 7. AdminGetUser
OUT=$(aws_cognito admin-get-user \
  --user-pool-id "$POOL_ID" \
  --username johndoe)
assert_contains "AdminGetUser username" "$OUT" "johndoe"
assert_contains "AdminGetUser email" "$OUT" "john@example.com"

# 8. ListUsers
OUT=$(aws_cognito list-users --user-pool-id "$POOL_ID")
assert_contains "ListUsers" "$OUT" "johndoe"

# 9. AdminSetUserPassword
OUT=$(aws_cognito admin-set-user-password \
  --user-pool-id "$POOL_ID" \
  --username johndoe \
  --password "NewPass1234!" \
  --permanent)
assert_contains "AdminSetUserPassword" "$OUT" ""

# Verify user is now CONFIRMED
OUT=$(aws_cognito admin-get-user --user-pool-id "$POOL_ID" --username johndoe)
assert_contains "AdminSetUserPassword confirmed" "$OUT" "CONFIRMED"

# 10. AdminDisableUser
OUT=$(aws_cognito admin-disable-user \
  --user-pool-id "$POOL_ID" \
  --username johndoe)
assert_contains "AdminDisableUser" "$OUT" ""

OUT=$(aws_cognito admin-get-user --user-pool-id "$POOL_ID" --username johndoe)
assert_contains "AdminDisableUser enabled false" "$OUT" "false"

# 11. AdminEnableUser
OUT=$(aws_cognito admin-enable-user \
  --user-pool-id "$POOL_ID" \
  --username johndoe)
assert_contains "AdminEnableUser" "$OUT" ""

OUT=$(aws_cognito admin-get-user --user-pool-id "$POOL_ID" --username johndoe)
assert_contains "AdminEnableUser enabled true" "$OUT" "true"

# 12. CreateUserPoolClient
OUT=$(aws_cognito create-user-pool-client \
  --user-pool-id "$POOL_ID" \
  --client-name MyAppClient \
  --explicit-auth-flows ALLOW_USER_PASSWORD_AUTH ALLOW_REFRESH_TOKEN_AUTH)
assert_contains "CreateUserPoolClient name" "$OUT" "MyAppClient"
assert_contains "CreateUserPoolClient pool id" "$OUT" "$POOL_ID"
CLIENT_ID=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['UserPoolClient']['ClientId'])" 2>/dev/null)

# 13. DescribeUserPoolClient
OUT=$(aws_cognito describe-user-pool-client \
  --user-pool-id "$POOL_ID" \
  --client-id "$CLIENT_ID")
assert_contains "DescribeUserPoolClient name" "$OUT" "MyAppClient"
assert_contains "DescribeUserPoolClient id" "$OUT" "$CLIENT_ID"

# 14. ListUserPoolClients
OUT=$(aws_cognito list-user-pool-clients --user-pool-id "$POOL_ID")
assert_contains "ListUserPoolClients" "$OUT" "MyAppClient"

# 15. CreateGroup
OUT=$(aws_cognito create-group \
  --user-pool-id "$POOL_ID" \
  --group-name admins \
  --description "Admin group")
assert_contains "CreateGroup name" "$OUT" "admins"
assert_contains "CreateGroup description" "$OUT" "Admin group"

# 16. GetGroup
OUT=$(aws_cognito get-group \
  --user-pool-id "$POOL_ID" \
  --group-name admins)
assert_contains "GetGroup" "$OUT" "admins"

# 17. ListGroups
OUT=$(aws_cognito list-groups --user-pool-id "$POOL_ID")
assert_contains "ListGroups" "$OUT" "admins"

# 18. AdminAddUserToGroup
OUT=$(aws_cognito admin-add-user-to-group \
  --user-pool-id "$POOL_ID" \
  --username johndoe \
  --group-name admins)
assert_contains "AdminAddUserToGroup" "$OUT" ""

# 19. AdminListGroupsForUser
OUT=$(aws_cognito admin-list-groups-for-user \
  --user-pool-id "$POOL_ID" \
  --username johndoe)
assert_contains "AdminListGroupsForUser" "$OUT" "admins"

# 20. ListUsersInGroup
OUT=$(aws_cognito list-users-in-group \
  --user-pool-id "$POOL_ID" \
  --group-name admins)
assert_contains "ListUsersInGroup" "$OUT" "johndoe"

# 21. AdminRemoveUserFromGroup
OUT=$(aws_cognito admin-remove-user-from-group \
  --user-pool-id "$POOL_ID" \
  --username johndoe \
  --group-name admins)
assert_contains "AdminRemoveUserFromGroup" "$OUT" ""

OUT=$(aws_cognito admin-list-groups-for-user \
  --user-pool-id "$POOL_ID" \
  --username johndoe)
assert_not_contains "AdminRemoveUserFromGroup verify" "$OUT" "admins"

# 22. AdminInitiateAuth
OUT=$(aws_cognito admin-initiate-auth \
  --user-pool-id "$POOL_ID" \
  --client-id "$CLIENT_ID" \
  --auth-flow ADMIN_USER_PASSWORD_AUTH \
  --auth-parameters USERNAME=johndoe,PASSWORD=NewPass1234!)
assert_contains "AdminInitiateAuth access token" "$OUT" "AccessToken"
assert_contains "AdminInitiateAuth id token" "$OUT" "IdToken"
assert_contains "AdminInitiateAuth refresh token" "$OUT" "RefreshToken"

# 23. SignUp
OUT=$(aws_cognito sign-up \
  --client-id "$CLIENT_ID" \
  --username newuser \
  --password "NewUser123!" \
  --user-attributes Name=email,Value=new@example.com)
assert_contains "SignUp user sub" "$OUT" "UserSub"
assert_contains "SignUp not confirmed" "$OUT" "false"

# 24. ConfirmSignUp
OUT=$(aws_cognito confirm-sign-up \
  --client-id "$CLIENT_ID" \
  --username newuser \
  --confirmation-code 123456)
assert_contains "ConfirmSignUp" "$OUT" ""

OUT=$(aws_cognito admin-get-user --user-pool-id "$POOL_ID" --username newuser)
assert_contains "ConfirmSignUp confirmed" "$OUT" "CONFIRMED"

# 25. Error cases
OUT=$(aws_cognito admin-get-user --user-pool-id "$POOL_ID" --username nonexistent)
assert_contains "AdminGetUser not found" "$OUT" "UserNotFoundException"

OUT=$(aws_cognito admin-create-user \
  --user-pool-id "$POOL_ID" \
  --username johndoe \
  --temporary-password "Temp1234!")
assert_contains "AdminCreateUser duplicate" "$OUT" "UsernameExistsException"

OUT=$(aws_cognito describe-user-pool --user-pool-id "us-east-1_nonexistent")
assert_contains "DescribeUserPool not found" "$OUT" "ResourceNotFoundException"

# 26. AdminDeleteUser
OUT=$(aws_cognito admin-delete-user \
  --user-pool-id "$POOL_ID" \
  --username johndoe)
assert_contains "AdminDeleteUser" "$OUT" ""

OUT=$(aws_cognito list-users --user-pool-id "$POOL_ID")
assert_not_contains "AdminDeleteUser verify" "$OUT" "johndoe"

# 27. DeleteGroup
OUT=$(aws_cognito delete-group \
  --user-pool-id "$POOL_ID" \
  --group-name admins)
assert_contains "DeleteGroup" "$OUT" ""

# 28. DeleteUserPoolClient
OUT=$(aws_cognito delete-user-pool-client \
  --user-pool-id "$POOL_ID" \
  --client-id "$CLIENT_ID")
assert_contains "DeleteUserPoolClient" "$OUT" ""

# 29. DeleteUserPool
OUT=$(aws_cognito delete-user-pool --user-pool-id "$POOL_ID")
assert_contains "DeleteUserPool" "$OUT" ""

OUT=$(aws_cognito describe-user-pool --user-pool-id "$POOL_ID")
assert_contains "DeleteUserPool verify" "$OUT" "ResourceNotFoundException"

# ── report ───────────────────────────────────────────────────────────────

report_results "Cognito"
exit $?
