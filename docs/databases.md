# Database Services

This category covers AWS database services including NoSQL, relational, graph, document, time-series, and in-memory databases. All data is held in memory with no disk persistence.

---

## DynamoDB

| | |
|---|---|
| **Port** | `8000` |
| **Protocol** | JSON RPC (`DynamoDB_20120810`) |
| **Endpoint** | `http://localhost:8000` |

### Supported Operations (16)

| Operation | Description |
|-----------|-------------|
| CreateTable | Create a new table with key schema and billing mode |
| DeleteTable | Delete an existing table and all its items |
| DescribeTable | Get detailed information about a table |
| ListTables | List all table names with optional pagination |
| UpdateTable | Update table settings (billing mode, provisioned throughput) |
| PutItem | Create or replace an item in a table |
| GetItem | Retrieve a single item by primary key |
| DeleteItem | Delete a single item by primary key |
| UpdateItem | Update attributes of an existing item using update expressions |
| Query | Query items using key condition expressions |
| Scan | Scan all items in a table with optional filter expressions |
| BatchGetItem | Retrieve up to 100 items across multiple tables |
| BatchWriteItem | Put or delete up to 25 items across multiple tables |
| TagResource | Add tags to a DynamoDB resource |
| UntagResource | Remove tags from a DynamoDB resource |
| ListTagsOfResource | List all tags on a DynamoDB resource |

### Wire Protocol Details

DynamoDB uses JSON RPC with the `X-Amz-Target` header set to `DynamoDB_20120810.<Action>`. All requests are POST to the root endpoint `/`. Request and response bodies are JSON. The service supports:

- **Key schemas**: HASH-only or HASH+RANGE composite keys
- **Attribute types**: S (String), N (Number), B (Binary), BOOL, NULL, L (List), M (Map), SS, NS, BS
- **Billing modes**: PAY_PER_REQUEST and PROVISIONED
- **Update expressions**: SET and REMOVE operations
- **Key condition expressions**: partition key equality with optional sort key conditions (=, <, >, <=, >=, BETWEEN, begins_with)
- **Filter expressions**: post-query filtering on non-key attributes
- **Projection expressions**: return only specified attributes

### Usage with AWS CLI

```bash
# Create a table
aws dynamodb create-table \
  --table-name MyTable \
  --attribute-definitions AttributeName=pk,AttributeType=S \
  --key-schema AttributeName=pk,KeyType=HASH \
  --billing-mode PAY_PER_REQUEST \
  --endpoint-url http://localhost:8000 \
  --no-sign-request

# Put an item
aws dynamodb put-item \
  --table-name MyTable \
  --item '{"pk": {"S": "user-1"}, "name": {"S": "Alice"}, "age": {"N": "30"}}' \
  --endpoint-url http://localhost:8000 \
  --no-sign-request

# Get an item
aws dynamodb get-item \
  --table-name MyTable \
  --key '{"pk": {"S": "user-1"}}' \
  --endpoint-url http://localhost:8000 \
  --no-sign-request

# Query items
aws dynamodb query \
  --table-name MyTable \
  --key-condition-expression "pk = :pk" \
  --expression-attribute-values '{":pk": {"S": "user-1"}}' \
  --endpoint-url http://localhost:8000 \
  --no-sign-request

# Scan all items
aws dynamodb scan \
  --table-name MyTable \
  --endpoint-url http://localhost:8000 \
  --no-sign-request

# Update an item
aws dynamodb update-item \
  --table-name MyTable \
  --key '{"pk": {"S": "user-1"}}' \
  --update-expression "SET age = :age" \
  --expression-attribute-values '{":age": {"N": "31"}}' \
  --endpoint-url http://localhost:8000 \
  --no-sign-request

# Delete an item
aws dynamodb delete-item \
  --table-name MyTable \
  --key '{"pk": {"S": "user-1"}}' \
  --endpoint-url http://localhost:8000 \
  --no-sign-request

# List tables
aws dynamodb list-tables \
  --endpoint-url http://localhost:8000 \
  --no-sign-request

# Delete a table
aws dynamodb delete-table \
  --table-name MyTable \
  --endpoint-url http://localhost:8000 \
  --no-sign-request
```

### Limitations

- Basic `KeyConditionExpression`, `UpdateExpression` (SET, REMOVE), `FilterExpression`, and `ProjectionExpression` are supported.
- Transactions (TransactGetItems, TransactWriteItems) are not implemented.
- Global Secondary Indexes (GSIs) and Local Secondary Indexes (LSIs) are not implemented.
- DynamoDB Streams are not implemented.
- Conditional expressions are not supported.

---

## RDS

| | |
|---|---|
| **Port** | `10012` |
| **Protocol** | Query / XML |
| **Endpoint** | `http://localhost:10012` |

### Supported Operations (6)

| Operation | Description |
|-----------|-------------|
| CreateDBInstance | Create a new RDS database instance |
| DescribeDBInstances | Describe one or more DB instances |
| DeleteDBInstance | Delete a DB instance |
| CreateDBCluster | Create a new RDS Aurora cluster |
| DescribeDBClusters | Describe one or more DB clusters |
| DeleteDBCluster | Delete a DB cluster |

### Usage with AWS CLI

```bash
# Create a DB instance
aws rds create-db-instance \
  --db-instance-identifier my-db \
  --db-instance-class db.t3.micro \
  --engine mysql \
  --endpoint-url http://localhost:10012 \
  --no-sign-request

# Describe DB instances
aws rds describe-db-instances \
  --endpoint-url http://localhost:10012 \
  --no-sign-request

# Delete a DB instance
aws rds delete-db-instance \
  --db-instance-identifier my-db \
  --skip-final-snapshot \
  --endpoint-url http://localhost:10012 \
  --no-sign-request
```

---

## ElastiCache

| | |
|---|---|
| **Port** | `10014` |
| **Protocol** | Query / XML |
| **Endpoint** | `http://localhost:10014` |

### Supported Operations (3)

| Operation | Description |
|-----------|-------------|
| CreateCacheCluster | Create a new ElastiCache cluster |
| DescribeCacheClusters | Describe one or more cache clusters |
| DeleteCacheCluster | Delete a cache cluster |

### Usage with AWS CLI

```bash
# Create a cache cluster
aws elasticache create-cache-cluster \
  --cache-cluster-id my-cache \
  --engine redis \
  --cache-node-type cache.t3.micro \
  --num-cache-nodes 1 \
  --endpoint-url http://localhost:10014 \
  --no-sign-request

# Describe cache clusters
aws elasticache describe-cache-clusters \
  --endpoint-url http://localhost:10014 \
  --no-sign-request

# Delete a cache cluster
aws elasticache delete-cache-cluster \
  --cache-cluster-id my-cache \
  --endpoint-url http://localhost:10014 \
  --no-sign-request
```

---

## Neptune

| | |
|---|---|
| **Port** | `10016` |
| **Protocol** | Query / XML |
| **Endpoint** | `http://localhost:10016` |

### Supported Operations (3)

| Operation | Description |
|-----------|-------------|
| CreateDBCluster | Create a new Neptune DB cluster |
| DescribeDBClusters | Describe one or more Neptune DB clusters |
| DeleteDBCluster | Delete a Neptune DB cluster |

### Usage with AWS CLI

```bash
# Create a Neptune cluster
aws neptune create-db-cluster \
  --db-cluster-identifier my-neptune \
  --engine neptune \
  --endpoint-url http://localhost:10016 \
  --no-sign-request

# Describe Neptune clusters
aws neptune describe-db-clusters \
  --endpoint-url http://localhost:10016 \
  --no-sign-request

# Delete a Neptune cluster
aws neptune delete-db-cluster \
  --db-cluster-identifier my-neptune \
  --skip-final-snapshot \
  --endpoint-url http://localhost:10016 \
  --no-sign-request
```

---

## DocumentDB

| | |
|---|---|
| **Port** | `10013` |
| **Protocol** | Query / XML |
| **Endpoint** | `http://localhost:10013` |

### Supported Operations (3)

| Operation | Description |
|-----------|-------------|
| CreateDBCluster | Create a new DocumentDB cluster |
| DescribeDBClusters | Describe one or more DocumentDB clusters |
| DeleteDBCluster | Delete a DocumentDB cluster |

### Usage with AWS CLI

```bash
# Create a DocumentDB cluster
aws docdb create-db-cluster \
  --db-cluster-identifier my-docdb \
  --engine docdb \
  --master-username admin \
  --master-user-password secret123 \
  --endpoint-url http://localhost:10013 \
  --no-sign-request

# Describe DocumentDB clusters
aws docdb describe-db-clusters \
  --endpoint-url http://localhost:10013 \
  --no-sign-request

# Delete a DocumentDB cluster
aws docdb delete-db-cluster \
  --db-cluster-identifier my-docdb \
  --skip-final-snapshot \
  --endpoint-url http://localhost:10013 \
  --no-sign-request
```

---

## Timestream

| | |
|---|---|
| **Port** | `10017` |
| **Protocol** | JSON RPC (`Timestream_20181101`) |
| **Endpoint** | `http://localhost:10017` |

### Supported Operations (8)

| Operation | Description |
|-----------|-------------|
| CreateDatabase | Create a new Timestream database |
| DescribeDatabase | Describe a Timestream database |
| ListDatabases | List all Timestream databases |
| DeleteDatabase | Delete a Timestream database |
| CreateTable | Create a new table in a Timestream database |
| DescribeTable | Describe a Timestream table |
| ListTables | List all tables in a Timestream database |
| DeleteTable | Delete a Timestream table |

### Usage with AWS CLI

```bash
# Create a database
aws timestream-write create-database \
  --database-name my-db \
  --endpoint-url http://localhost:10017 \
  --no-sign-request

# List databases
aws timestream-write list-databases \
  --endpoint-url http://localhost:10017 \
  --no-sign-request

# Create a table
aws timestream-write create-table \
  --database-name my-db \
  --table-name my-table \
  --endpoint-url http://localhost:10017 \
  --no-sign-request

# List tables
aws timestream-write list-tables \
  --database-name my-db \
  --endpoint-url http://localhost:10017 \
  --no-sign-request
```

---

## Keyspaces

| | |
|---|---|
| **Port** | `10015` |
| **Protocol** | JSON RPC (`KeyspacesService`) |
| **Endpoint** | `http://localhost:10015` |

### Supported Operations (8)

| Operation | Description |
|-----------|-------------|
| CreateKeyspace | Create a new keyspace |
| DescribeKeyspace | Describe a keyspace |
| ListKeyspaces | List all keyspaces |
| DeleteKeyspace | Delete a keyspace |
| CreateTable | Create a new table in a keyspace |
| DescribeTable | Describe a table in a keyspace |
| ListTables | List all tables in a keyspace |
| DeleteTable | Delete a table from a keyspace |

### Usage with AWS CLI

```bash
# Create a keyspace
aws keyspaces create-keyspace \
  --keyspace-name my_keyspace \
  --endpoint-url http://localhost:10015 \
  --no-sign-request

# List keyspaces
aws keyspaces list-keyspaces \
  --endpoint-url http://localhost:10015 \
  --no-sign-request

# Create a table
aws keyspaces create-table \
  --keyspace-name my_keyspace \
  --table-name my_table \
  --schema-definition '{"allColumns":[{"name":"id","type":"text"}],"partitionKeys":[{"name":"id"}]}' \
  --endpoint-url http://localhost:10015 \
  --no-sign-request

# List tables
aws keyspaces list-tables \
  --keyspace-name my_keyspace \
  --endpoint-url http://localhost:10015 \
  --no-sign-request
```

---

## MemoryDB

| | |
|---|---|
| **Port** | `6379` |
| **Protocol** | JSON RPC (`AmazonMemoryDB`) |
| **Endpoint** | `http://localhost:6379` |

### Supported Operations (21)

| Operation | Description |
|-----------|-------------|
| CreateCluster | Create a new MemoryDB cluster |
| DeleteCluster | Delete a MemoryDB cluster |
| DescribeClusters | Describe one or more MemoryDB clusters |
| UpdateCluster | Update cluster configuration |
| CreateSubnetGroup | Create a subnet group for MemoryDB clusters |
| DeleteSubnetGroup | Delete a subnet group |
| DescribeSubnetGroups | Describe one or more subnet groups |
| CreateUser | Create a new MemoryDB user for authentication |
| DeleteUser | Delete a MemoryDB user |
| DescribeUsers | Describe one or more MemoryDB users |
| UpdateUser | Update user authentication or access string |
| CreateACL | Create an access control list |
| DeleteACL | Delete an access control list |
| DescribeACLs | Describe one or more access control lists |
| UpdateACL | Update ACL user membership |
| CreateSnapshot | Create a snapshot of a cluster |
| DeleteSnapshot | Delete a snapshot |
| DescribeSnapshots | Describe one or more snapshots |
| TagResource | Add tags to a MemoryDB resource |
| UntagResource | Remove tags from a MemoryDB resource |
| ListTags | List all tags on a MemoryDB resource |

### Wire Protocol Details

MemoryDB uses JSON RPC with the `X-Amz-Target` header set to `AmazonMemoryDB.<Action>`. All requests are POST to the root endpoint `/`. The service manages five resource types: clusters, subnet groups, users, ACLs, and snapshots. Each resource type supports full CRUD operations. Tags can be managed on any resource via ARN.

### Usage with AWS CLI

```bash
# Create a subnet group
aws memorydb create-subnet-group \
  --subnet-group-name my-subnet-group \
  --subnet-ids subnet-12345678 \
  --endpoint-url http://localhost:6379 \
  --no-sign-request

# Create a user
aws memorydb create-user \
  --user-name my-user \
  --access-string "on ~* +@all" \
  --authentication-mode Type=no-password \
  --endpoint-url http://localhost:6379 \
  --no-sign-request

# Create an ACL
aws memorydb create-acl \
  --acl-name my-acl \
  --user-names my-user \
  --endpoint-url http://localhost:6379 \
  --no-sign-request

# Create a cluster
aws memorydb create-cluster \
  --cluster-name my-cluster \
  --node-type db.t4g.small \
  --acl-name my-acl \
  --subnet-group-name my-subnet-group \
  --endpoint-url http://localhost:6379 \
  --no-sign-request

# Describe clusters
aws memorydb describe-clusters \
  --endpoint-url http://localhost:6379 \
  --no-sign-request

# Create a snapshot
aws memorydb create-snapshot \
  --cluster-name my-cluster \
  --snapshot-name my-snapshot \
  --endpoint-url http://localhost:6379 \
  --no-sign-request

# List tags
aws memorydb list-tags \
  --resource-arn arn:aws:memorydb:us-east-1:000000000000:cluster/my-cluster \
  --endpoint-url http://localhost:6379 \
  --no-sign-request

# Delete a cluster
aws memorydb delete-cluster \
  --cluster-name my-cluster \
  --endpoint-url http://localhost:6379 \
  --no-sign-request
```

### Limitations

- Clusters are created with simulated metadata but no actual Redis instances are started.
- Snapshots are metadata-only; no actual data is captured.
- Users and ACLs are stored but authentication is not enforced.

---

## Redshift

| | |
|---|---|
| **Port** | `10060` |
| **Protocol** | Query / XML |
| **Endpoint** | `http://localhost:10060` |

### Supported Operations (3)

| Operation | Description |
|-----------|-------------|
| CreateCluster | Create a new Redshift cluster |
| DescribeClusters | Describe one or more Redshift clusters |
| DeleteCluster | Delete a Redshift cluster |

### Usage with AWS CLI

```bash
# Create a cluster
aws redshift create-cluster \
  --cluster-identifier my-redshift \
  --node-type dc2.large \
  --master-username admin \
  --master-user-password Secret123 \
  --endpoint-url http://localhost:10060 \
  --no-sign-request

# Describe clusters
aws redshift describe-clusters \
  --endpoint-url http://localhost:10060 \
  --no-sign-request

# Delete a cluster
aws redshift delete-cluster \
  --cluster-identifier my-redshift \
  --skip-final-cluster-snapshot \
  --endpoint-url http://localhost:10060 \
  --no-sign-request
```
