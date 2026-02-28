# Business Applications

This category covers AWS business application services including customer engagement, collaboration, and email. All services store state in memory with no persistence.

---

## Connect

| | |
|---|---|
| **Port** | `10124` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10124` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateInstance | Create a Connect instance |
| GetInstance | Get details of a Connect instance |
| ListInstances | List all Connect instances |
| DeleteInstance | Delete a Connect instance |

### Usage with AWS CLI

```bash
# List instances
aws connect list-instances \
  --endpoint-url http://localhost:10124 \
  --no-sign-request
```

---

## Chime

| | |
|---|---|
| **Port** | `10123` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10123` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateAccount | Create a Chime account |
| GetAccount | Get details of a Chime account |
| ListAccounts | List all Chime accounts |
| DeleteAccount | Delete a Chime account |

### Usage with AWS CLI

```bash
# List accounts
aws chime list-accounts \
  --endpoint-url http://localhost:10123 \
  --no-sign-request
```

---

## WorkDocs

| | |
|---|---|
| **Port** | `10126` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10126` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateFolder | Create a WorkDocs folder |
| GetFolder | Get details of a WorkDocs folder |
| ListFolders | List all WorkDocs folders |
| DeleteFolder | Delete a WorkDocs folder |

### Usage with AWS CLI

```bash
# List folders
aws workdocs describe-folder-contents \
  --folder-id root \
  --endpoint-url http://localhost:10126 \
  --no-sign-request
```

---

## WorkMail

| | |
|---|---|
| **Port** | `10127` |
| **Protocol** | JSON RPC (`WorkMailService`) |
| **Endpoint** | `http://localhost:10127` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateOrganization | Create a WorkMail organization |
| DescribeOrganization | Describe a WorkMail organization |
| ListOrganizations | List all WorkMail organizations |
| DeleteOrganization | Delete a WorkMail organization |

### Usage with AWS CLI

```bash
# List organizations
aws workmail list-organizations \
  --endpoint-url http://localhost:10127 \
  --no-sign-request
```

---

## WorkSpaces

| | |
|---|---|
| **Port** | `10152` |
| **Protocol** | JSON RPC (`WorkspacesService`) |
| **Endpoint** | `http://localhost:10152` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateWorkspace | Create a WorkSpaces workspace |
| DescribeWorkspace | Describe a WorkSpaces workspace |
| ListWorkspaces | List all WorkSpaces workspaces |
| DeleteWorkspace | Delete a WorkSpaces workspace |

### Usage with AWS CLI

```bash
# List workspaces
aws workspaces describe-workspaces \
  --endpoint-url http://localhost:10152 \
  --no-sign-request
```

---

## Pinpoint

| | |
|---|---|
| **Port** | `10125` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10125` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateApp | Create a Pinpoint application |
| GetApp | Get details of a Pinpoint application |
| ListApps | List all Pinpoint applications |
| DeleteApp | Delete a Pinpoint application |

### Usage with AWS CLI

```bash
# List apps
aws pinpoint get-apps \
  --endpoint-url http://localhost:10125 \
  --no-sign-request
```

---

## SES

| | |
|---|---|
| **Port** | `9300` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:9300` |

### Supported Operations (5)

| Operation | Description |
|-----------|-------------|
| SendEmail | Send an email message (simulated, not actually delivered) |
| CreateEmailIdentity | Create and auto-verify an email identity |
| DeleteEmailIdentity | Delete an email identity |
| GetEmailIdentity | Get details of an email identity |
| ListEmailIdentities | List all email identities |

### Wire Protocol Details

SES uses REST JSON with versioned URL paths prefixed with `/v2/email/`. Email operations use `/v2/email/outbound-emails` and identity operations use `/v2/email/identities`. List endpoints support `PageSize` query parameter filtering.

- **Emails are not delivered**: `SendEmail` accepts the request and returns a message ID but does not deliver email.
- **All identities are auto-verified**: `CreateEmailIdentity` immediately marks the identity as verified without DNS or email confirmation.

### Usage with AWS CLI

```bash
# Create an email identity
aws sesv2 create-email-identity \
  --email-identity user@example.com \
  --endpoint-url http://localhost:9300 \
  --no-sign-request

# List email identities
aws sesv2 list-email-identities \
  --endpoint-url http://localhost:9300 \
  --no-sign-request

# Get an email identity
aws sesv2 get-email-identity \
  --email-identity user@example.com \
  --endpoint-url http://localhost:9300 \
  --no-sign-request

# Send an email
aws sesv2 send-email \
  --from-email-address user@example.com \
  --destination '{"ToAddresses":["recipient@example.com"]}' \
  --content '{"Simple":{"Subject":{"Data":"Test"},"Body":{"Text":{"Data":"Hello from SES"}}}}' \
  --endpoint-url http://localhost:9300 \
  --no-sign-request

# Delete an email identity
aws sesv2 delete-email-identity \
  --email-identity user@example.com \
  --endpoint-url http://localhost:9300 \
  --no-sign-request
```

### Limitations

- Emails are not delivered. `SendEmail` accepts requests but does not send email.
- All identities are auto-verified without DNS or email confirmation.
- No sending quotas or suppression list management.
