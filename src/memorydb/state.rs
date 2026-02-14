use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::cluster::*;
use super::error::MemoryDbError;
use super::types::*;

struct MemoryDbStateInner {
    clusters: HashMap<String, Cluster>,
    subnet_groups: HashMap<String, SubnetGroup>,
    users: HashMap<String, User>,
    acls: HashMap<String, Acl>,
    snapshots: HashMap<String, Snapshot>,
    tags: HashMap<String, Vec<Tag>>,
    account_id: String,
    region: String,
}

pub struct MemoryDbState {
    inner: Arc<Mutex<MemoryDbStateInner>>,
}

impl MemoryDbState {
    pub fn new(account_id: String, region: String) -> Self {
        MemoryDbState {
            inner: Arc::new(Mutex::new(MemoryDbStateInner {
                clusters: HashMap::new(),
                subnet_groups: HashMap::new(),
                users: HashMap::new(),
                acls: HashMap::new(),
                snapshots: HashMap::new(),
                tags: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    // --- Cluster operations ---

    pub async fn create_cluster(
        &self,
        req: CreateClusterRequest,
    ) -> Result<CreateClusterResponse, MemoryDbError> {
        let mut state = self.inner.lock().await;

        if state.clusters.contains_key(&req.cluster_name) {
            return Err(MemoryDbError::ClusterAlreadyExistsFault(format!(
                "Cluster {} already exists",
                req.cluster_name
            )));
        }

        let arn = format!(
            "arn:aws:memorydb:{}:{}:cluster/{}",
            state.region, state.account_id, req.cluster_name
        );

        let engine = req.engine.unwrap_or_else(|| "redis".to_string());
        let engine_version = req.engine_version.unwrap_or_else(|| "7.1".to_string());
        let num_shards = req.num_shards.unwrap_or(1);
        let tls_enabled = req.tls_enabled.unwrap_or(true);
        let region_short = state.region.replace('-', "");

        let cluster_endpoint = Endpoint {
            address: format!(
                "{}.xxxxx.0001.{}.cache.amazonaws.com",
                req.cluster_name, region_short
            ),
            port: 6379,
        };

        let node_endpoint = Endpoint {
            address: format!(
                "{}-0001-001.xxxxx.0001.{}.cache.amazonaws.com",
                req.cluster_name, region_short
            ),
            port: 6379,
        };

        let node = Node {
            name: format!("{}-0001-001", req.cluster_name),
            status: "available".to_string(),
            availability_zone: Some(format!("{}a", state.region)),
            create_time: Some(chrono::Utc::now().to_rfc3339()),
            endpoint: Some(node_endpoint),
        };

        let shard = Shard {
            name: "0001".to_string(),
            status: "available".to_string(),
            slots: Some("0-16383".to_string()),
            number_of_nodes: 1,
            nodes: Some(vec![node]),
        };

        let security_groups = req.security_group_ids.map(|ids| {
            ids.into_iter()
                .map(|id| SecurityGroupMembership {
                    security_group_id: id,
                    status: "active".to_string(),
                })
                .collect()
        });

        let parameter_group_name = req
            .parameter_group_name
            .unwrap_or_else(|| "default.memorydb-redis7".to_string());

        let cluster = Cluster {
            name: req.cluster_name.clone(),
            arn: arn.clone(),
            status: "available".to_string(),
            description: req.description,
            node_type: req.node_type,
            engine,
            engine_version,
            number_of_shards: num_shards,
            acl_name: req.acl_name,
            subnet_group_name: req.subnet_group_name,
            tls_enabled,
            kms_key_id: req.kms_key_id,
            sns_topic_arn: req.sns_topic_arn,
            maintenance_window: req.maintenance_window,
            parameter_group_name,
            parameter_group_status: "in-sync".to_string(),
            security_groups,
            shards: Some(vec![shard]),
            cluster_endpoint: Some(cluster_endpoint),
            auto_minor_version_upgrade: req.auto_minor_version_upgrade.unwrap_or(true),
            snapshot_retention_limit: req.snapshot_retention_limit.unwrap_or(0),
            snapshot_window: req.snapshot_window,
        };

        if let Some(tags) = req.tags {
            state.tags.insert(arn.clone(), tags);
        }

        state.clusters.insert(req.cluster_name.clone(), cluster.clone());

        Ok(CreateClusterResponse { cluster })
    }

    pub async fn delete_cluster(
        &self,
        req: DeleteClusterRequest,
    ) -> Result<DeleteClusterResponse, MemoryDbError> {
        let mut state = self.inner.lock().await;

        let cluster = state
            .clusters
            .remove(&req.cluster_name)
            .ok_or_else(|| {
                MemoryDbError::ClusterNotFoundFault(format!(
                    "Cluster {} not found",
                    req.cluster_name
                ))
            })?;

        // Remove tags for this cluster
        state.tags.remove(&cluster.arn);

        let mut deleted_cluster = cluster;
        deleted_cluster.status = "deleting".to_string();

        Ok(DeleteClusterResponse {
            cluster: deleted_cluster,
        })
    }

    pub async fn describe_clusters(
        &self,
        req: DescribeClustersRequest,
    ) -> Result<DescribeClustersResponse, MemoryDbError> {
        let state = self.inner.lock().await;

        let clusters: Vec<Cluster> = if let Some(name) = &req.cluster_name {
            match state.clusters.get(name) {
                Some(c) => vec![c.clone()],
                None => {
                    return Err(MemoryDbError::ClusterNotFoundFault(format!(
                        "Cluster {} not found",
                        name
                    )));
                }
            }
        } else {
            let mut all: Vec<Cluster> = state.clusters.values().cloned().collect();
            all.sort_by(|a, b| a.name.cmp(&b.name));
            all
        };

        let max_results = req.max_results.unwrap_or(100) as usize;
        let start = req
            .next_token
            .as_ref()
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(0);

        let end = (start + max_results).min(clusters.len());
        let page = clusters[start..end].to_vec();
        let next_token = if end < clusters.len() {
            Some(end.to_string())
        } else {
            None
        };

        Ok(DescribeClustersResponse {
            clusters: page,
            next_token,
        })
    }

    pub async fn update_cluster(
        &self,
        req: UpdateClusterRequest,
    ) -> Result<UpdateClusterResponse, MemoryDbError> {
        let mut state = self.inner.lock().await;

        let cluster = state
            .clusters
            .get_mut(&req.cluster_name)
            .ok_or_else(|| {
                MemoryDbError::ClusterNotFoundFault(format!(
                    "Cluster {} not found",
                    req.cluster_name
                ))
            })?;

        if let Some(desc) = req.description {
            cluster.description = Some(desc);
        }
        if let Some(node_type) = req.node_type {
            cluster.node_type = node_type;
        }
        if let Some(ev) = req.engine_version {
            cluster.engine_version = ev;
        }
        if let Some(acl) = req.acl_name {
            cluster.acl_name = acl;
        }
        if let Some(mw) = req.maintenance_window {
            cluster.maintenance_window = Some(mw);
        }
        if let Some(sns) = req.sns_topic_arn {
            cluster.sns_topic_arn = Some(sns);
        }
        if let Some(pg) = req.parameter_group_name {
            cluster.parameter_group_name = pg;
        }
        if let Some(srl) = req.snapshot_retention_limit {
            cluster.snapshot_retention_limit = srl;
        }
        if let Some(sw) = req.snapshot_window {
            cluster.snapshot_window = Some(sw);
        }
        if let Some(sg_ids) = req.security_group_ids {
            cluster.security_groups = Some(
                sg_ids
                    .into_iter()
                    .map(|id| SecurityGroupMembership {
                        security_group_id: id,
                        status: "active".to_string(),
                    })
                    .collect(),
            );
        }

        let cluster = cluster.clone();
        Ok(UpdateClusterResponse { cluster })
    }

    // --- SubnetGroup operations ---

    pub async fn create_subnet_group(
        &self,
        req: CreateSubnetGroupRequest,
    ) -> Result<CreateSubnetGroupResponse, MemoryDbError> {
        let mut state = self.inner.lock().await;

        if state.subnet_groups.contains_key(&req.subnet_group_name) {
            return Err(MemoryDbError::SubnetGroupAlreadyExistsFault(format!(
                "Subnet group {} already exists",
                req.subnet_group_name
            )));
        }

        let arn = format!(
            "arn:aws:memorydb:{}:{}:subnetgroup/{}",
            state.region, state.account_id, req.subnet_group_name
        );

        let subnets = req.subnet_ids.map(|ids| {
            ids.into_iter()
                .map(|id| Subnet {
                    identifier: id,
                    availability_zone: Some(AvailabilityZone {
                        name: format!("{}a", state.region),
                    }),
                })
                .collect()
        });

        let subnet_group = SubnetGroup {
            name: req.subnet_group_name.clone(),
            arn: arn.clone(),
            description: req.description,
            vpc_id: "vpc-00000000".to_string(),
            subnets,
        };

        if let Some(tags) = req.tags {
            state.tags.insert(arn.clone(), tags);
        }

        state
            .subnet_groups
            .insert(req.subnet_group_name.clone(), subnet_group.clone());

        Ok(CreateSubnetGroupResponse { subnet_group })
    }

    pub async fn delete_subnet_group(
        &self,
        req: DeleteSubnetGroupRequest,
    ) -> Result<DeleteSubnetGroupResponse, MemoryDbError> {
        let mut state = self.inner.lock().await;

        // Check if any cluster uses this subnet group
        for cluster in state.clusters.values() {
            if cluster.subnet_group_name.as_deref() == Some(&req.subnet_group_name) {
                return Err(MemoryDbError::SubnetGroupInUseFault(format!(
                    "Subnet group {} is in use by cluster {}",
                    req.subnet_group_name, cluster.name
                )));
            }
        }

        let subnet_group = state
            .subnet_groups
            .remove(&req.subnet_group_name)
            .ok_or_else(|| {
                MemoryDbError::SubnetGroupNotFoundFault(format!(
                    "Subnet group {} not found",
                    req.subnet_group_name
                ))
            })?;

        state.tags.remove(&subnet_group.arn);

        Ok(DeleteSubnetGroupResponse { subnet_group })
    }

    pub async fn describe_subnet_groups(
        &self,
        req: DescribeSubnetGroupsRequest,
    ) -> Result<DescribeSubnetGroupsResponse, MemoryDbError> {
        let state = self.inner.lock().await;

        let groups: Vec<SubnetGroup> = if let Some(name) = &req.subnet_group_name {
            match state.subnet_groups.get(name) {
                Some(sg) => vec![sg.clone()],
                None => {
                    return Err(MemoryDbError::SubnetGroupNotFoundFault(format!(
                        "Subnet group {} not found",
                        name
                    )));
                }
            }
        } else {
            let mut all: Vec<SubnetGroup> = state.subnet_groups.values().cloned().collect();
            all.sort_by(|a, b| a.name.cmp(&b.name));
            all
        };

        let max_results = req.max_results.unwrap_or(100) as usize;
        let start = req
            .next_token
            .as_ref()
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(0);

        let end = (start + max_results).min(groups.len());
        let page = groups[start..end].to_vec();
        let next_token = if end < groups.len() {
            Some(end.to_string())
        } else {
            None
        };

        Ok(DescribeSubnetGroupsResponse {
            subnet_groups: page,
            next_token,
        })
    }

    // --- User operations ---

    pub async fn create_user(
        &self,
        req: CreateUserRequest,
    ) -> Result<CreateUserResponse, MemoryDbError> {
        let mut state = self.inner.lock().await;

        if state.users.contains_key(&req.user_name) {
            return Err(MemoryDbError::UserAlreadyExistsFault(format!(
                "User {} already exists",
                req.user_name
            )));
        }

        let arn = format!(
            "arn:aws:memorydb:{}:{}:user/{}",
            state.region, state.account_id, req.user_name
        );

        let password_count = req
            .authentication_mode
            .passwords
            .as_ref()
            .map(|p| p.len() as i32)
            .unwrap_or(0);

        let authentication = Authentication {
            auth_type: req.authentication_mode.auth_type,
            password_count: Some(password_count),
        };

        let user = User {
            name: req.user_name.clone(),
            arn: arn.clone(),
            status: "active".to_string(),
            access_string: req.access_string,
            authentication: Some(authentication),
            acl_names: Some(vec![]),
            minimum_engine_version: "6.0.0".to_string(),
        };

        if let Some(tags) = req.tags {
            state.tags.insert(arn.clone(), tags);
        }

        state.users.insert(req.user_name.clone(), user.clone());

        Ok(CreateUserResponse { user })
    }

    pub async fn delete_user(
        &self,
        req: DeleteUserRequest,
    ) -> Result<DeleteUserResponse, MemoryDbError> {
        let mut state = self.inner.lock().await;

        let user = state.users.remove(&req.user_name).ok_or_else(|| {
            MemoryDbError::UserNotFoundFault(format!("User {} not found", req.user_name))
        })?;

        state.tags.remove(&user.arn);

        // Remove user from any ACLs
        for acl in state.acls.values_mut() {
            if let Some(ref mut user_names) = acl.user_names {
                user_names.retain(|n| n != &req.user_name);
            }
        }

        let mut deleted_user = user;
        deleted_user.status = "deleting".to_string();

        Ok(DeleteUserResponse {
            user: deleted_user,
        })
    }

    pub async fn describe_users(
        &self,
        req: DescribeUsersRequest,
    ) -> Result<DescribeUsersResponse, MemoryDbError> {
        let state = self.inner.lock().await;

        let users: Vec<User> = if let Some(name) = &req.user_name {
            match state.users.get(name) {
                Some(u) => vec![u.clone()],
                None => {
                    return Err(MemoryDbError::UserNotFoundFault(format!(
                        "User {} not found",
                        name
                    )));
                }
            }
        } else {
            let mut all: Vec<User> = state.users.values().cloned().collect();
            all.sort_by(|a, b| a.name.cmp(&b.name));
            all
        };

        let max_results = req.max_results.unwrap_or(100) as usize;
        let start = req
            .next_token
            .as_ref()
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(0);

        let end = (start + max_results).min(users.len());
        let page = users[start..end].to_vec();
        let next_token = if end < users.len() {
            Some(end.to_string())
        } else {
            None
        };

        Ok(DescribeUsersResponse {
            users: page,
            next_token,
        })
    }

    pub async fn update_user(
        &self,
        req: UpdateUserRequest,
    ) -> Result<UpdateUserResponse, MemoryDbError> {
        let mut state = self.inner.lock().await;

        let user = state.users.get_mut(&req.user_name).ok_or_else(|| {
            MemoryDbError::UserNotFoundFault(format!("User {} not found", req.user_name))
        })?;

        if let Some(access_string) = req.access_string {
            user.access_string = access_string;
        }

        if let Some(auth_mode) = req.authentication_mode {
            let password_count = auth_mode
                .passwords
                .as_ref()
                .map(|p| p.len() as i32)
                .unwrap_or(0);
            user.authentication = Some(Authentication {
                auth_type: auth_mode.auth_type,
                password_count: Some(password_count),
            });
        }

        let user = user.clone();
        Ok(UpdateUserResponse { user })
    }

    // --- ACL operations ---

    pub async fn create_acl(
        &self,
        req: CreateAclRequest,
    ) -> Result<CreateAclResponse, MemoryDbError> {
        let mut state = self.inner.lock().await;

        if state.acls.contains_key(&req.acl_name) {
            return Err(MemoryDbError::ACLAlreadyExistsFault(format!(
                "ACL {} already exists",
                req.acl_name
            )));
        }

        let arn = format!(
            "arn:aws:memorydb:{}:{}:acl/{}",
            state.region, state.account_id, req.acl_name
        );

        let user_names = req.user_names.unwrap_or_default();

        // Update users' acl_names
        for uname in &user_names {
            if let Some(user) = state.users.get_mut(uname) {
                if let Some(ref mut acl_names) = user.acl_names {
                    if !acl_names.contains(&req.acl_name) {
                        acl_names.push(req.acl_name.clone());
                    }
                }
            }
        }

        let acl = Acl {
            name: req.acl_name.clone(),
            arn: arn.clone(),
            status: "active".to_string(),
            user_names: Some(user_names),
            minimum_engine_version: "6.0.0".to_string(),
            clusters: Some(vec![]),
        };

        if let Some(tags) = req.tags {
            state.tags.insert(arn.clone(), tags);
        }

        state.acls.insert(req.acl_name.clone(), acl.clone());

        Ok(CreateAclResponse { acl })
    }

    pub async fn delete_acl(
        &self,
        req: DeleteAclRequest,
    ) -> Result<DeleteAclResponse, MemoryDbError> {
        let mut state = self.inner.lock().await;

        let acl = state.acls.remove(&req.acl_name).ok_or_else(|| {
            MemoryDbError::ACLNotFoundFault(format!("ACL {} not found", req.acl_name))
        })?;

        state.tags.remove(&acl.arn);

        // Remove ACL from users' acl_names
        if let Some(ref user_names) = acl.user_names {
            for uname in user_names {
                if let Some(user) = state.users.get_mut(uname) {
                    if let Some(ref mut acl_names) = user.acl_names {
                        acl_names.retain(|n| n != &req.acl_name);
                    }
                }
            }
        }

        let mut deleted_acl = acl;
        deleted_acl.status = "deleting".to_string();

        Ok(DeleteAclResponse { acl: deleted_acl })
    }

    pub async fn describe_acls(
        &self,
        req: DescribeAclsRequest,
    ) -> Result<DescribeAclsResponse, MemoryDbError> {
        let state = self.inner.lock().await;

        let acls: Vec<Acl> = if let Some(name) = &req.acl_name {
            match state.acls.get(name) {
                Some(a) => vec![a.clone()],
                None => {
                    return Err(MemoryDbError::ACLNotFoundFault(format!(
                        "ACL {} not found",
                        name
                    )));
                }
            }
        } else {
            let mut all: Vec<Acl> = state.acls.values().cloned().collect();
            all.sort_by(|a, b| a.name.cmp(&b.name));
            all
        };

        let max_results = req.max_results.unwrap_or(100) as usize;
        let start = req
            .next_token
            .as_ref()
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(0);

        let end = (start + max_results).min(acls.len());
        let page = acls[start..end].to_vec();
        let next_token = if end < acls.len() {
            Some(end.to_string())
        } else {
            None
        };

        Ok(DescribeAclsResponse {
            acls: page,
            next_token,
        })
    }

    pub async fn update_acl(
        &self,
        req: UpdateAclRequest,
    ) -> Result<UpdateAclResponse, MemoryDbError> {
        let mut state = self.inner.lock().await;

        if !state.acls.contains_key(&req.acl_name) {
            return Err(MemoryDbError::ACLNotFoundFault(format!(
                "ACL {} not found",
                req.acl_name
            )));
        }

        // Collect the add/remove lists first
        let mut added_users = Vec::new();
        let mut removed_users = Vec::new();

        {
            let acl = state.acls.get_mut(&req.acl_name).unwrap();
            let user_names = acl.user_names.get_or_insert_with(Vec::new);

            if let Some(to_add) = &req.user_names_to_add {
                for uname in to_add {
                    if !user_names.contains(uname) {
                        user_names.push(uname.clone());
                        added_users.push(uname.clone());
                    }
                }
            }

            if let Some(to_remove) = &req.user_names_to_remove {
                for uname in to_remove {
                    if user_names.contains(uname) {
                        removed_users.push(uname.clone());
                    }
                    user_names.retain(|n| n != uname);
                }
            }
        }

        // Now update user references
        for uname in &added_users {
            if let Some(user) = state.users.get_mut(uname) {
                if let Some(ref mut acl_names) = user.acl_names {
                    if !acl_names.contains(&req.acl_name) {
                        acl_names.push(req.acl_name.clone());
                    }
                }
            }
        }

        for uname in &removed_users {
            if let Some(user) = state.users.get_mut(uname) {
                if let Some(ref mut acl_names) = user.acl_names {
                    acl_names.retain(|n| n != &req.acl_name);
                }
            }
        }

        let acl = state.acls.get(&req.acl_name).unwrap().clone();
        Ok(UpdateAclResponse { acl })
    }

    // --- Snapshot operations ---

    pub async fn create_snapshot(
        &self,
        req: CreateSnapshotRequest,
    ) -> Result<CreateSnapshotResponse, MemoryDbError> {
        let mut state = self.inner.lock().await;

        if state.snapshots.contains_key(&req.snapshot_name) {
            return Err(MemoryDbError::SnapshotAlreadyExistsFault(format!(
                "Snapshot {} already exists",
                req.snapshot_name
            )));
        }

        let cluster = state
            .clusters
            .get(&req.cluster_name)
            .ok_or_else(|| {
                MemoryDbError::ClusterNotFoundFault(format!(
                    "Cluster {} not found",
                    req.cluster_name
                ))
            })?
            .clone();

        let arn = format!(
            "arn:aws:memorydb:{}:{}:snapshot/{}",
            state.region, state.account_id, req.snapshot_name
        );

        let cluster_config = ClusterConfiguration {
            name: cluster.name.clone(),
            description: cluster.description.clone(),
            node_type: cluster.node_type.clone(),
            engine_version: cluster.engine_version.clone(),
            maintenance_window: cluster.maintenance_window.clone(),
            snapshot_retention_limit: cluster.snapshot_retention_limit,
            subnet_group_name: cluster.subnet_group_name.clone(),
            vpc_id: None,
            number_of_shards: cluster.number_of_shards,
            shards: Some(vec![ShardDetail {
                name: "0001".to_string(),
                size: Some("0".to_string()),
                snapshot_creation_time: chrono::Utc::now().to_rfc3339(),
            }]),
        };

        let snapshot = Snapshot {
            name: req.snapshot_name.clone(),
            arn: arn.clone(),
            status: "available".to_string(),
            cluster_configuration: Some(cluster_config),
            source: Some(cluster.name),
        };

        if let Some(tags) = req.tags {
            state.tags.insert(arn.clone(), tags);
        }

        state
            .snapshots
            .insert(req.snapshot_name.clone(), snapshot.clone());

        Ok(CreateSnapshotResponse { snapshot })
    }

    pub async fn delete_snapshot(
        &self,
        req: DeleteSnapshotRequest,
    ) -> Result<DeleteSnapshotResponse, MemoryDbError> {
        let mut state = self.inner.lock().await;

        let snapshot = state
            .snapshots
            .remove(&req.snapshot_name)
            .ok_or_else(|| {
                MemoryDbError::SnapshotNotFoundFault(format!(
                    "Snapshot {} not found",
                    req.snapshot_name
                ))
            })?;

        state.tags.remove(&snapshot.arn);

        let mut deleted_snapshot = snapshot;
        deleted_snapshot.status = "deleting".to_string();

        Ok(DeleteSnapshotResponse {
            snapshot: deleted_snapshot,
        })
    }

    pub async fn describe_snapshots(
        &self,
        req: DescribeSnapshotsRequest,
    ) -> Result<DescribeSnapshotsResponse, MemoryDbError> {
        let state = self.inner.lock().await;

        let snapshots: Vec<Snapshot> = if let Some(name) = &req.snapshot_name {
            match state.snapshots.get(name) {
                Some(s) => vec![s.clone()],
                None => {
                    return Err(MemoryDbError::SnapshotNotFoundFault(format!(
                        "Snapshot {} not found",
                        name
                    )));
                }
            }
        } else {
            let mut all: Vec<Snapshot> = state.snapshots.values().cloned().collect();
            if let Some(cluster_name) = &req.cluster_name {
                all.retain(|s| s.source.as_deref() == Some(cluster_name.as_str()));
            }
            all.sort_by(|a, b| a.name.cmp(&b.name));
            all
        };

        let max_results = req.max_results.unwrap_or(100) as usize;
        let start = req
            .next_token
            .as_ref()
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(0);

        let end = (start + max_results).min(snapshots.len());
        let page = snapshots[start..end].to_vec();
        let next_token = if end < snapshots.len() {
            Some(end.to_string())
        } else {
            None
        };

        Ok(DescribeSnapshotsResponse {
            snapshots: page,
            next_token,
        })
    }

    // --- Tag operations ---

    fn find_resource_arn<'a>(state: &'a MemoryDbStateInner, arn: &str) -> bool {
        state.clusters.values().any(|c| c.arn == arn)
            || state.subnet_groups.values().any(|s| s.arn == arn)
            || state.users.values().any(|u| u.arn == arn)
            || state.acls.values().any(|a| a.arn == arn)
            || state.snapshots.values().any(|s| s.arn == arn)
    }

    pub async fn tag_resource(
        &self,
        req: TagResourceRequest,
    ) -> Result<TagResourceResponse, MemoryDbError> {
        let mut state = self.inner.lock().await;

        if !Self::find_resource_arn(&state, &req.resource_arn) {
            return Err(MemoryDbError::InvalidARNFault(format!(
                "Resource {} not found",
                req.resource_arn
            )));
        }

        let tags = state
            .tags
            .entry(req.resource_arn.clone())
            .or_insert_with(Vec::new);

        for new_tag in req.tags {
            if let Some(existing) = tags.iter_mut().find(|t| t.key == new_tag.key) {
                existing.value = new_tag.value;
            } else {
                tags.push(new_tag);
            }
        }

        let tag_list = tags.clone();

        Ok(TagResourceResponse { tag_list })
    }

    pub async fn untag_resource(
        &self,
        req: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, MemoryDbError> {
        let mut state = self.inner.lock().await;

        if !Self::find_resource_arn(&state, &req.resource_arn) {
            return Err(MemoryDbError::InvalidARNFault(format!(
                "Resource {} not found",
                req.resource_arn
            )));
        }

        let tags = state
            .tags
            .entry(req.resource_arn.clone())
            .or_insert_with(Vec::new);

        tags.retain(|t| !req.tag_keys.contains(&t.key));

        let tag_list = tags.clone();

        Ok(UntagResourceResponse { tag_list })
    }

    pub async fn list_tags(
        &self,
        req: ListTagsRequest,
    ) -> Result<ListTagsResponse, MemoryDbError> {
        let state = self.inner.lock().await;

        if !Self::find_resource_arn(&state, &req.resource_arn) {
            return Err(MemoryDbError::InvalidARNFault(format!(
                "Resource {} not found",
                req.resource_arn
            )));
        }

        let tag_list = state
            .tags
            .get(&req.resource_arn)
            .cloned()
            .unwrap_or_default();

        Ok(ListTagsResponse { tag_list })
    }
}
