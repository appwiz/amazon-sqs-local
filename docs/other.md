# Other Services

## Amplify

| | |
|---|---|
| **Port** | `10154` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10154` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateApp | Create a new Amplify app |
| GetApp | Get details of a specific app |
| ListApps | List all apps |
| DeleteApp | Delete an app |

### Usage with AWS CLI

```bash
# Create
aws amplify create-app --name my-app --endpoint-url http://localhost:10154 --no-sign-request

# List
aws amplify list-apps --endpoint-url http://localhost:10154 --no-sign-request
```

---

## AppFlow

| | |
|---|---|
| **Port** | `10112` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10112` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateFlow | Create a new data flow |
| GetFlow | Get details of a specific flow |
| ListFlows | List all flows |
| DeleteFlow | Delete a flow |

### Usage with AWS CLI

```bash
# Create
aws appflow create-flow --flow-name my-flow --trigger-config '{"triggerType":"OnDemand"}' --source-flow-config '{}' --destination-flow-config-list '[]' --tasks '[]' --endpoint-url http://localhost:10112 --no-sign-request

# List
aws appflow list-flows --endpoint-url http://localhost:10112 --no-sign-request
```

---

## AppFabric

| | |
|---|---|
| **Port** | `10128` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10128` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateAppBundle | Create a new app bundle |
| GetAppBundle | Get details of a specific app bundle |
| ListAppBundles | List all app bundles |
| DeleteAppBundle | Delete an app bundle |

### Usage with AWS CLI

```bash
# Create
aws appfabric create-app-bundle --endpoint-url http://localhost:10128 --no-sign-request

# List
aws appfabric list-app-bundles --endpoint-url http://localhost:10128 --no-sign-request
```

---

## B2BI

| | |
|---|---|
| **Port** | `10116` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10116` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateProfile | Create a new B2B integration profile |
| GetProfile | Get details of a specific profile |
| ListProfiles | List all profiles |
| DeleteProfile | Delete a profile |

### Usage with AWS CLI

```bash
# Create
aws b2bi create-profile --name my-profile --phone 555-0100 --business-name my-business --logging ENABLED --endpoint-url http://localhost:10116 --no-sign-request

# List
aws b2bi list-profiles --endpoint-url http://localhost:10116 --no-sign-request
```

---

## Braket

| | |
|---|---|
| **Port** | `10150` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10150` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateQuantumTask | Create a new quantum computing task |
| GetQuantumTask | Get details of a specific quantum task |
| ListQuantumTasks | List all quantum tasks |
| DeleteQuantumTask | Delete a quantum task |

### Usage with AWS CLI

```bash
# Create
aws braket create-quantum-task --device-arn arn:aws:braket:::device/quantum-simulator/amazon/sv1 --output-s3-bucket my-bucket --output-s3-key-prefix results --shots 100 --action '{}' --endpoint-url http://localhost:10150 --no-sign-request

# List
aws braket search-quantum-tasks --filters '[]' --endpoint-url http://localhost:10150 --no-sign-request
```

---

## QBusiness

| | |
|---|---|
| **Port** | `10108` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10108` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateApplication | Create a new Q Business application |
| GetApplication | Get details of a specific application |
| ListApplications | List all applications |
| DeleteApplication | Delete an application |

### Usage with AWS CLI

```bash
# Create
aws qbusiness create-application --display-name my-app --endpoint-url http://localhost:10108 --no-sign-request

# List
aws qbusiness list-applications --endpoint-url http://localhost:10108 --no-sign-request
```

---

## DevOpsGuru

| | |
|---|---|
| **Port** | `10106` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10106` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateInsight | Create a new operational insight |
| GetInsight | Get details of a specific insight |
| ListInsights | List all insights |
| DeleteInsight | Delete an insight |

### Usage with AWS CLI

```bash
# Create
aws devops-guru describe-account-overview --from-time 2024-01-01T00:00:00Z --endpoint-url http://localhost:10106 --no-sign-request

# List
aws devops-guru list-insights --status-filter '{"Any":{"Type":"REACTIVE","StartTimeRange":{"FromTime":"2024-01-01T00:00:00Z"}}}' --endpoint-url http://localhost:10106 --no-sign-request
```

---

## DeviceFarm

| | |
|---|---|
| **Port** | `10155` |
| **Protocol** | JSON RPC (DeviceFarm_20150623) |
| **Endpoint** | `http://localhost:10155` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateProject | Create a new device testing project |
| DescribeProject | Describe a specific project |
| ListProjects | List all projects |
| DeleteProject | Delete a project |

### Usage with AWS CLI

```bash
# Create
aws devicefarm create-project --name my-project --endpoint-url http://localhost:10155 --no-sign-request

# List
aws devicefarm list-projects --endpoint-url http://localhost:10155 --no-sign-request
```

---

## GroundStation

| | |
|---|---|
| **Port** | `10151` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10151` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateConfig | Create a new ground station config |
| GetConfig | Get details of a specific config |
| ListConfigs | List all configs |
| DeleteConfig | Delete a config |

### Usage with AWS CLI

```bash
# Create
aws groundstation create-config --name my-config --config-data '{"antennaDownlinkConfig":{"spectrumConfig":{"bandwidth":{"units":"MHz","value":10},"centerFrequency":{"units":"MHz","value":2100}}}}' --endpoint-url http://localhost:10151 --no-sign-request

# List
aws groundstation list-configs --endpoint-url http://localhost:10151 --no-sign-request
```

---

## Location

| | |
|---|---|
| **Port** | `10153` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10153` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateMap | Create a new map resource |
| GetMap | Get details of a specific map |
| ListMaps | List all maps |
| DeleteMap | Delete a map |

### Usage with AWS CLI

```bash
# Create
aws location create-map --map-name my-map --configuration '{"style":"VectorEsriStreets"}' --endpoint-url http://localhost:10153 --no-sign-request

# List
aws location list-maps --endpoint-url http://localhost:10153 --no-sign-request
```

---

## ManagedBlockchain

| | |
|---|---|
| **Port** | `10157` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10157` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateNetwork | Create a new blockchain network |
| GetNetwork | Get details of a specific network |
| ListNetworks | List all networks |
| DeleteNetwork | Delete a network |

### Usage with AWS CLI

```bash
# Create
aws managedblockchain create-network --name my-network --framework HYPERLEDGER_FABRIC --framework-version 2.2 --framework-configuration '{}' --voting-policy '{"ApprovalThresholdPolicy":{"ThresholdPercentage":50,"ProposalDurationInHours":24,"ThresholdComparator":"GREATER_THAN"}}' --member-configuration '{"Name":"my-member","FrameworkConfiguration":{"Fabric":{"AdminUsername":"admin","AdminPassword":"Password123"}}}' --endpoint-url http://localhost:10157 --no-sign-request

# List
aws managedblockchain list-networks --endpoint-url http://localhost:10157 --no-sign-request
```
