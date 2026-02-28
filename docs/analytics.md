# Analytics

## Athena

| Property | Value |
|----------|-------|
| Port | `10050` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10050` |
| Target prefix | `AmazonAthena` |

### Operations (4)

| Operation | Description |
|-----------|-------------|
| `CreateWorkGroup` | Create a new workgroup |
| `DescribeWorkGroup` | Describe a workgroup |
| `ListWorkGroups` | List all workgroups |
| `DeleteWorkGroup` | Delete a workgroup |

### CLI Example

```bash
aws athena list-work-groups \
  --endpoint-url http://localhost:10050 \
  --no-sign-request
```

---

## Glue

| Property | Value |
|----------|-------|
| Port | `10065` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10065` |
| Target prefix | `AWSGlue` |

### Operations (12)

| Operation | Description |
|-----------|-------------|
| `CreateDatabase` | Create a new database |
| `DescribeDatabase` | Describe a database |
| `ListDatabases` | List all databases |
| `DeleteDatabase` | Delete a database |
| `CreateTable` | Create a new table |
| `DescribeTable` | Describe a table |
| `ListTables` | List all tables |
| `DeleteTable` | Delete a table |
| `CreateJob` | Create a new job |
| `DescribeJob` | Describe a job |
| `ListJobs` | List all jobs |
| `DeleteJob` | Delete a job |

### CLI Example

```bash
aws glue create-database \
  --database-input '{"Name":"my-database"}' \
  --endpoint-url http://localhost:10065 \
  --no-sign-request
```

---

## EMR

| Property | Value |
|----------|-------|
| Port | `10053` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10053` |
| Target prefix | `ElasticMapReduce` |

### Operations (4)

| Operation | Description |
|-----------|-------------|
| `CreateCluster` | Create a new cluster |
| `DescribeCluster` | Describe a cluster |
| `ListClusters` | List all clusters |
| `DeleteCluster` | Delete a cluster |

### CLI Example

```bash
aws emr list-clusters \
  --endpoint-url http://localhost:10053 \
  --no-sign-request
```

---

## OpenSearch

| Property | Value |
|----------|-------|
| Port | `10058` |
| Protocol | REST JSON |
| Endpoint | `http://localhost:10058` |

### Operations (4)

| Operation | Method | Path |
|-----------|--------|------|
| `CreateDomain` | POST | `/2021-01-01/opensearch/domain` |
| `ListDomains` | GET | `/2021-01-01/domain` |
| `GetDomain` | GET | `/2021-01-01/opensearch/domain/{domainName}` |
| `DeleteDomain` | DELETE | `/2021-01-01/opensearch/domain/{domainName}` |

### CLI Example

```bash
aws opensearch list-domain-names \
  --endpoint-url http://localhost:10058 \
  --no-sign-request
```

---

## Kinesis

| Property | Value |
|----------|-------|
| Port | `4568` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:4568` |
| Target prefix | `Kinesis_20131202` |

### Operations (15)

| Operation | Description |
|-----------|-------------|
| `CreateStream` | Create a new data stream |
| `DeleteStream` | Delete a data stream |
| `DescribeStream` | Get detailed information about a stream |
| `DescribeStreamSummary` | Get a summary of stream details |
| `ListStreams` | List all data streams |
| `PutRecord` | Write a single record to a stream |
| `PutRecords` | Write multiple records to a stream in a single call |
| `GetShardIterator` | Get an iterator for reading from a shard |
| `GetRecords` | Read records from a shard using an iterator |
| `ListShards` | List the shards in a stream |
| `AddTagsToStream` | Add tags to a stream |
| `RemoveTagsFromStream` | Remove tags from a stream |
| `ListTagsForStream` | List tags for a stream |
| `IncreaseStreamRetentionPeriod` | Increase the data retention period |
| `DecreaseStreamRetentionPeriod` | Decrease the data retention period |

### Wire Protocol

All requests are HTTP POST to `/` with headers:

```
Content-Type: application/x-amz-json-1.1
X-Amz-Target: Kinesis_20131202.<Action>
```

The request body is a JSON object specific to each action.

### CLI Examples

**Create a stream, write records, and read them back:**

```bash
# Create a stream
aws kinesis create-stream \
  --stream-name my-stream \
  --shard-count 1 \
  --endpoint-url http://localhost:4568 \
  --no-sign-request

# Describe the stream
aws kinesis describe-stream \
  --stream-name my-stream \
  --endpoint-url http://localhost:4568 \
  --no-sign-request

# Put a single record
aws kinesis put-record \
  --stream-name my-stream \
  --partition-key pk1 \
  --data "SGVsbG8gS2luZXNpcw==" \
  --endpoint-url http://localhost:4568 \
  --no-sign-request

# Put multiple records
aws kinesis put-records \
  --stream-name my-stream \
  --records '[{"Data":"cmVjb3JkMQ==","PartitionKey":"pk1"},{"Data":"cmVjb3JkMg==","PartitionKey":"pk2"}]' \
  --endpoint-url http://localhost:4568 \
  --no-sign-request

# Get a shard iterator
aws kinesis get-shard-iterator \
  --stream-name my-stream \
  --shard-id shardId-000000000000 \
  --shard-iterator-type TRIM_HORIZON \
  --endpoint-url http://localhost:4568 \
  --no-sign-request

# Get records (use the ShardIterator from the previous command)
aws kinesis get-records \
  --shard-iterator <shard-iterator-value> \
  --endpoint-url http://localhost:4568 \
  --no-sign-request

# List streams
aws kinesis list-streams \
  --endpoint-url http://localhost:4568 \
  --no-sign-request

# List shards
aws kinesis list-shards \
  --stream-name my-stream \
  --endpoint-url http://localhost:4568 \
  --no-sign-request

# Tag a stream
aws kinesis add-tags-to-stream \
  --stream-name my-stream \
  --tags env=dev \
  --endpoint-url http://localhost:4568 \
  --no-sign-request

# Adjust retention period
aws kinesis increase-stream-retention-period \
  --stream-name my-stream \
  --retention-period-hours 48 \
  --endpoint-url http://localhost:4568 \
  --no-sign-request

# Delete the stream
aws kinesis delete-stream \
  --stream-name my-stream \
  --endpoint-url http://localhost:4568 \
  --no-sign-request
```

### Limitations

- Records are stored in memory only and lost on restart.
- Shard splitting and merging are not supported.
- Enhanced fan-out consumers are not supported.
- Encryption settings are accepted but data is not encrypted.

---

## Firehose

| Property | Value |
|----------|-------|
| Port | `4573` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:4573` |
| Target prefix | `Firehose_20150804` |

### Operations (10)

| Operation | Description |
|-----------|-------------|
| `CreateDeliveryStream` | Create a new delivery stream |
| `DeleteDeliveryStream` | Delete a delivery stream |
| `DescribeDeliveryStream` | Describe a delivery stream |
| `ListDeliveryStreams` | List all delivery streams |
| `UpdateDestination` | Update the destination configuration |
| `PutRecord` | Write a single record to a delivery stream |
| `PutRecordBatch` | Write multiple records in a single call |
| `TagDeliveryStream` | Add tags to a delivery stream |
| `UntagDeliveryStream` | Remove tags from a delivery stream |
| `ListTagsForDeliveryStream` | List tags for a delivery stream |

### Wire Protocol

All requests are HTTP POST to `/` with headers:

```
Content-Type: application/x-amz-json-1.1
X-Amz-Target: Firehose_20150804.<Action>
```

The request body is a JSON object specific to each action.

### CLI Examples

**Create a delivery stream and send records:**

```bash
# Create a delivery stream
aws firehose create-delivery-stream \
  --delivery-stream-name my-stream \
  --endpoint-url http://localhost:4573 \
  --no-sign-request

# Describe the delivery stream
aws firehose describe-delivery-stream \
  --delivery-stream-name my-stream \
  --endpoint-url http://localhost:4573 \
  --no-sign-request

# Put a single record
aws firehose put-record \
  --delivery-stream-name my-stream \
  --record '{"Data":"SGVsbG8gRmlyZWhvc2U="}' \
  --endpoint-url http://localhost:4573 \
  --no-sign-request

# Put a batch of records
aws firehose put-record-batch \
  --delivery-stream-name my-stream \
  --records '[{"Data":"cmVjb3JkMQ=="},{"Data":"cmVjb3JkMg=="}]' \
  --endpoint-url http://localhost:4573 \
  --no-sign-request

# List delivery streams
aws firehose list-delivery-streams \
  --endpoint-url http://localhost:4573 \
  --no-sign-request

# Tag a delivery stream
aws firehose tag-delivery-stream \
  --delivery-stream-name my-stream \
  --tags Key=env,Value=dev \
  --endpoint-url http://localhost:4573 \
  --no-sign-request

# List tags
aws firehose list-tags-for-delivery-stream \
  --delivery-stream-name my-stream \
  --endpoint-url http://localhost:4573 \
  --no-sign-request

# Delete the delivery stream
aws firehose delete-delivery-stream \
  --delivery-stream-name my-stream \
  --endpoint-url http://localhost:4573 \
  --no-sign-request
```

### Limitations

- Records are accepted and stored in memory but not delivered to any destination.
- Delivery stream transformations and buffering configurations are stored but not applied.
- All state is in-memory only.

---

## QuickSight

| Property | Value |
|----------|-------|
| Port | `10059` |
| Protocol | REST JSON |
| Endpoint | `http://localhost:10059` |

### Operations (4)

| Operation | Method | Path |
|-----------|--------|------|
| `CreateDataSet` | POST | `/accounts/{accountId}/data-sets` |
| `ListDataSets` | GET | `/accounts/{accountId}/data-sets` |
| `GetDataSet` | GET | `/accounts/{accountId}/data-sets/{dataSetId}` |
| `DeleteDataSet` | DELETE | `/accounts/{accountId}/data-sets/{dataSetId}` |

### CLI Example

```bash
aws quicksight list-data-sets \
  --aws-account-id 000000000000 \
  --endpoint-url http://localhost:10059 \
  --no-sign-request
```

---

## CloudSearch

| Property | Value |
|----------|-------|
| Port | `10051` |
| Protocol | Query/XML |
| Endpoint | `http://localhost:10051` |
| Action parameter | `Action` query parameter |

### Operations (3)

| Operation | Description |
|-----------|-------------|
| `CreateDomain` | Create a new search domain |
| `DescribeDomains` | Describe one or more search domains |
| `DeleteDomain` | Delete a search domain |

### CLI Example

```bash
aws cloudsearch create-domain \
  --domain-name my-domain \
  --endpoint-url http://localhost:10051 \
  --no-sign-request
```

---

## MSK

| Property | Value |
|----------|-------|
| Port | `10057` |
| Protocol | REST JSON |
| Endpoint | `http://localhost:10057` |

### Operations (4)

| Operation | Method | Path |
|-----------|--------|------|
| `CreateCluster` | POST | `/v1/clusters` |
| `ListClusters` | GET | `/v1/clusters` |
| `GetCluster` | GET | `/v1/clusters/{clusterArn}` |
| `DeleteCluster` | DELETE | `/v1/clusters/{clusterArn}` |

### CLI Example

```bash
aws kafka list-clusters \
  --endpoint-url http://localhost:10057 \
  --no-sign-request
```

---

## Kinesis Video Streams

| Property | Value |
|----------|-------|
| Port | `10055` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10055` |
| Target prefix | `KinesisVideo_20170930` |

### Operations (4)

| Operation | Description |
|-----------|-------------|
| `CreateStream` | Create a new video stream |
| `DescribeStream` | Describe a video stream |
| `ListStreams` | List all video streams |
| `DeleteStream` | Delete a video stream |

### CLI Example

```bash
aws kinesisvideo list-streams \
  --endpoint-url http://localhost:10055 \
  --no-sign-request
```

---

## Managed Flink

| Property | Value |
|----------|-------|
| Port | `10056` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10056` |
| Target prefix | `KinesisAnalytics_20180523` |

### Operations (4)

| Operation | Description |
|-----------|-------------|
| `CreateApplication` | Create a new Flink application |
| `DescribeApplication` | Describe a Flink application |
| `ListApplications` | List all Flink applications |
| `DeleteApplication` | Delete a Flink application |

### CLI Example

```bash
aws kinesisanalyticsv2 list-applications \
  --endpoint-url http://localhost:10056 \
  --no-sign-request
```

---

## DataZone

| Property | Value |
|----------|-------|
| Port | `10052` |
| Protocol | REST JSON |
| Endpoint | `http://localhost:10052` |

### Operations (4)

| Operation | Method | Path |
|-----------|--------|------|
| `CreateDomain` | POST | `/v2/domains` |
| `GetDomain` | GET | `/v2/domains/{domainId}` |
| `ListDomains` | GET | `/v2/domains` |
| `DeleteDomain` | DELETE | `/v2/domains/{domainId}` |

### CLI Example

```bash
aws datazone list-domains \
  --endpoint-url http://localhost:10052 \
  --no-sign-request
```

---

## FinSpace

| Property | Value |
|----------|-------|
| Port | `10054` |
| Protocol | REST JSON |
| Endpoint | `http://localhost:10054` |

### Operations (4)

| Operation | Method | Path |
|-----------|--------|------|
| `CreateEnvironment` | POST | `/environment` |
| `GetEnvironment` | GET | `/environment/{environmentId}` |
| `ListEnvironments` | GET | `/environment` |
| `DeleteEnvironment` | DELETE | `/environment/{environmentId}` |

### CLI Example

```bash
aws finspace list-environments \
  --endpoint-url http://localhost:10054 \
  --no-sign-request
```

---

## Lake Formation

| Property | Value |
|----------|-------|
| Port | `10066` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10066` |
| Target prefix | `AWSLakeFormation` |

### Operations (4)

| Operation | Description |
|-----------|-------------|
| `CreateResource` | Create a resource |
| `DescribeResource` | Describe a resource |
| `ListResources` | List resources |
| `DeleteResource` | Delete a resource |

### CLI Example

```bash
aws lakeformation list-resources \
  --endpoint-url http://localhost:10066 \
  --no-sign-request
```

---

## Data Exchange

| Property | Value |
|----------|-------|
| Port | `10062` |
| Protocol | REST JSON |
| Endpoint | `http://localhost:10062` |

### Operations (4)

| Operation | Method | Path |
|-----------|--------|------|
| `CreateDataSet` | POST | `/v1/data-sets` |
| `GetDataSet` | GET | `/v1/data-sets/{dataSetId}` |
| `ListDataSets` | GET | `/v1/data-sets` |
| `DeleteDataSet` | DELETE | `/v1/data-sets/{dataSetId}` |

### CLI Example

```bash
aws dataexchange list-data-sets \
  --endpoint-url http://localhost:10062 \
  --no-sign-request
```

---

## Data Pipeline

| Property | Value |
|----------|-------|
| Port | `10063` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10063` |
| Target prefix | `DataPipeline` |

### Operations (4)

| Operation | Description |
|-----------|-------------|
| `CreatePipeline` | Create a new pipeline |
| `DescribePipeline` | Describe a pipeline |
| `ListPipelines` | List all pipelines |
| `DeletePipeline` | Delete a pipeline |

### CLI Example

```bash
aws datapipeline list-pipelines \
  --endpoint-url http://localhost:10063 \
  --no-sign-request
```

---

## Entity Resolution

| Property | Value |
|----------|-------|
| Port | `10064` |
| Protocol | REST JSON |
| Endpoint | `http://localhost:10064` |

### Operations (4)

| Operation | Method | Path |
|-----------|--------|------|
| `CreateMatchingWorkflow` | POST | `/matchingworkflows` |
| `GetMatchingWorkflow` | GET | `/matchingworkflows/{workflowName}` |
| `ListMatchingWorkflows` | GET | `/matchingworkflows` |
| `DeleteMatchingWorkflow` | DELETE | `/matchingworkflows/{workflowName}` |

### CLI Example

```bash
aws entityresolution list-matching-workflows \
  --endpoint-url http://localhost:10064 \
  --no-sign-request
```

---

## Clean Rooms

| Property | Value |
|----------|-------|
| Port | `10061` |
| Protocol | REST JSON |
| Endpoint | `http://localhost:10061` |

### Operations (4)

| Operation | Method | Path |
|-----------|--------|------|
| `CreateCollaboration` | POST | `/collaborations` |
| `GetCollaboration` | GET | `/collaborations/{collaborationId}` |
| `ListCollaborations` | GET | `/collaborations` |
| `DeleteCollaboration` | DELETE | `/collaborations/{collaborationId}` |

### CLI Example

```bash
aws cleanrooms list-collaborations \
  --endpoint-url http://localhost:10061 \
  --no-sign-request
```
