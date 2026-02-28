# Media Services

## MediaConvert

| | |
|---|---|
| **Port** | `10134` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10134` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateJob | Create a new transcoding job |
| ListJobs | List all transcoding jobs |
| GetJob | Get details of a specific job |
| DeleteJob | Delete a transcoding job |

### Usage with AWS CLI

```bash
# Create
aws mediaconvert create-job --role arn:aws:iam::012345678901:role/role --settings '{}' --endpoint-url http://localhost:10134 --no-sign-request

# List
aws mediaconvert list-jobs --endpoint-url http://localhost:10134 --no-sign-request
```

---

## MediaLive

| | |
|---|---|
| **Port** | `10135` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10135` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateChannel | Create a new live channel |
| ListChannels | List all live channels |
| GetChannel | Get details of a specific channel |
| DeleteChannel | Delete a live channel |

### Usage with AWS CLI

```bash
# Create
aws medialive create-channel --name my-channel --endpoint-url http://localhost:10135 --no-sign-request

# List
aws medialive list-channels --endpoint-url http://localhost:10135 --no-sign-request
```

---

## MediaPackage

| | |
|---|---|
| **Port** | `10136` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10136` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateChannel | Create a new packaging channel |
| ListChannels | List all packaging channels |
| GetChannel | Get details of a specific channel |
| DeleteChannel | Delete a packaging channel |

### Usage with AWS CLI

```bash
# Create
aws mediapackage create-channel --id my-channel --endpoint-url http://localhost:10136 --no-sign-request

# List
aws mediapackage list-channels --endpoint-url http://localhost:10136 --no-sign-request
```

---

## MediaStore

| | |
|---|---|
| **Port** | `10137` |
| **Protocol** | JSON RPC (MediaStore_20170901) |
| **Endpoint** | `http://localhost:10137` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateContainer | Create a new media store container |
| DescribeContainer | Describe a specific container |
| ListContainers | List all containers |
| DeleteContainer | Delete a container |

### Usage with AWS CLI

```bash
# Create
aws mediastore create-container --container-name my-container --endpoint-url http://localhost:10137 --no-sign-request

# List
aws mediastore list-containers --endpoint-url http://localhost:10137 --no-sign-request
```

---

## IVS

| | |
|---|---|
| **Port** | `10133` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10133` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateChannel | Create a new IVS channel |
| ListChannels | List all IVS channels |
| GetChannel | Get details of a specific channel |
| DeleteChannel | Delete an IVS channel |

### Usage with AWS CLI

```bash
# Create
aws ivs create-channel --name my-channel --endpoint-url http://localhost:10133 --no-sign-request

# List
aws ivs list-channels --endpoint-url http://localhost:10133 --no-sign-request
```

---

## ElasticTranscoder

| | |
|---|---|
| **Port** | `10132` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10132` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreatePipeline | Create a new transcoding pipeline |
| ListPipelines | List all transcoding pipelines |
| GetPipeline | Get details of a specific pipeline |
| DeletePipeline | Delete a transcoding pipeline |

### Usage with AWS CLI

```bash
# Create
aws elastictranscoder create-pipeline --name my-pipeline --input-bucket my-input --role arn:aws:iam::012345678901:role/role --endpoint-url http://localhost:10132 --no-sign-request

# List
aws elastictranscoder list-pipelines --endpoint-url http://localhost:10132 --no-sign-request
```
