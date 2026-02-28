# Security, Identity & Compliance

## IAM

| | |
|---|---|
| **Port** | `10033` |
| **Protocol** | Query/XML (Action parameter) |
| **Endpoint** | `http://localhost:10033` |

### Supported Operations (9)

| Operation | Description |
|-----------|-------------|
| CreateUser | Create an IAM user |
| DescribeUsers | List and describe IAM users |
| DeleteUser | Delete an IAM user |
| CreateRole | Create an IAM role |
| DescribeRoles | List and describe IAM roles |
| DeleteRole | Delete an IAM role |
| CreatePolicy | Create an IAM policy |
| DescribePolicys | List and describe IAM policies |
| DeletePolicy | Delete an IAM policy |

### Usage with AWS CLI

```bash
# Create a user
aws iam create-user \
  --user-name my-user \
  --endpoint-url http://localhost:10033 \
  --no-sign-request

# List users
aws iam list-users \
  --endpoint-url http://localhost:10033 \
  --no-sign-request

# Create a role
aws iam create-role \
  --role-name my-role \
  --assume-role-policy-document '{"Version":"2012-10-17","Statement":[]}' \
  --endpoint-url http://localhost:10033 \
  --no-sign-request
```

---

## Cognito

| | |
|---|---|
| **Port** | `9229` |
| **Protocol** | JSON RPC (`AWSCognitoIdentityProviderService`) |
| **Endpoint** | `http://localhost:9229` |

### Supported Operations (33)

#### User Pool Management (5)

| Operation | Description |
|-----------|-------------|
| CreateUserPool | Create a new user pool |
| DescribeUserPool | Get details of a user pool |
| ListUserPools | List all user pools |
| UpdateUserPool | Update user pool settings |
| DeleteUserPool | Delete a user pool |

#### User Management (9)

| Operation | Description |
|-----------|-------------|
| AdminCreateUser | Create a user in a pool |
| AdminGetUser | Get user details |
| AdminDeleteUser | Delete a user |
| AdminSetUserPassword | Set a user's password |
| AdminEnableUser | Enable a user account |
| AdminDisableUser | Disable a user account |
| AdminResetUserPassword | Reset a user's password |
| AdminUpdateUserAttributes | Update user attributes |
| ListUsers | List all users in a pool |

#### User Pool Client Management (5)

| Operation | Description |
|-----------|-------------|
| CreateUserPoolClient | Create an app client |
| DescribeUserPoolClient | Get app client details |
| ListUserPoolClients | List all app clients |
| UpdateUserPoolClient | Update an app client |
| DeleteUserPoolClient | Delete an app client |

#### Group Management (8)

| Operation | Description |
|-----------|-------------|
| CreateGroup | Create a group in a pool |
| GetGroup | Get group details |
| ListGroups | List all groups in a pool |
| DeleteGroup | Delete a group |
| AdminAddUserToGroup | Add a user to a group |
| AdminRemoveUserFromGroup | Remove a user from a group |
| AdminListGroupsForUser | List groups a user belongs to |
| ListUsersInGroup | List users in a group |

#### Authentication (6)

| Operation | Description |
|-----------|-------------|
| InitiateAuth | Start an authentication flow |
| AdminInitiateAuth | Admin-initiated authentication flow |
| SignUp | Self-service user registration |
| ConfirmSignUp | Confirm user registration |
| ForgotPassword | Initiate forgot password flow |
| ConfirmForgotPassword | Complete forgot password flow |

### Wire Protocol

Cognito uses JSON RPC via `X-Amz-Target` header with prefix `AWSCognitoIdentityProviderService.`. All requests are `POST /` with `Content-Type: application/x-amz-json-1.1`.

### Usage with AWS CLI

```bash
# Create a user pool
aws cognito-idp create-user-pool \
  --pool-name my-pool \
  --endpoint-url http://localhost:9229 \
  --no-sign-request

# List user pools
aws cognito-idp list-user-pools \
  --max-results 10 \
  --endpoint-url http://localhost:9229 \
  --no-sign-request

# Describe a user pool
aws cognito-idp describe-user-pool \
  --user-pool-id <pool-id> \
  --endpoint-url http://localhost:9229 \
  --no-sign-request

# Create a user
aws cognito-idp admin-create-user \
  --user-pool-id <pool-id> \
  --username testuser \
  --endpoint-url http://localhost:9229 \
  --no-sign-request

# Get user details
aws cognito-idp admin-get-user \
  --user-pool-id <pool-id> \
  --username testuser \
  --endpoint-url http://localhost:9229 \
  --no-sign-request

# Set user password
aws cognito-idp admin-set-user-password \
  --user-pool-id <pool-id> \
  --username testuser \
  --password 'NewP@ss1!' \
  --permanent \
  --endpoint-url http://localhost:9229 \
  --no-sign-request

# List users
aws cognito-idp list-users \
  --user-pool-id <pool-id> \
  --endpoint-url http://localhost:9229 \
  --no-sign-request

# Create an app client
aws cognito-idp create-user-pool-client \
  --user-pool-id <pool-id> \
  --client-name my-client \
  --endpoint-url http://localhost:9229 \
  --no-sign-request

# Create a group
aws cognito-idp create-group \
  --user-pool-id <pool-id> \
  --group-name admins \
  --endpoint-url http://localhost:9229 \
  --no-sign-request

# Add user to group
aws cognito-idp admin-add-user-to-group \
  --user-pool-id <pool-id> \
  --username testuser \
  --group-name admins \
  --endpoint-url http://localhost:9229 \
  --no-sign-request

# Delete a user pool
aws cognito-idp delete-user-pool \
  --user-pool-id <pool-id> \
  --endpoint-url http://localhost:9229 \
  --no-sign-request
```

### Limitations

- Auth flows return stub token responses. No actual JWT signing or token validation is performed.
- All subscriptions are immediately confirmed without endpoint verification.

---

## KMS

| | |
|---|---|
| **Port** | `7600` |
| **Protocol** | JSON RPC (`TrentService`) |
| **Endpoint** | `http://localhost:7600` |

### Supported Operations (22)

#### Key Management (7)

| Operation | Description |
|-----------|-------------|
| CreateKey | Create a new KMS key |
| DescribeKey | Get details of a key |
| ListKeys | List all keys |
| ScheduleKeyDeletion | Schedule a key for deletion |
| CancelKeyDeletion | Cancel scheduled key deletion |
| EnableKey | Enable a disabled key |
| DisableKey | Disable a key |

#### Cryptographic Operations (7)

| Operation | Description |
|-----------|-------------|
| Encrypt | Encrypt plaintext (simulated) |
| Decrypt | Decrypt ciphertext (simulated) |
| GenerateDataKey | Generate a data key |
| GenerateDataKeyWithoutPlaintext | Generate data key without plaintext |
| GenerateRandom | Generate random bytes |
| Sign | Create a digital signature (simulated) |
| Verify | Verify a digital signature (simulated) |

#### Tagging (3)

| Operation | Description |
|-----------|-------------|
| TagResource | Add tags to a key |
| UntagResource | Remove tags from a key |
| ListResourceTags | List tags for a key |

#### Aliases (3)

| Operation | Description |
|-----------|-------------|
| CreateAlias | Create an alias for a key |
| DeleteAlias | Delete an alias |
| ListAliases | List all aliases |

#### Key Policy (2)

| Operation | Description |
|-----------|-------------|
| GetKeyPolicy | Get the key policy |
| PutKeyPolicy | Set the key policy |

### Wire Protocol

KMS uses JSON RPC via `X-Amz-Target` header with prefix `TrentService.`. All requests are `POST /` with `Content-Type: application/x-amz-json-1.1`.

### Usage with AWS CLI

```bash
# Create a key
aws kms create-key \
  --endpoint-url http://localhost:7600 \
  --no-sign-request

# List keys
aws kms list-keys \
  --endpoint-url http://localhost:7600 \
  --no-sign-request

# Describe a key
aws kms describe-key \
  --key-id <key-id> \
  --endpoint-url http://localhost:7600 \
  --no-sign-request

# Encrypt data
aws kms encrypt \
  --key-id <key-id> \
  --plaintext "Hello, world!" \
  --endpoint-url http://localhost:7600 \
  --no-sign-request

# Decrypt data
aws kms decrypt \
  --ciphertext-blob <ciphertext> \
  --endpoint-url http://localhost:7600 \
  --no-sign-request

# Generate a data key
aws kms generate-data-key \
  --key-id <key-id> \
  --key-spec AES_256 \
  --endpoint-url http://localhost:7600 \
  --no-sign-request

# Generate random bytes
aws kms generate-random \
  --number-of-bytes 32 \
  --endpoint-url http://localhost:7600 \
  --no-sign-request

# Create an alias
aws kms create-alias \
  --alias-name alias/my-key \
  --target-key-id <key-id> \
  --endpoint-url http://localhost:7600 \
  --no-sign-request

# List aliases
aws kms list-aliases \
  --endpoint-url http://localhost:7600 \
  --no-sign-request

# Tag a key
aws kms tag-resource \
  --key-id <key-id> \
  --tags TagKey=env,TagValue=test \
  --endpoint-url http://localhost:7600 \
  --no-sign-request

# Schedule key deletion
aws kms schedule-key-deletion \
  --key-id <key-id> \
  --pending-window-in-days 7 \
  --endpoint-url http://localhost:7600 \
  --no-sign-request
```

### Limitations

- Encrypt/Decrypt, Sign/Verify produce deterministic fake outputs. No actual cryptographic operations are performed.
- Key material is simulated and not cryptographically secure.

---

## Secrets Manager

| | |
|---|---|
| **Port** | `7700` |
| **Protocol** | JSON RPC (`secretsmanager`) |
| **Endpoint** | `http://localhost:7700` |

### Supported Operations (11)

| Operation | Description |
|-----------|-------------|
| CreateSecret | Create a new secret |
| GetSecretValue | Retrieve a secret's value |
| PutSecretValue | Store a new secret value |
| DescribeSecret | Get metadata about a secret |
| ListSecrets | List all secrets |
| UpdateSecret | Update a secret's metadata |
| DeleteSecret | Delete a secret |
| RestoreSecret | Restore a previously deleted secret |
| TagResource | Add tags to a secret |
| UntagResource | Remove tags from a secret |
| ListSecretVersionIds | List all version IDs for a secret |

### Wire Protocol

Secrets Manager uses JSON RPC via `X-Amz-Target` header with prefix `secretsmanager.`. All requests are `POST /` with `Content-Type: application/x-amz-json-1.1`.

### Usage with AWS CLI

```bash
# Create a secret
aws secretsmanager create-secret \
  --name my-secret \
  --secret-string '{"username":"admin","password":"s3cr3t"}' \
  --endpoint-url http://localhost:7700 \
  --no-sign-request

# Get a secret value
aws secretsmanager get-secret-value \
  --secret-id my-secret \
  --endpoint-url http://localhost:7700 \
  --no-sign-request

# List secrets
aws secretsmanager list-secrets \
  --endpoint-url http://localhost:7700 \
  --no-sign-request

# Describe a secret
aws secretsmanager describe-secret \
  --secret-id my-secret \
  --endpoint-url http://localhost:7700 \
  --no-sign-request

# Put a new secret value
aws secretsmanager put-secret-value \
  --secret-id my-secret \
  --secret-string '{"username":"admin","password":"n3wP@ss"}' \
  --endpoint-url http://localhost:7700 \
  --no-sign-request

# Update secret metadata
aws secretsmanager update-secret \
  --secret-id my-secret \
  --description "Updated description" \
  --endpoint-url http://localhost:7700 \
  --no-sign-request

# Tag a secret
aws secretsmanager tag-resource \
  --secret-id my-secret \
  --tags Key=env,Value=test \
  --endpoint-url http://localhost:7700 \
  --no-sign-request

# List secret version IDs
aws secretsmanager list-secret-version-ids \
  --secret-id my-secret \
  --endpoint-url http://localhost:7700 \
  --no-sign-request

# Delete a secret
aws secretsmanager delete-secret \
  --secret-id my-secret \
  --force-delete-without-recovery \
  --endpoint-url http://localhost:7700 \
  --no-sign-request

# Restore a deleted secret
aws secretsmanager restore-secret \
  --secret-id my-secret \
  --endpoint-url http://localhost:7700 \
  --no-sign-request
```

### Limitations

- `DeleteSecret` with `--force-delete-without-recovery` removes the secret immediately.
- No automatic rotation is performed.

---

## WAF

| | |
|---|---|
| **Port** | `10035` |
| **Protocol** | JSON RPC (`AWSWAF_20190729`) |
| **Endpoint** | `http://localhost:10035` |

### Supported Operations (8)

| Operation | Description |
|-----------|-------------|
| CreateWebACL | Create a web ACL |
| DescribeWebACL | Describe a web ACL |
| ListWebACLs | List all web ACLs |
| DeleteWebACL | Delete a web ACL |
| CreateIPSet | Create an IP set |
| DescribeIPSet | Describe an IP set |
| ListIPSets | List all IP sets |
| DeleteIPSet | Delete an IP set |

### Usage with AWS CLI

```bash
# Create a web ACL
aws wafv2 create-web-acl \
  --name my-acl \
  --scope REGIONAL \
  --default-action '{"Allow":{}}' \
  --visibility-config '{"SampledRequestsEnabled":false,"CloudWatchMetricsEnabled":false,"MetricName":"my-acl"}' \
  --endpoint-url http://localhost:10035 \
  --no-sign-request

# List web ACLs
aws wafv2 list-web-acls \
  --scope REGIONAL \
  --endpoint-url http://localhost:10035 \
  --no-sign-request
```

---

## Shield

| | |
|---|---|
| **Port** | `10036` |
| **Protocol** | JSON RPC (`AWSShield_20160616`) |
| **Endpoint** | `http://localhost:10036` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateProtection | Create a Shield protection |
| DescribeProtection | Describe a protection |
| ListProtections | List all protections |
| DeleteProtection | Delete a protection |

### Usage with AWS CLI

```bash
# Create a protection
aws shield create-protection \
  --name my-protection \
  --resource-arn arn:aws:ec2:us-east-1:000000000000:instance/i-1234567890abcdef0 \
  --endpoint-url http://localhost:10036 \
  --no-sign-request

# List protections
aws shield list-protections \
  --endpoint-url http://localhost:10036 \
  --no-sign-request
```

---

## GuardDuty

| | |
|---|---|
| **Port** | `10037` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10037` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateDetector | Create a GuardDuty detector |
| GetDetector | Get details of a detector |
| ListDetectors | List all detectors |
| DeleteDetector | Delete a detector |

### Usage with AWS CLI

```bash
# Create a detector
aws guardduty create-detector \
  --enable \
  --endpoint-url http://localhost:10037 \
  --no-sign-request

# List detectors
aws guardduty list-detectors \
  --endpoint-url http://localhost:10037 \
  --no-sign-request
```

---

## ACM

| | |
|---|---|
| **Port** | `10034` |
| **Protocol** | JSON RPC (`CertificateManager`) |
| **Endpoint** | `http://localhost:10034` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateCertificate | Request a certificate |
| DescribeCertificate | Describe a certificate |
| ListCertificates | List all certificates |
| DeleteCertificate | Delete a certificate |

### Usage with AWS CLI

```bash
# Request a certificate
aws acm request-certificate \
  --domain-name example.com \
  --endpoint-url http://localhost:10034 \
  --no-sign-request

# List certificates
aws acm list-certificates \
  --endpoint-url http://localhost:10034 \
  --no-sign-request
```

---

## Macie

| | |
|---|---|
| **Port** | `10039` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10039` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateFinding | Create a finding |
| GetFinding | Get details of a finding |
| ListFindings | List all findings |
| DeleteFinding | Delete a finding |

### Usage with AWS CLI

```bash
# List findings
aws macie2 list-findings \
  --endpoint-url http://localhost:10039 \
  --no-sign-request
```

---

## Inspector

| | |
|---|---|
| **Port** | `10038` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10038` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateFinding | Create a finding |
| GetFinding | Get details of a finding |
| ListFindings | List all findings |
| DeleteFinding | Delete a finding |

### Usage with AWS CLI

```bash
# List findings
aws inspector2 list-findings \
  --endpoint-url http://localhost:10038 \
  --no-sign-request
```

---

## Security Hub

| | |
|---|---|
| **Port** | `10046` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10046` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateFinding | Create a finding |
| GetFinding | Get details of a finding |
| ListFindings | List all findings |
| DeleteFinding | Delete a finding |

### Usage with AWS CLI

```bash
# List findings
aws securityhub get-findings \
  --endpoint-url http://localhost:10046 \
  --no-sign-request
```

---

## Security Lake

| | |
|---|---|
| **Port** | `10041` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10041` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateDataLake | Create a data lake |
| GetDataLake | Get details of a data lake |
| ListDataLakes | List all data lakes |
| DeleteDataLake | Delete a data lake |

### Usage with AWS CLI

```bash
# List data lakes
aws securitylake list-data-lakes \
  --endpoint-url http://localhost:10041 \
  --no-sign-request
```

---

## Verified Permissions

| | |
|---|---|
| **Port** | `10042` |
| **Protocol** | JSON RPC (`VerifiedPermissions`) |
| **Endpoint** | `http://localhost:10042` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreatePolicyStore | Create a policy store |
| DescribePolicyStore | Describe a policy store |
| ListPolicyStores | List all policy stores |
| DeletePolicyStore | Delete a policy store |

### Usage with AWS CLI

```bash
# Create a policy store
aws verifiedpermissions create-policy-store \
  --validation-settings '{"mode":"OFF"}' \
  --endpoint-url http://localhost:10042 \
  --no-sign-request

# List policy stores
aws verifiedpermissions list-policy-stores \
  --endpoint-url http://localhost:10042 \
  --no-sign-request
```

---

## CloudHSM

| | |
|---|---|
| **Port** | `10044` |
| **Protocol** | JSON RPC (`CloudHsmV2`) |
| **Endpoint** | `http://localhost:10044` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateCluster | Create a CloudHSM cluster |
| DescribeCluster | Describe a cluster |
| ListClusters | List all clusters |
| DeleteCluster | Delete a cluster |

### Usage with AWS CLI

```bash
# Create a cluster
aws cloudhsmv2 create-cluster \
  --hsm-type hsm1.medium \
  --subnet-ids subnet-1 \
  --endpoint-url http://localhost:10044 \
  --no-sign-request

# List clusters
aws cloudhsmv2 describe-clusters \
  --endpoint-url http://localhost:10044 \
  --no-sign-request
```

---

## RAM

| | |
|---|---|
| **Port** | `10045` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10045` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateResourceShare | Create a resource share |
| GetResourceShare | Get details of a resource share |
| ListResourceShares | List all resource shares |
| DeleteResourceShare | Delete a resource share |

### Usage with AWS CLI

```bash
# Create a resource share
aws ram create-resource-share \
  --name my-share \
  --endpoint-url http://localhost:10045 \
  --no-sign-request

# List resource shares
aws ram get-resource-shares \
  --resource-owner SELF \
  --endpoint-url http://localhost:10045 \
  --no-sign-request
```

---

## Directory Service

| | |
|---|---|
| **Port** | `10043` |
| **Protocol** | JSON RPC (`DirectoryService_20150416`) |
| **Endpoint** | `http://localhost:10043` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateDirectory | Create a directory |
| DescribeDirectory | Describe a directory |
| ListDirectories | List all directories |
| DeleteDirectory | Delete a directory |

### Usage with AWS CLI

```bash
# Create a directory
aws ds create-directory \
  --name corp.example.com \
  --password 'P@ssw0rd!' \
  --size Small \
  --endpoint-url http://localhost:10043 \
  --no-sign-request

# List directories
aws ds describe-directories \
  --endpoint-url http://localhost:10043 \
  --no-sign-request
```

---

## IAM Identity Center

| | |
|---|---|
| **Port** | `10049` |
| **Protocol** | JSON RPC (`SWBExternalService`) |
| **Endpoint** | `http://localhost:10049` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreatePermissionSet | Create a permission set |
| DescribePermissionSet | Describe a permission set |
| ListPermissionSets | List all permission sets |
| DeletePermissionSet | Delete a permission set |

### Usage with AWS CLI

```bash
# List permission sets
aws sso-admin list-permission-sets \
  --instance-arn arn:aws:sso:::instance/ssoins-1234567890abcdef0 \
  --endpoint-url http://localhost:10049 \
  --no-sign-request
```

---

## Firewall Manager

| | |
|---|---|
| **Port** | `10047` |
| **Protocol** | JSON RPC (`FMS_20180101`) |
| **Endpoint** | `http://localhost:10047` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreatePolicy | Create a Firewall Manager policy |
| DescribePolicy | Describe a policy |
| ListPolicys | List all policies |
| DeletePolicy | Delete a policy |

### Usage with AWS CLI

```bash
# List policies
aws fms list-policies \
  --endpoint-url http://localhost:10047 \
  --no-sign-request
```

---

## Detective

| | |
|---|---|
| **Port** | `10040` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10040` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateGraph | Create a behavior graph |
| GetGraph | Get details of a graph |
| ListGraphs | List all graphs |
| DeleteGraph | Delete a graph |

### Usage with AWS CLI

```bash
# Create a graph
aws detective create-graph \
  --endpoint-url http://localhost:10040 \
  --no-sign-request

# List graphs
aws detective list-graphs \
  --endpoint-url http://localhost:10040 \
  --no-sign-request
```
