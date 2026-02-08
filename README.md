# aws-sqs-local

A local implementation of the Amazon Simple Queue Service (SQS) API, written in Rust.

## Wire Protocol

All API operations use the **AWS JSON 1.0** protocol over HTTP POST.

- **Content-Type**: `application/x-amz-json-1.0`
- **Action routing**: `X-Amz-Target: AmazonSQS.<ActionName>` header
- **Request/response body**: JSON
- **Endpoint**: `http://localhost:<port>/`

### Error Response Format

Errors are returned as JSON with an appropriate HTTP status code:

```json
{
  "__type": "com.amazonaws.sqs#QueueDoesNotExist",
  "message": "The specified queue does not exist."
}
```

| HTTP Status | Meaning |
|-------------|---------|
| 200 | Success |
| 400 | Client error (invalid parameters, missing fields) |
| 404 | Resource not found (queue does not exist) |
| 409 | Conflict (queue name already exists with different attributes, purge in progress) |
| 429 | Throttled |
| 500 | Internal server error |

---

## Queue Types

### Standard Queues

- **At-least-once delivery**: messages may be delivered more than once.
- **Best-effort ordering**: message order is not guaranteed.
- **Unlimited throughput**: no hard limit on API call rate.

### FIFO Queues

- **Exactly-once processing**: duplicate messages are eliminated within a 5-minute deduplication window.
- **Strict ordering**: messages within a message group are delivered in the order they were sent.
- **Queue name must end with `.fifo`**.
- Require `MessageGroupId` on every `SendMessage` call.
- Support optional `ContentBasedDeduplication` (SHA-256 of message body as dedup ID).

---

## Queue Attributes

These attributes are set at queue creation time via `CreateQueue` and can be modified via `SetQueueAttributes` (unless noted otherwise).

| Attribute | Type | Default | Range | Notes |
|-----------|------|---------|-------|-------|
| `VisibilityTimeout` | integer (seconds) | 30 | 0 -- 43200 | Time a received message is hidden from other consumers. |
| `MessageRetentionPeriod` | integer (seconds) | 345600 (4 days) | 60 -- 1209600 (14 days) | How long undeleted messages are retained. |
| `DelaySeconds` | integer (seconds) | 0 | 0 -- 900 | Default delivery delay for new messages. |
| `MaximumMessageSize` | integer (bytes) | 262144 (256 KB) | 1024 -- 262144 | Maximum allowed message body size. |
| `ReceiveMessageWaitTimeSeconds` | integer (seconds) | 0 | 0 -- 20 | Default long-poll wait time for `ReceiveMessage`. |
| `RedrivePolicy` | JSON string | none | -- | Dead-letter queue configuration (see below). |
| `RedriveAllowPolicy` | JSON string | none | -- | Controls which source queues can use this queue as a DLQ. |
| `FifoQueue` | boolean string | `"false"` | -- | **Immutable after creation.** |
| `ContentBasedDeduplication` | boolean string | `"false"` | -- | FIFO only. Uses SHA-256 of body as dedup ID. |
| `DeduplicationScope` | string | `"Queue"` | `Queue`, `MessageGroup` | FIFO only. Scope of deduplication ID uniqueness. |
| `FifoThroughputLimit` | string | `"PerQueue"` | `PerQueue`, `PerMessageGroupId` | FIFO only. High-throughput mode setting. |
| `SqsManagedSseEnabled` | boolean string | `"true"` | -- | Server-side encryption with SQS-managed keys. |
| `KmsMasterKeyId` | string | none | -- | KMS key ID for server-side encryption. |
| `KmsDataKeyReusePeriodSeconds` | integer (seconds) | 300 | 60 -- 86400 | How long to reuse a data encryption key. |

### Read-Only Attributes

Returned by `GetQueueAttributes` but not settable:

| Attribute | Type | Description |
|-----------|------|-------------|
| `QueueArn` | string | The ARN of the queue. |
| `ApproximateNumberOfMessages` | integer | Approximate count of visible messages. |
| `ApproximateNumberOfMessagesNotVisible` | integer | Approximate count of in-flight messages. |
| `ApproximateNumberOfMessagesDelayed` | integer | Approximate count of delayed messages. |
| `CreatedTimestamp` | integer (epoch seconds) | When the queue was created. |
| `LastModifiedTimestamp` | integer (epoch seconds) | When the queue attributes were last changed. |

### RedrivePolicy Format

```json
{
  "deadLetterTargetArn": "arn:aws:sqs:us-east-1:123456789012:my-dlq",
  "maxReceiveCount": 3
}
```

- `maxReceiveCount`: after a message has been received this many times without being deleted, it is moved to the dead-letter queue.
- The DLQ must be the same queue type as the source (standard-to-standard, FIFO-to-FIFO).

### RedriveAllowPolicy Format

```json
{
  "redrivePermission": "allowAll" | "denyAll" | "byQueue",
  "sourceQueueArns": ["arn:aws:sqs:us-east-1:123456789012:source-queue"]
}
```

---

## API Operations

### CreateQueue

Create a new SQS queue.

**Target**: `AmazonSQS.CreateQueue`

**Request**:
```json
{
  "QueueName": "my-queue",
  "Attributes": {
    "VisibilityTimeout": "30",
    "MessageRetentionPeriod": "345600"
  },
  "tags": {
    "Environment": "test"
  }
}
```

**Response** (200):
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `QueueName` | string | yes | 1--80 characters. Alphanumeric, hyphens, underscores only. FIFO queues must end with `.fifo`. Case-sensitive. |
| `Attributes` | map\<string, string\> | no | Queue attributes (see table above). All values are strings. |
| `tags` | map\<string, string\> | no | Tags to apply to the queue. |

**Behavior**:
- If a queue with the same name already exists **and** all provided attributes match the existing queue, the existing queue URL is returned (idempotent).
- If a queue with the same name exists but attributes differ, return `QueueAlreadyExists` (409).
- FIFO queue names must end with `.fifo`. Standard queue names must not.

**Errors**:
- `QueueAlreadyExists` -- name taken with different attributes.
- `InvalidAttributeName` -- unrecognized attribute key.
- `InvalidAttributeValue` -- attribute value out of range or wrong type.

---

### DeleteQueue

Delete a queue and all its messages.

**Target**: `AmazonSQS.DeleteQueue`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue"
}
```

**Response** (200): empty JSON `{}`

**Behavior**:
- All messages in the queue are permanently deleted.
- The queue name cannot be reused for 60 seconds after deletion.
- Succeeds silently if the queue does not exist (idempotent).

**Errors**:
- `QueueDoesNotExist` (if strict mode).

---

### GetQueueUrl

Look up a queue URL by name.

**Target**: `AmazonSQS.GetQueueUrl`

**Request**:
```json
{
  "QueueName": "my-queue",
  "QueueOwnerAWSAccountId": "123456789012"
}
```

**Response** (200):
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `QueueName` | string | yes | Name of the queue. |
| `QueueOwnerAWSAccountId` | string | no | Account ID of the queue owner (for cross-account). |

**Errors**:
- `QueueDoesNotExist` -- no queue with that name.

---

### ListQueues

List queues, optionally filtered by a name prefix.

**Target**: `AmazonSQS.ListQueues`

**Request**:
```json
{
  "QueueNamePrefix": "prod-",
  "MaxResults": 100,
  "NextToken": null
}
```

**Response** (200):
```json
{
  "QueueUrls": [
    "http://localhost:9324/123456789012/prod-orders",
    "http://localhost:9324/123456789012/prod-events"
  ],
  "NextToken": null
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `QueueNamePrefix` | string | no | Return only queues whose name starts with this prefix. |
| `MaxResults` | integer | no | 1--1000. Default 1000. |
| `NextToken` | string | no | Pagination token from a previous response. |

---

### GetQueueAttributes

Retrieve attributes of a queue.

**Target**: `AmazonSQS.GetQueueAttributes`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue",
  "AttributeNames": ["All"]
}
```

**Response** (200):
```json
{
  "Attributes": {
    "QueueArn": "arn:aws:sqs:us-east-1:123456789012:my-queue",
    "VisibilityTimeout": "30",
    "MessageRetentionPeriod": "345600",
    "ApproximateNumberOfMessages": "5",
    "ApproximateNumberOfMessagesNotVisible": "2",
    "ApproximateNumberOfMessagesDelayed": "0",
    "CreatedTimestamp": "1700000000",
    "LastModifiedTimestamp": "1700000000"
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `QueueUrl` | string | yes | URL of the queue. |
| `AttributeNames` | list\<string\> | no | Specific attribute names, or `["All"]` for everything. |

**Behavior**:
- `ApproximateNumberOfMessages*` counts are eventually consistent and may lag slightly behind the actual state.
- All attribute values are returned as strings.

**Errors**:
- `QueueDoesNotExist`
- `InvalidAttributeName` -- unrecognized attribute name in the list.

---

### SetQueueAttributes

Modify attributes of an existing queue.

**Target**: `AmazonSQS.SetQueueAttributes`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue",
  "Attributes": {
    "VisibilityTimeout": "60",
    "RedrivePolicy": "{\"deadLetterTargetArn\":\"arn:aws:sqs:us-east-1:123456789012:my-dlq\",\"maxReceiveCount\":\"5\"}"
  }
}
```

**Response** (200): empty JSON `{}`

**Behavior**:
- Changes take effect immediately.
- `FifoQueue` cannot be changed after creation.
- Updates `LastModifiedTimestamp`.

**Errors**:
- `QueueDoesNotExist`
- `InvalidAttributeName`
- `InvalidAttributeValue`

---

### PurgeQueue

Delete all messages in a queue without deleting the queue itself.

**Target**: `AmazonSQS.PurgeQueue`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue"
}
```

**Response** (200): empty JSON `{}`

**Behavior**:
- Deletes all messages (visible, in-flight, and delayed).
- Cannot be called again on the same queue within 60 seconds.

**Errors**:
- `QueueDoesNotExist`
- `PurgeQueueInProgress` -- a purge was already initiated within the last 60 seconds.

---

### SendMessage

Send a single message to a queue.

**Target**: `AmazonSQS.SendMessage`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue",
  "MessageBody": "Hello, world!",
  "DelaySeconds": 0,
  "MessageAttributes": {
    "MyAttribute": {
      "DataType": "String",
      "StringValue": "my-value"
    }
  },
  "MessageSystemAttributes": {
    "AWSTraceHeader": {
      "DataType": "String",
      "StringValue": "Root=1-abc-def"
    }
  },
  "MessageDeduplicationId": "dedup-123",
  "MessageGroupId": "group-1"
}
```

**Response** (200):
```json
{
  "MessageId": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "MD5OfMessageBody": "65a8e27d8879283831b664bd8b7f0ad4",
  "MD5OfMessageAttributes": "abc123def456",
  "MD5OfMessageSystemAttributes": "789ghi012",
  "SequenceNumber": "10000000000000001000"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `QueueUrl` | string | yes | URL of the target queue. |
| `MessageBody` | string | yes | 1 byte to `MaximumMessageSize` bytes (default 256 KB). |
| `DelaySeconds` | integer | no | 0--900. Overrides queue default. |
| `MessageAttributes` | map | no | Custom key-value attributes (see Message Attributes below). |
| `MessageSystemAttributes` | map | no | Only `AWSTraceHeader` is supported. |
| `MessageDeduplicationId` | string | no | Required for FIFO queues without `ContentBasedDeduplication`. Max 128 characters. |
| `MessageGroupId` | string | no | **Required** for FIFO queues. Messages in the same group are delivered in order. Max 128 characters. |

**Behavior**:
- The message is stored in the queue until explicitly deleted or the retention period expires.
- `DelaySeconds` makes the message invisible to consumers for the specified duration.
- `SequenceNumber` is only returned for FIFO queues.
- For FIFO queues with `ContentBasedDeduplication` enabled, if `MessageDeduplicationId` is omitted, the SHA-256 hash of the message body is used.
- Deduplication window: 5 minutes. Sending a message with the same dedup ID within this window returns success but does not create a duplicate.

**Errors**:
- `QueueDoesNotExist`
- `InvalidParameterValue` -- body too large, delay out of range, etc.
- `InvalidMessageContents` -- message body contains invalid characters.
- `UnsupportedOperation` -- e.g., sending to FIFO queue without `MessageGroupId`.

---

### SendMessageBatch

Send up to 10 messages in a single request.

**Target**: `AmazonSQS.SendMessageBatch`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue",
  "Entries": [
    {
      "Id": "msg-1",
      "MessageBody": "First message",
      "DelaySeconds": 0
    },
    {
      "Id": "msg-2",
      "MessageBody": "Second message",
      "DelaySeconds": 5
    }
  ]
}
```

**Response** (200):
```json
{
  "Successful": [
    {
      "Id": "msg-1",
      "MessageId": "a1b2c3d4-...",
      "MD5OfMessageBody": "...",
      "SequenceNumber": "..."
    }
  ],
  "Failed": [
    {
      "Id": "msg-2",
      "Code": "InvalidParameterValue",
      "Message": "Value for parameter DelaySeconds is invalid.",
      "SenderFault": true
    }
  ]
}
```

Each entry in `Entries` accepts the same fields as `SendMessage` plus:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `Id` | string | yes | Caller-chosen identifier for this entry. Must be unique within the batch. Alphanumeric, hyphens, underscores. |

**Behavior**:
- Maximum 10 entries per request.
- Total payload of all entries must not exceed 262144 bytes (256 KB).
- Partial success is possible: check both `Successful` and `Failed` lists.
- Each entry is processed independently.

**Errors** (request-level):
- `EmptyBatchRequest` -- no entries.
- `TooManyEntriesInBatchRequest` -- more than 10 entries.
- `BatchEntryIdsNotDistinct` -- duplicate `Id` values.
- `BatchRequestTooLong` -- total payload exceeds 256 KB.
- `InvalidBatchEntryId` -- `Id` contains invalid characters.

---

### ReceiveMessage

Retrieve one or more messages from a queue.

**Target**: `AmazonSQS.ReceiveMessage`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue",
  "MaxNumberOfMessages": 10,
  "VisibilityTimeout": 30,
  "WaitTimeSeconds": 20,
  "AttributeNames": ["All"],
  "MessageAttributeNames": ["All"],
  "MessageSystemAttributeNames": ["All"],
  "ReceiveRequestAttemptId": "attempt-1"
}
```

**Response** (200):
```json
{
  "Messages": [
    {
      "MessageId": "a1b2c3d4-...",
      "ReceiptHandle": "AQEB...long-opaque-string...",
      "Body": "Hello, world!",
      "MD5OfBody": "65a8e27d8879283831b664bd8b7f0ad4",
      "MD5OfMessageAttributes": "abc123def456",
      "Attributes": {
        "SenderId": "123456789012",
        "SentTimestamp": "1700000000000",
        "ApproximateReceiveCount": "1",
        "ApproximateFirstReceiveTimestamp": "1700000001000",
        "MessageDeduplicationId": "dedup-123",
        "MessageGroupId": "group-1",
        "SequenceNumber": "10000000000000001000",
        "AWSTraceHeader": "Root=1-abc-def"
      },
      "MessageAttributes": {
        "MyAttribute": {
          "DataType": "String",
          "StringValue": "my-value"
        }
      }
    }
  ]
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `QueueUrl` | string | yes | URL of the queue. |
| `MaxNumberOfMessages` | integer | no | 1--10. Default 1. |
| `VisibilityTimeout` | integer | no | 0--43200. Overrides queue default for this receive. |
| `WaitTimeSeconds` | integer | no | 0--20. Enables long polling. 0 = short poll (return immediately). |
| `AttributeNames` | list\<string\> | no | System attributes to return. `All` or specific names. |
| `MessageAttributeNames` | list\<string\> | no | Custom attributes to return. `All`, specific names, or prefix patterns like `foo.*`. |
| `MessageSystemAttributeNames` | list\<string\> | no | System attributes to return (`AWSTraceHeader`). |
| `ReceiveRequestAttemptId` | string | no | FIFO only. Deduplication token for the receive request itself. Max 128 characters. |

**Message System Attributes** (returned in `Attributes`):

| Attribute | Type | Description |
|-----------|------|-------------|
| `SenderId` | string | Account ID of the sender. |
| `SentTimestamp` | string (epoch ms) | When the message was sent. |
| `ApproximateReceiveCount` | string (integer) | How many times this message has been received. |
| `ApproximateFirstReceiveTimestamp` | string (epoch ms) | When the message was first received. |
| `MessageDeduplicationId` | string | FIFO only. The deduplication ID. |
| `MessageGroupId` | string | FIFO only. The message group. |
| `SequenceNumber` | string | FIFO only. Sequence number within the group. |
| `AWSTraceHeader` | string | X-Ray trace header, if set by the sender. |

**Behavior**:
- Received messages become invisible for `VisibilityTimeout` seconds. During this time, no other consumer can receive them.
- After the visibility timeout expires without the message being deleted, it becomes visible again and can be received by another consumer.
- **Short polling** (`WaitTimeSeconds = 0`): returns immediately, may return 0 messages even if messages exist (samples a subset of servers).
- **Long polling** (`WaitTimeSeconds > 0`): waits up to the specified duration for messages to arrive before returning. Reduces empty responses and API calls.
- An empty `Messages` list does **not** mean the queue is empty.
- `ReceiptHandle` is required for subsequent `DeleteMessage` and `ChangeMessageVisibility` calls. It is only valid for the duration of the visibility timeout.
- FIFO queues deliver messages in order within each `MessageGroupId`. Only one inflight message per message group is delivered at a time (unless high-throughput mode is enabled).
- `ReceiveRequestAttemptId` (FIFO): if the same ID is used within a 5-minute window, the same set of messages is returned (idempotent receive).

**Errors**:
- `QueueDoesNotExist`
- `OverLimit` -- too many inflight messages (120,000 for standard, 20,000 for FIFO).
- `InvalidParameterValue`

---

### DeleteMessage

Delete a message from the queue after successful processing.

**Target**: `AmazonSQS.DeleteMessage`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue",
  "ReceiptHandle": "AQEB...long-opaque-string..."
}
```

**Response** (200): empty JSON `{}`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `QueueUrl` | string | yes | URL of the queue. |
| `ReceiptHandle` | string | yes | Receipt handle from `ReceiveMessage`. |

**Behavior**:
- The receipt handle must be from the most recent receive of the message. If the visibility timeout has expired and the message was received again, the old receipt handle is invalid.
- Deleting an already-deleted message succeeds silently (idempotent).
- After deletion the message is permanently removed and cannot be received again.

**Errors**:
- `QueueDoesNotExist`
- `ReceiptHandleIsInvalid` -- handle is malformed, expired, or from a different queue.
- `InvalidIdFormat`

---

### DeleteMessageBatch

Delete up to 10 messages in a single request.

**Target**: `AmazonSQS.DeleteMessageBatch`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue",
  "Entries": [
    { "Id": "del-1", "ReceiptHandle": "AQEB..." },
    { "Id": "del-2", "ReceiptHandle": "AQEB..." }
  ]
}
```

**Response** (200):
```json
{
  "Successful": [{ "Id": "del-1" }],
  "Failed": [
    {
      "Id": "del-2",
      "Code": "ReceiptHandleIsInvalid",
      "Message": "The receipt handle is not valid.",
      "SenderFault": true
    }
  ]
}
```

**Behavior**: same as `DeleteMessage`, applied per-entry. Partial success is possible. Maximum 10 entries.

**Errors** (request-level):
- `EmptyBatchRequest`
- `TooManyEntriesInBatchRequest`
- `BatchEntryIdsNotDistinct`
- `InvalidBatchEntryId`

---

### ChangeMessageVisibility

Extend or shorten the visibility timeout of a message currently in-flight.

**Target**: `AmazonSQS.ChangeMessageVisibility`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue",
  "ReceiptHandle": "AQEB...",
  "VisibilityTimeout": 60
}
```

**Response** (200): empty JSON `{}`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `QueueUrl` | string | yes | URL of the queue. |
| `ReceiptHandle` | string | yes | Receipt handle from `ReceiveMessage`. |
| `VisibilityTimeout` | integer | yes | New timeout in seconds. 0--43200. |

**Behavior**:
- Setting `VisibilityTimeout` to `0` makes the message immediately visible to other consumers.
- Can be used to extend processing time before the current visibility timeout expires.
- The new timeout is counted from the time of the `ChangeMessageVisibility` call, not from the original receive time.
- The receipt handle must be current (from the most recent receive).

**Errors**:
- `QueueDoesNotExist`
- `MessageNotInflight` -- the message is not currently invisible (already visible or deleted).
- `ReceiptHandleIsInvalid`
- `InvalidParameterValue`

---

### ChangeMessageVisibilityBatch

Change visibility timeout for up to 10 messages.

**Target**: `AmazonSQS.ChangeMessageVisibilityBatch`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue",
  "Entries": [
    { "Id": "vis-1", "ReceiptHandle": "AQEB...", "VisibilityTimeout": 60 },
    { "Id": "vis-2", "ReceiptHandle": "AQEB...", "VisibilityTimeout": 0 }
  ]
}
```

**Response** (200):
```json
{
  "Successful": [{ "Id": "vis-1" }],
  "Failed": [
    {
      "Id": "vis-2",
      "Code": "MessageNotInflight",
      "Message": "The message is not in flight.",
      "SenderFault": true
    }
  ]
}
```

**Behavior**: same as `ChangeMessageVisibility`, applied per-entry. Maximum 10 entries.

**Errors** (request-level):
- `EmptyBatchRequest`
- `TooManyEntriesInBatchRequest`
- `BatchEntryIdsNotDistinct`
- `InvalidBatchEntryId`

---

### TagQueue

Add or update tags on a queue.

**Target**: `AmazonSQS.TagQueue`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue",
  "Tags": {
    "Environment": "production",
    "Team": "backend"
  }
}
```

**Response** (200): empty JSON `{}`

**Behavior**:
- Maximum 50 tags per queue.
- Tag keys: 1--128 Unicode characters. Case-sensitive.
- Tag values: 0--256 Unicode characters.
- If a tag key already exists, its value is overwritten.

**Errors**:
- `QueueDoesNotExist`
- `InvalidParameterValue`

---

### UntagQueue

Remove tags from a queue.

**Target**: `AmazonSQS.UntagQueue`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue",
  "TagKeys": ["Environment", "Team"]
}
```

**Response** (200): empty JSON `{}`

**Behavior**:
- Removing a tag key that does not exist succeeds silently.

**Errors**:
- `QueueDoesNotExist`

---

### ListQueueTags

List all tags on a queue.

**Target**: `AmazonSQS.ListQueueTags`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue"
}
```

**Response** (200):
```json
{
  "Tags": {
    "Environment": "production",
    "Team": "backend"
  }
}
```

**Errors**:
- `QueueDoesNotExist`

---

### AddPermission

Add a permission to the queue's access policy.

**Target**: `AmazonSQS.AddPermission`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue",
  "Label": "allow-send",
  "AWSAccountIds": ["111122223333"],
  "Actions": ["SendMessage", "ReceiveMessage"]
}
```

**Response** (200): empty JSON `{}`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `QueueUrl` | string | yes | URL of the queue. |
| `Label` | string | yes | Unique identifier for this permission statement. Max 80 characters. |
| `AWSAccountIds` | list\<string\> | yes | AWS account IDs to grant access. |
| `Actions` | list\<string\> | yes | SQS actions to allow. Valid values: `SendMessage`, `ReceiveMessage`, `DeleteMessage`, `ChangeMessageVisibility`, `GetQueueAttributes`, `GetQueueUrl`. |

**Behavior**:
- Labels must be unique per queue. Adding the same label again is an error.
- Maximum 7 permission statements (labels) per queue policy.

**Errors**:
- `QueueDoesNotExist`
- `OverLimit` -- too many permissions.
- `InvalidParameterValue`

---

### RemovePermission

Remove a permission from the queue's access policy.

**Target**: `AmazonSQS.RemovePermission`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-queue",
  "Label": "allow-send"
}
```

**Response** (200): empty JSON `{}`

**Errors**:
- `QueueDoesNotExist`
- `InvalidParameterValue` -- label not found.

---

### ListDeadLetterSourceQueues

List all queues that have this queue configured as their dead-letter queue.

**Target**: `AmazonSQS.ListDeadLetterSourceQueues`

**Request**:
```json
{
  "QueueUrl": "http://localhost:9324/123456789012/my-dlq",
  "MaxResults": 100,
  "NextToken": null
}
```

**Response** (200):
```json
{
  "queueUrls": [
    "http://localhost:9324/123456789012/source-queue-1",
    "http://localhost:9324/123456789012/source-queue-2"
  ],
  "NextToken": null
}
```

**Errors**:
- `QueueDoesNotExist`

---

### StartMessageMoveTask

Start an asynchronous task to move messages from one queue to another.

**Target**: `AmazonSQS.StartMessageMoveTask`

**Request**:
```json
{
  "SourceArn": "arn:aws:sqs:us-east-1:123456789012:my-dlq",
  "DestinationArn": "arn:aws:sqs:us-east-1:123456789012:my-queue",
  "MaxNumberOfMessagesPerSecond": 50
}
```

**Response** (200):
```json
{
  "TaskHandle": "task-handle-string"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `SourceArn` | string | yes | ARN of the source queue. |
| `DestinationArn` | string | no | ARN of the destination queue. If omitted, messages are moved back to their original source queues. |
| `MaxNumberOfMessagesPerSecond` | integer | no | Throttle for the move rate. |

**Behavior**:
- Typically used to redrive messages from a DLQ back to the source queue.
- Cannot move messages between standard and FIFO queues.
- Only one active move task per source queue at a time.

**Errors**:
- `ResourceNotFoundException` -- source or destination queue does not exist.
- `UnsupportedOperation` -- cross-type move (standard to FIFO or vice versa).
- `InvalidParameterValue`

---

### CancelMessageMoveTask

Cancel an in-progress message move task.

**Target**: `AmazonSQS.CancelMessageMoveTask`

**Request**:
```json
{
  "TaskHandle": "task-handle-string"
}
```

**Response** (200):
```json
{
  "ApproximateNumberOfMessagesMoved": 42
}
```

**Errors**:
- `ResourceNotFoundException` -- task not found or already completed.
- `InvalidParameterValue`

---

### ListMessageMoveTasks

List message move tasks for a source queue.

**Target**: `AmazonSQS.ListMessageMoveTasks`

**Request**:
```json
{
  "SourceArn": "arn:aws:sqs:us-east-1:123456789012:my-dlq",
  "MaxResults": 10,
  "NextToken": null
}
```

**Response** (200):
```json
{
  "Results": [
    {
      "TaskHandle": "task-handle-string",
      "Status": "RUNNING",
      "SourceArn": "arn:aws:sqs:us-east-1:123456789012:my-dlq",
      "DestinationArn": "arn:aws:sqs:us-east-1:123456789012:my-queue",
      "ApproximateNumberOfMessagesMoved": 42,
      "ApproximateNumberOfMessagesToMove": 100,
      "MaxNumberOfMessagesPerSecond": 50,
      "StartedTimestamp": 1700000000000
    }
  ],
  "NextToken": null
}
```

**Task statuses**: `RUNNING`, `COMPLETED`, `CANCELLING`, `CANCELLED`, `FAILED`.

**Errors**:
- `ResourceNotFoundException`

---

## Message Attributes

Custom message attributes sent via `SendMessage` / `SendMessageBatch` and returned by `ReceiveMessage`.

```json
{
  "AttributeName": {
    "DataType": "String",
    "StringValue": "attribute-value"
  }
}
```

| DataType | Value Field | Description |
|----------|-------------|-------------|
| `String` | `StringValue` | UTF-8 string. |
| `Number` | `StringValue` | Numeric value as a string (integer or float). |
| `Binary` | `BinaryValue` | Base64-encoded binary data. |
| `String.custom` | `StringValue` | Custom subtype for application use. |
| `Number.custom` | `StringValue` | Custom subtype for application use. |
| `Binary.custom` | `BinaryValue` | Custom subtype for application use. |

**Constraints**:
- Maximum 10 message attributes per message.
- Attribute name: up to 256 characters. Alphanumeric, hyphens, underscores, periods.
- Attribute names are case-sensitive.
- Total size of all attributes (names + values + data types) counts toward the 256 KB message size limit.

---

## Key Behavioral Details

### Visibility Timeout

When a consumer receives a message, it becomes invisible to other consumers for the duration of the visibility timeout. This prevents multiple consumers from processing the same message simultaneously.

- Default: 30 seconds (configurable per-queue via `VisibilityTimeout` attribute).
- Can be overridden per-receive via the `VisibilityTimeout` parameter on `ReceiveMessage`.
- Can be extended or shortened via `ChangeMessageVisibility` while the message is in-flight.
- If the consumer does not delete the message before the timeout expires, the message becomes visible again and `ApproximateReceiveCount` is incremented.

### Message Lifecycle

1. **Producer** sends a message via `SendMessage`. If `DelaySeconds > 0`, the message enters a delayed state.
2. After the delay period (if any), the message becomes **visible** and available for consumers.
3. **Consumer** calls `ReceiveMessage`. The message becomes **in-flight** (invisible) for `VisibilityTimeout` seconds.
4. Consumer processes the message and calls `DeleteMessage` with the receipt handle.
5. If the consumer fails to delete in time, the message returns to **visible** state and can be received again.
6. If a `RedrivePolicy` is configured and `ApproximateReceiveCount` exceeds `maxReceiveCount`, the message is moved to the dead-letter queue instead of returning to visible state.
7. Messages that are neither deleted nor moved to a DLQ are permanently deleted after `MessageRetentionPeriod` expires.

### Dead-Letter Queues

- A DLQ is a regular queue designated to receive messages that fail processing.
- Configured via the `RedrivePolicy` attribute on the **source** queue.
- The DLQ must be the same type as the source queue (standard-to-standard, FIFO-to-FIFO).
- `RedriveAllowPolicy` on the DLQ controls which source queues may send messages to it.
- Messages moved to the DLQ retain their original message ID and attributes, but get a new receipt handle.
- The `ApproximateReceiveCount` carries over to the DLQ.

### FIFO Queue Semantics

- **Ordering**: messages within the same `MessageGroupId` are strictly ordered. Different message groups are independent.
- **Deduplication**: if `ContentBasedDeduplication` is enabled, the SHA-256 hash of the body is the dedup ID. Otherwise, `MessageDeduplicationId` must be provided. Duplicate messages (same dedup ID within 5 minutes) are accepted but not stored again.
- **Exactly-once processing**: the combination of deduplication and in-order delivery ensures each message is processed exactly once.
- **Message groups**: only one inflight message per message group at a time (in standard throughput mode). A group is "locked" until the inflight message is deleted or its visibility timeout expires.
- **High-throughput mode**: when `FifoThroughputLimit` is `PerMessageGroupId` and `DeduplicationScope` is `MessageGroup`, multiple message groups can be processed in parallel with higher throughput limits.
- **ReceiveRequestAttemptId**: provides idempotent receives. If a receive call fails and is retried with the same attempt ID within 5 minutes, the same messages are returned.

### Long Polling vs Short Polling

- **Short polling** (`WaitTimeSeconds = 0`): the server responds immediately. May return an empty response even if messages exist because it only queries a subset of servers.
- **Long polling** (`WaitTimeSeconds > 0`, max 20): the server waits up to the specified time for at least one message to arrive before responding. Reduces empty responses and API call costs.
- The queue-level `ReceiveMessageWaitTimeSeconds` attribute sets the default; the per-call `WaitTimeSeconds` parameter overrides it.

### Inflight Message Limits

- **Standard queues**: maximum 120,000 inflight messages per queue.
- **FIFO queues**: maximum 20,000 inflight messages per queue.
- Exceeding these limits causes `ReceiveMessage` to return an `OverLimit` error.
- An inflight message is one that has been received but not yet deleted or whose visibility timeout has not yet expired.
