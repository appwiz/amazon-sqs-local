# Internet of Things (IoT)

This category covers AWS IoT services for connecting and managing devices. All services store state in memory with no persistence.

---

## IoT Core

| | |
|---|---|
| **Port** | `10117` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10117` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateThing | Create an IoT thing |
| GetThing | Get details of an IoT thing |
| ListThings | List all IoT things |
| DeleteThing | Delete an IoT thing |

### Usage with AWS CLI

```bash
# Create a thing
aws iot create-thing \
  --thing-name my-device \
  --endpoint-url http://localhost:10117 \
  --no-sign-request

# List things
aws iot list-things \
  --endpoint-url http://localhost:10117 \
  --no-sign-request

# Delete a thing
aws iot delete-thing \
  --thing-name my-device \
  --endpoint-url http://localhost:10117 \
  --no-sign-request
```

---

## IoT Events

| | |
|---|---|
| **Port** | `10118` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10118` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateDetectorModel | Create a detector model |
| GetDetectorModel | Get details of a detector model |
| ListDetectorModels | List all detector models |
| DeleteDetectorModel | Delete a detector model |

### Usage with AWS CLI

```bash
# List detector models
aws iotevents list-detector-models \
  --endpoint-url http://localhost:10118 \
  --no-sign-request
```

---

## IoT SiteWise

| | |
|---|---|
| **Port** | `10121` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10121` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateAsset | Create a SiteWise asset |
| GetAsset | Get details of a SiteWise asset |
| ListAssets | List all SiteWise assets |
| DeleteAsset | Delete a SiteWise asset |

### Usage with AWS CLI

```bash
# List assets
aws iotsitewise list-assets \
  --endpoint-url http://localhost:10121 \
  --no-sign-request
```

---

## IoT TwinMaker

| | |
|---|---|
| **Port** | `10122` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10122` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateWorkspace | Create a TwinMaker workspace |
| GetWorkspace | Get details of a TwinMaker workspace |
| ListWorkspaces | List all TwinMaker workspaces |
| DeleteWorkspace | Delete a TwinMaker workspace |

### Usage with AWS CLI

```bash
# List workspaces
aws iottwinmaker list-workspaces \
  --endpoint-url http://localhost:10122 \
  --no-sign-request
```

---

## IoT Greengrass

| | |
|---|---|
| **Port** | `10120` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10120` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateComponent | Create a Greengrass component |
| GetComponent | Get details of a Greengrass component |
| ListComponents | List all Greengrass components |
| DeleteComponent | Delete a Greengrass component |

### Usage with AWS CLI

```bash
# List components
aws greengrassv2 list-components \
  --endpoint-url http://localhost:10120 \
  --no-sign-request
```

---

## IoT FleetWise

| | |
|---|---|
| **Port** | `10119` |
| **Protocol** | JSON RPC (`IoTAutobahnControlPlane`) |
| **Endpoint** | `http://localhost:10119` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateVehicle | Create a FleetWise vehicle |
| DescribeVehicle | Describe a FleetWise vehicle |
| ListVehicles | List all FleetWise vehicles |
| DeleteVehicle | Delete a FleetWise vehicle |

### Usage with AWS CLI

```bash
# List vehicles
aws iotfleetwise list-vehicles \
  --endpoint-url http://localhost:10119 \
  --no-sign-request
```
