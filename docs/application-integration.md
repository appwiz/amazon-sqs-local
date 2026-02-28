# Application Integration

This category covers AWS application integration services including messaging, event buses, workflow orchestration, and API management. All services store state in memory with no persistence.

---

## SNS

| | |
|---|---|
| **Port** | `9911` |
| **Protocol** | Query/XML (`Action` parameter) |
| **Endpoint** | `http://localhost:9911` |

### Supported Operations (17)

| Operation | Description |
|-----------|-------------|
| CreateTopic | Create a new SNS topic (standard or FIFO) |
| DeleteTopic | Delete an SNS topic and all its subscriptions |
| ListTopics | List all topics with optional pagination |
| GetTopicAttributes | Get all attributes of a topic |
| SetTopicAttributes | Set a single attribute on a topic (e.g., DisplayName, Policy) |
| Subscribe | Subscribe an endpoint to a topic (auto-confirmed) |
| Unsubscribe | Remove a subscription |
| ConfirmSubscription | Confirm a pending subscription (always succeeds) |
| ListSubscriptions | List all subscriptions across all topics |
| ListSubscriptionsByTopic | List subscriptions filtered by topic ARN |
| GetSubscriptionAttributes | Get all attributes of a subscription |
| SetSubscriptionAttributes | Set a single attribute on a subscription |
| Publish | Publish a message to a topic or target ARN |
| PublishBatch | Publish up to 10 messages in a single batch request |
| TagResource | Add tags to a topic |
| UntagResource | Remove tags from a topic |
| ListTagsForResource | List all tags on a topic |

### Wire Protocol Details

SNS uses the AWS Query protocol over HTTP POST with form-urlencoded bodies. The `Action` parameter determines the operation. Responses are XML documents in the `http://sns.amazonaws.com/doc/2010-03-31/` namespace.

- **FIFO topics**: create a topic with a name ending in `.fifo` and set the `FifoTopic` attribute to `true`. Publish calls require `MessageGroupId`; the service generates `SequenceNumber` values.
- **Subscriptions are auto-confirmed**: the service skips endpoint verification and immediately marks subscriptions as confirmed.
- **Message delivery is simulated**: `Publish` and `PublishBatch` accept messages and assign IDs but do not actually deliver to endpoints.

### Usage with AWS CLI

```bash
# Create a topic
aws sns create-topic \
  --name my-topic \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# Create a FIFO topic
aws sns create-topic \
  --name my-topic.fifo \
  --attributes FifoTopic=true,ContentBasedDeduplication=true \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# List topics
aws sns list-topics \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# Get topic attributes
aws sns get-topic-attributes \
  --topic-arn arn:aws:sns:us-east-1:000000000000:my-topic \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# Set topic attributes
aws sns set-topic-attributes \
  --topic-arn arn:aws:sns:us-east-1:000000000000:my-topic \
  --attribute-name DisplayName \
  --attribute-value "My Topic" \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# Subscribe an endpoint
aws sns subscribe \
  --topic-arn arn:aws:sns:us-east-1:000000000000:my-topic \
  --protocol email \
  --notification-endpoint user@example.com \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# List subscriptions
aws sns list-subscriptions \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# Publish a message
aws sns publish \
  --topic-arn arn:aws:sns:us-east-1:000000000000:my-topic \
  --message "Hello from SNS" \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# Unsubscribe
aws sns unsubscribe \
  --subscription-arn arn:aws:sns:us-east-1:000000000000:my-topic:sub-id \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# Tag a topic
aws sns tag-resource \
  --resource-arn arn:aws:sns:us-east-1:000000000000:my-topic \
  --tags Key=env,Value=test \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# List tags
aws sns list-tags-for-resource \
  --resource-arn arn:aws:sns:us-east-1:000000000000:my-topic \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# Delete a topic
aws sns delete-topic \
  --topic-arn arn:aws:sns:us-east-1:000000000000:my-topic \
  --endpoint-url http://localhost:9911 \
  --no-sign-request
```

### Limitations

- Subscriptions are auto-confirmed without endpoint verification.
- Messages are accepted and assigned IDs but not actually delivered to endpoints.
- No message filtering policies are evaluated.

---

## SQS

| | |
|---|---|
| **Port** | `9324` |
| **Protocol** | JSON RPC (`AmazonSQS`) |
| **Endpoint** | `http://localhost:9324` |

### Supported Operations (23)

| Operation | Description |
|-----------|-------------|
| CreateQueue | Create a standard or FIFO queue with configurable attributes |
| DeleteQueue | Delete a queue and all its messages |
| GetQueueUrl | Get the URL of a queue by name |
| ListQueues | List all queues, optionally filtered by prefix |
| GetQueueAttributes | Get queue attributes (e.g., message counts, policy) |
| SetQueueAttributes | Update queue attributes |
| PurgeQueue | Remove all messages from a queue |
| SendMessage | Send a message to a queue with optional delay |
| SendMessageBatch | Send up to 10 messages in a single request |
| ReceiveMessage | Receive one or more messages with visibility timeout |
| DeleteMessage | Delete a received message using its receipt handle |
| DeleteMessageBatch | Delete up to 10 messages in a single request |
| ChangeMessageVisibility | Change the visibility timeout of a received message |
| ChangeMessageVisibilityBatch | Change visibility for up to 10 messages |
| TagQueue | Add tags to a queue |
| UntagQueue | Remove tags from a queue |
| ListQueueTags | List all tags on a queue |
| AddPermission | Add a permission to the queue policy |
| RemovePermission | Remove a permission from the queue policy |
| ListDeadLetterSourceQueues | List queues that have this queue as their DLQ |
| StartMessageMoveTask | Start moving messages from a DLQ back to source |
| CancelMessageMoveTask | Cancel an in-progress message move task |
| ListMessageMoveTasks | List message move tasks for a queue |

### Wire Protocol Details

SQS uses JSON RPC over HTTP POST. The `X-Amz-Target` header must be set to `AmazonSQS.<Action>` (e.g., `AmazonSQS.CreateQueue`). Request and response bodies are JSON.

- **FIFO queues**: create a queue with a name ending in `.fifo` and set `FifoQueue` to `true`. Messages require `MessageGroupId` and support `MessageDeduplicationId`.
- **Visibility timeout**: received messages are hidden for the configured visibility timeout. Use `ChangeMessageVisibility` to extend or shorten the timeout.
- **Dead-letter queues**: configure `RedrivePolicy` with `deadLetterTargetArn` and `maxReceiveCount`. Messages exceeding the receive count are moved to the DLQ.
- **Delay queues**: set `DelaySeconds` on the queue or per-message to defer delivery.
- **Permissions are stored but not enforced**: `AddPermission` and `RemovePermission` update the queue policy, but no access checks are performed.

### Usage with AWS CLI

```bash
# Create a standard queue
aws sqs create-queue \
  --queue-name my-queue \
  --endpoint-url http://localhost:9324 \
  --no-sign-request

# Create a FIFO queue
aws sqs create-queue \
  --queue-name my-queue.fifo \
  --attributes FifoQueue=true,ContentBasedDeduplication=true \
  --endpoint-url http://localhost:9324 \
  --no-sign-request

# Get queue URL
aws sqs get-queue-url \
  --queue-name my-queue \
  --endpoint-url http://localhost:9324 \
  --no-sign-request

# List queues
aws sqs list-queues \
  --endpoint-url http://localhost:9324 \
  --no-sign-request

# Send a message
aws sqs send-message \
  --queue-url http://localhost:9324/000000000000/my-queue \
  --message-body "Hello from SQS" \
  --endpoint-url http://localhost:9324 \
  --no-sign-request

# Receive messages
aws sqs receive-message \
  --queue-url http://localhost:9324/000000000000/my-queue \
  --max-number-of-messages 10 \
  --endpoint-url http://localhost:9324 \
  --no-sign-request

# Delete a message
aws sqs delete-message \
  --queue-url http://localhost:9324/000000000000/my-queue \
  --receipt-handle <receipt-handle> \
  --endpoint-url http://localhost:9324 \
  --no-sign-request

# Purge a queue
aws sqs purge-queue \
  --queue-url http://localhost:9324/000000000000/my-queue \
  --endpoint-url http://localhost:9324 \
  --no-sign-request

# Get queue attributes
aws sqs get-queue-attributes \
  --queue-url http://localhost:9324/000000000000/my-queue \
  --attribute-names All \
  --endpoint-url http://localhost:9324 \
  --no-sign-request

# Tag a queue
aws sqs tag-queue \
  --queue-url http://localhost:9324/000000000000/my-queue \
  --tags env=test \
  --endpoint-url http://localhost:9324 \
  --no-sign-request

# Delete a queue
aws sqs delete-queue \
  --queue-url http://localhost:9324/000000000000/my-queue \
  --endpoint-url http://localhost:9324 \
  --no-sign-request
```

### Limitations

- Permissions are stored but not enforced.
- No message attribute filtering on receive.
- Message move tasks are tracked but messages are moved immediately.

---

## EventBridge

| | |
|---|---|
| **Port** | `9195` |
| **Protocol** | JSON RPC (`AmazonEventBridge`) |
| **Endpoint** | `http://localhost:9195` |

### Supported Operations (15)

| Operation | Description |
|-----------|-------------|
| CreateEventBus | Create a custom event bus |
| DeleteEventBus | Delete a custom event bus (default bus cannot be deleted) |
| DescribeEventBus | Describe an event bus by name |
| ListEventBuses | List all event buses |
| PutEvents | Send custom events to an event bus |
| PutRule | Create or update a rule on an event bus |
| DeleteRule | Delete a rule from an event bus |
| DescribeRule | Describe a rule by name |
| ListRules | List all rules, optionally filtered by event bus |
| PutTargets | Add targets to a rule |
| RemoveTargets | Remove targets from a rule |
| ListTargetsByRule | List all targets for a rule |
| TagResource | Add tags to an EventBridge resource |
| UntagResource | Remove tags from an EventBridge resource |
| ListTagsForResource | List all tags on an EventBridge resource |

### Wire Protocol Details

EventBridge uses JSON RPC over HTTP POST. The `X-Amz-Target` header must be set to `AWSEvents.<Action>` or `AmazonEventBridge.<Action>`. A default event bus named `default` is always available.

- **Event pattern matching is not evaluated**: `PutEvents` accepts events and returns event IDs, but events are not matched against rules or delivered to targets.
- **Rules and targets are stored**: you can create rules with event patterns and attach targets, but no invocation occurs.

### Usage with AWS CLI

```bash
# Create a custom event bus
aws events create-event-bus \
  --name my-bus \
  --endpoint-url http://localhost:9195 \
  --no-sign-request

# Describe the default event bus
aws events describe-event-bus \
  --endpoint-url http://localhost:9195 \
  --no-sign-request

# List event buses
aws events list-event-buses \
  --endpoint-url http://localhost:9195 \
  --no-sign-request

# Put a rule
aws events put-rule \
  --name my-rule \
  --event-pattern '{"source":["my.app"]}' \
  --endpoint-url http://localhost:9195 \
  --no-sign-request

# List rules
aws events list-rules \
  --endpoint-url http://localhost:9195 \
  --no-sign-request

# Put targets on a rule
aws events put-targets \
  --rule my-rule \
  --targets "Id=target1,Arn=arn:aws:sqs:us-east-1:000000000000:my-queue" \
  --endpoint-url http://localhost:9195 \
  --no-sign-request

# List targets by rule
aws events list-targets-by-rule \
  --rule my-rule \
  --endpoint-url http://localhost:9195 \
  --no-sign-request

# Put events
aws events put-events \
  --entries '[{"Source":"my.app","DetailType":"MyEvent","Detail":"{\"key\":\"value\"}"}]' \
  --endpoint-url http://localhost:9195 \
  --no-sign-request

# Tag a resource
aws events tag-resource \
  --resource-arn arn:aws:events:us-east-1:000000000000:event-bus/my-bus \
  --tags Key=env,Value=test \
  --endpoint-url http://localhost:9195 \
  --no-sign-request

# Delete a rule
aws events delete-rule \
  --name my-rule \
  --endpoint-url http://localhost:9195 \
  --no-sign-request

# Delete a custom event bus
aws events delete-event-bus \
  --name my-bus \
  --endpoint-url http://localhost:9195 \
  --no-sign-request
```

### Limitations

- Rules do not evaluate events. `PutEvents` accepts events but does not match them against rules or invoke targets.
- No cross-account or cross-region event delivery.

---

## AppSync

| | |
|---|---|
| **Port** | `9700` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:9700` |

### Supported Operations (19)

| Operation | Description |
|-----------|-------------|
| CreateGraphqlApi | Create a new GraphQL API |
| GetGraphqlApi | Get details of a GraphQL API |
| ListGraphqlApis | List all GraphQL APIs |
| UpdateGraphqlApi | Update a GraphQL API |
| DeleteGraphqlApi | Delete a GraphQL API |
| CreateApiKey | Create an API key for a GraphQL API |
| ListApiKeys | List API keys for a GraphQL API |
| UpdateApiKey | Update an API key |
| DeleteApiKey | Delete an API key |
| CreateDataSource | Create a data source for a GraphQL API |
| GetDataSource | Get details of a data source |
| ListDataSources | List data sources for a GraphQL API |
| UpdateDataSource | Update a data source |
| DeleteDataSource | Delete a data source |
| StartSchemaCreation | Start creating/updating a GraphQL schema |
| GetSchemaCreationStatus | Get the status of a schema creation |
| TagResource | Add tags to an AppSync resource |
| UntagResource | Remove tags from an AppSync resource |
| ListTagsForResource | List all tags on an AppSync resource |

### Wire Protocol Details

AppSync uses REST JSON with versioned URL paths prefixed with `/v1/`. GraphQL API operations use `/v1/apis`, API key operations use `/v1/apis/{apiId}/apikeys`, data source operations use `/v1/apis/{apiId}/datasources`, and schema operations use `/v1/apis/{apiId}/schemacreation`. Tag operations use `/v1/tags/{resourceArn}`.

- **GraphQL APIs are simulated**: APIs are created with synthetic URIs (both GraphQL and real-time endpoints) but no actual GraphQL endpoint is running.
- **Schema creation is tracked**: the definition is stored and status transitions to `SUCCESS`, but no schema validation is performed.

### Usage with AWS CLI

```bash
# Create a GraphQL API
aws appsync create-graphql-api \
  --name my-api \
  --authentication-type API_KEY \
  --endpoint-url http://localhost:9700 \
  --no-sign-request

# List GraphQL APIs
aws appsync list-graphql-apis \
  --endpoint-url http://localhost:9700 \
  --no-sign-request

# Get a GraphQL API
aws appsync get-graphql-api \
  --api-id <api-id> \
  --endpoint-url http://localhost:9700 \
  --no-sign-request

# Update a GraphQL API
aws appsync update-graphql-api \
  --api-id <api-id> \
  --name updated-api \
  --authentication-type API_KEY \
  --endpoint-url http://localhost:9700 \
  --no-sign-request

# Create an API key
aws appsync create-api-key \
  --api-id <api-id> \
  --endpoint-url http://localhost:9700 \
  --no-sign-request

# List API keys
aws appsync list-api-keys \
  --api-id <api-id> \
  --endpoint-url http://localhost:9700 \
  --no-sign-request

# Create a data source
aws appsync create-data-source \
  --api-id <api-id> \
  --name myds \
  --type NONE \
  --endpoint-url http://localhost:9700 \
  --no-sign-request

# List data sources
aws appsync list-data-sources \
  --api-id <api-id> \
  --endpoint-url http://localhost:9700 \
  --no-sign-request

# Start schema creation
aws appsync start-schema-creation \
  --api-id <api-id> \
  --definition "dHlwZSBRdWVyeSB7IGhlbGxvOiBTdHJpbmcgfQ==" \
  --endpoint-url http://localhost:9700 \
  --no-sign-request

# Delete a GraphQL API
aws appsync delete-graphql-api \
  --api-id <api-id> \
  --endpoint-url http://localhost:9700 \
  --no-sign-request
```

### Limitations

- GraphQL APIs are created with synthetic URIs but no actual GraphQL endpoint is running.
- Schema creation is tracked but no schema validation is performed.
- No resolver execution or GraphQL query processing.

---

## Step Functions

| | |
|---|---|
| **Port** | `8083` |
| **Protocol** | JSON RPC (`AmazonStates`) |
| **Endpoint** | `http://localhost:8083` |

### Supported Operations (15)

| Operation | Description |
|-----------|-------------|
| CreateStateMachine | Create a new state machine with an ASL definition |
| DeleteStateMachine | Delete a state machine |
| DescribeStateMachine | Describe a state machine |
| ListStateMachines | List all state machines |
| StartExecution | Start an execution of a state machine |
| StopExecution | Stop a running execution |
| DescribeExecution | Describe an execution |
| ListExecutions | List executions, optionally filtered by state machine |
| GetExecutionHistory | Get the event history of an execution |
| SendTaskSuccess | Complete a task with output |
| SendTaskFailure | Fail a task with an error |
| SendTaskHeartbeat | Send a heartbeat for a task |
| TagResource | Add tags to a Step Functions resource |
| UntagResource | Remove tags from a Step Functions resource |
| ListTagsForResource | List all tags on a Step Functions resource |

### Wire Protocol Details

Step Functions uses JSON RPC over HTTP POST. The `X-Amz-Target` header must be set to `AWSStepFunctions.<Action>` or `AmazonStates.<Action>`. The state machine definition is stored as an ASL (Amazon States Language) JSON string.

- **Executions do not run**: `StartExecution` creates an execution in `RUNNING` state but does not evaluate the state machine definition.
- **Task callbacks are accepted**: `SendTaskSuccess`, `SendTaskFailure`, and `SendTaskHeartbeat` are accepted but do not advance execution state.
- **Execution history**: `GetExecutionHistory` returns a synthetic `ExecutionStarted` event.

### Usage with AWS CLI

```bash
# Create a state machine
aws stepfunctions create-state-machine \
  --name my-state-machine \
  --definition '{"StartAt":"Hello","States":{"Hello":{"Type":"Pass","End":true}}}' \
  --role-arn arn:aws:iam::000000000000:role/StepFunctionsRole \
  --endpoint-url http://localhost:8083 \
  --no-sign-request

# List state machines
aws stepfunctions list-state-machines \
  --endpoint-url http://localhost:8083 \
  --no-sign-request

# Describe a state machine
aws stepfunctions describe-state-machine \
  --state-machine-arn arn:aws:states:us-east-1:000000000000:stateMachine:my-state-machine \
  --endpoint-url http://localhost:8083 \
  --no-sign-request

# Start an execution
aws stepfunctions start-execution \
  --state-machine-arn arn:aws:states:us-east-1:000000000000:stateMachine:my-state-machine \
  --input '{"key":"value"}' \
  --endpoint-url http://localhost:8083 \
  --no-sign-request

# List executions
aws stepfunctions list-executions \
  --endpoint-url http://localhost:8083 \
  --no-sign-request

# Describe an execution
aws stepfunctions describe-execution \
  --execution-arn arn:aws:states:us-east-1:000000000000:execution:my-state-machine:exec-id \
  --endpoint-url http://localhost:8083 \
  --no-sign-request

# Stop an execution
aws stepfunctions stop-execution \
  --execution-arn arn:aws:states:us-east-1:000000000000:execution:my-state-machine:exec-id \
  --endpoint-url http://localhost:8083 \
  --no-sign-request

# Tag a resource
aws stepfunctions tag-resource \
  --resource-arn arn:aws:states:us-east-1:000000000000:stateMachine:my-state-machine \
  --tags Key=env,Value=test \
  --endpoint-url http://localhost:8083 \
  --no-sign-request

# Delete a state machine
aws stepfunctions delete-state-machine \
  --state-machine-arn arn:aws:states:us-east-1:000000000000:stateMachine:my-state-machine \
  --endpoint-url http://localhost:8083 \
  --no-sign-request
```

### Limitations

- Executions do not run. `StartExecution` creates an execution in RUNNING state but does not evaluate the state machine definition.
- Task callbacks (`SendTaskSuccess`, `SendTaskFailure`, `SendTaskHeartbeat`) are accepted but do not advance execution state.

---

## SWF

| | |
|---|---|
| **Port** | `10115` |
| **Protocol** | JSON RPC (`SimpleWorkflowService`) |
| **Endpoint** | `http://localhost:10115` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateDomain | Create a SWF domain |
| DescribeDomain | Describe a SWF domain |
| ListDomains | List all SWF domains |
| DeleteDomain | Delete a SWF domain |

### Usage with AWS CLI

```bash
# List domains
aws swf list-domains \
  --registration-status REGISTERED \
  --endpoint-url http://localhost:10115 \
  --no-sign-request
```

---

## MQ

| | |
|---|---|
| **Port** | `10113` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10113` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateBroker | Create an MQ broker |
| ListBrokers | List all MQ brokers |
| GetBroker | Get details of an MQ broker |
| DeleteBroker | Delete an MQ broker |

### Usage with AWS CLI

```bash
# List brokers
aws mq list-brokers \
  --endpoint-url http://localhost:10113 \
  --no-sign-request
```

---

## MWAA

| | |
|---|---|
| **Port** | `10114` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10114` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateEnvironment | Create an MWAA environment |
| ListEnvironments | List all MWAA environments |
| GetEnvironment | Get details of an MWAA environment |
| DeleteEnvironment | Delete an MWAA environment |

### Usage with AWS CLI

```bash
# List environments
aws mwaa list-environments \
  --endpoint-url http://localhost:10114 \
  --no-sign-request
```

---

## Service Catalog

| | |
|---|---|
| **Port** | `9400` |
| **Protocol** | JSON RPC (`AWS242ServiceCatalogService`) |
| **Endpoint** | `http://localhost:9400` |

### Supported Operations (16)

| Operation | Description |
|-----------|-------------|
| CreatePortfolio | Create a new portfolio |
| DeletePortfolio | Delete a portfolio |
| DescribePortfolio | Describe a portfolio |
| ListPortfolios | List all portfolios |
| UpdatePortfolio | Update a portfolio |
| CreateProduct | Create a new product |
| DeleteProduct | Delete a product |
| DescribeProduct | Describe a product |
| UpdateProduct | Update a product |
| SearchProducts | Search for products by filters or full-text query |
| AssociateProductWithPortfolio | Associate a product with a portfolio |
| DisassociateProductFromPortfolio | Disassociate a product from a portfolio |
| ProvisionProduct | Provision a product (simulated) |
| DescribeProvisionedProduct | Describe a provisioned product |
| SearchProvisionedProducts | Search for provisioned products |
| TerminateProvisionedProduct | Terminate a provisioned product |

### Wire Protocol Details

Service Catalog uses JSON RPC over HTTP POST. The `X-Amz-Target` header must be set to `AWS242ServiceCatalogService.<Action>`.

- **Portfolio and product management**: full CRUD operations for portfolios and products, with association/disassociation support.
- **Provisioning is simulated**: `ProvisionProduct` creates a provisioned product record with an `AVAILABLE` status but does not deploy CloudFormation stacks.
- **Search**: `SearchProducts` supports filtering by product name and full-text search.

### Usage with AWS CLI

```bash
# Create a portfolio
aws servicecatalog create-portfolio \
  --display-name "My Portfolio" \
  --provider-name "My Org" \
  --endpoint-url http://localhost:9400 \
  --no-sign-request

# List portfolios
aws servicecatalog list-portfolios \
  --endpoint-url http://localhost:9400 \
  --no-sign-request

# Create a product
aws servicecatalog create-product \
  --name "My Product" \
  --owner "My Org" \
  --product-type CLOUD_FORMATION_TEMPLATE \
  --provisioning-artifact-parameters '{"Name":"v1","Info":{"LoadTemplateFromURL":"https://example.com/template.yaml"}}' \
  --endpoint-url http://localhost:9400 \
  --no-sign-request

# Search products
aws servicecatalog search-products \
  --endpoint-url http://localhost:9400 \
  --no-sign-request

# Associate product with portfolio
aws servicecatalog associate-product-with-portfolio \
  --product-id prod-12345678 \
  --portfolio-id port-12345678 \
  --endpoint-url http://localhost:9400 \
  --no-sign-request

# Provision a product
aws servicecatalog provision-product \
  --product-id prod-12345678 \
  --provisioning-artifact-id pa-12345678 \
  --provisioned-product-name my-instance \
  --endpoint-url http://localhost:9400 \
  --no-sign-request

# Describe a provisioned product
aws servicecatalog describe-provisioned-product \
  --id pp-12345678 \
  --endpoint-url http://localhost:9400 \
  --no-sign-request

# Terminate a provisioned product
aws servicecatalog terminate-provisioned-product \
  --provisioned-product-id pp-12345678 \
  --endpoint-url http://localhost:9400 \
  --no-sign-request

# Delete a portfolio
aws servicecatalog delete-portfolio \
  --id port-12345678 \
  --endpoint-url http://localhost:9400 \
  --no-sign-request
```

### Limitations

- Provisioning is simulated. `ProvisionProduct` creates a record but does not deploy CloudFormation stacks.
- No constraint or launch path management.
