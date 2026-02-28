# Migration & Transfer

## DMS

| | |
|---|---|
| **Port** | `10018` |
| **Protocol** | JSON RPC (AmazonDMSv20160101) |
| **Endpoint** | `http://localhost:10018` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateReplicationInstance | Create a new replication instance |
| DescribeReplicationInstance | Describe a specific replication instance |
| ListReplicationInstances | List all replication instances |
| DeleteReplicationInstance | Delete a replication instance |

### Usage with AWS CLI

```bash
# Create
aws dms create-replication-instance --replication-instance-identifier my-instance --replication-instance-class dms.t3.medium --endpoint-url http://localhost:10018 --no-sign-request

# List
aws dms describe-replication-instances --endpoint-url http://localhost:10018 --no-sign-request
```

---

## DataSync

| | |
|---|---|
| **Port** | `10138` |
| **Protocol** | JSON RPC (FmrsService) |
| **Endpoint** | `http://localhost:10138` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateTask | Create a new data sync task |
| DescribeTask | Describe a specific task |
| ListTasks | List all tasks |
| DeleteTask | Delete a task |

### Usage with AWS CLI

```bash
# Create
aws datasync create-task --source-location-arn arn:aws:datasync:us-east-1:012345678901:location/loc-1 --destination-location-arn arn:aws:datasync:us-east-1:012345678901:location/loc-2 --endpoint-url http://localhost:10138 --no-sign-request

# List
aws datasync list-tasks --endpoint-url http://localhost:10138 --no-sign-request
```

---

## TransferFamily

| | |
|---|---|
| **Port** | `10141` |
| **Protocol** | JSON RPC (TransferService) |
| **Endpoint** | `http://localhost:10141` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateServer | Create a new transfer server |
| DescribeServer | Describe a specific server |
| ListServers | List all servers |
| DeleteServer | Delete a server |

### Usage with AWS CLI

```bash
# Create
aws transfer create-server --endpoint-url http://localhost:10141 --no-sign-request

# List
aws transfer list-servers --endpoint-url http://localhost:10141 --no-sign-request
```

---

## MigrationHub

| | |
|---|---|
| **Port** | `10140` |
| **Protocol** | JSON RPC (AWSMigrationHub) |
| **Endpoint** | `http://localhost:10140` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateProgressUpdateStream | Create a new progress update stream |
| DescribeProgressUpdateStream | Describe a specific progress update stream |
| ListProgressUpdateStreams | List all progress update streams |
| DeleteProgressUpdateStream | Delete a progress update stream |

### Usage with AWS CLI

```bash
# Create
aws mgh create-progress-update-stream --progress-update-stream-name my-stream --endpoint-url http://localhost:10140 --no-sign-request

# List
aws mgh list-progress-update-streams --endpoint-url http://localhost:10140 --no-sign-request
```

---

## MainframeMod

| | |
|---|---|
| **Port** | `10139` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10139` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateApplication | Create a new mainframe modernization application |
| ListApplications | List all applications |
| GetApplication | Get details of a specific application |
| DeleteApplication | Delete an application |

### Usage with AWS CLI

```bash
# Create
aws m2 create-application --name my-app --definition '{}' --engine-type microfocus --endpoint-url http://localhost:10139 --no-sign-request

# List
aws m2 list-applications --endpoint-url http://localhost:10139 --no-sign-request
```

---

## DRS

| | |
|---|---|
| **Port** | `10149` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10149` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateSourceServer | Create a new source server for disaster recovery |
| GetSourceServer | Get details of a specific source server |
| ListSourceServers | List all source servers |
| DeleteSourceServer | Delete a source server |

### Usage with AWS CLI

```bash
# Create
aws drs create-source-server --source-server-id my-server --endpoint-url http://localhost:10149 --no-sign-request

# List
aws drs describe-source-servers --endpoint-url http://localhost:10149 --no-sign-request
```
