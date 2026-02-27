use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use super::error::EfsError;
use super::types::*;

struct EfsStateInner {
    file_systems: HashMap<String, FileSystemDescription>,
    mount_targets: HashMap<String, MountTargetDescription>,
    access_points: HashMap<String, AccessPointDescription>,
    lifecycle_configs: HashMap<String, Vec<LifecyclePolicy>>,
    // Map from creation_token to file_system_id for idempotency
    creation_tokens: HashMap<String, String>,
    // Map from client_token to access_point_id for idempotency
    client_tokens: HashMap<String, String>,
    account_id: String,
    region: String,
}

pub struct EfsState {
    inner: Arc<Mutex<EfsStateInner>>,
}

impl EfsState {
    pub fn new(account_id: String, region: String) -> Self {
        EfsState {
            inner: Arc::new(Mutex::new(EfsStateInner {
                file_systems: HashMap::new(),
                mount_targets: HashMap::new(),
                access_points: HashMap::new(),
                lifecycle_configs: HashMap::new(),
                creation_tokens: HashMap::new(),
                client_tokens: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    fn generate_fs_id() -> String {
        let hex = Uuid::new_v4().to_string().replace('-', "");
        format!("fs-{}", &hex[..17])
    }

    fn generate_mount_target_id() -> String {
        let hex = Uuid::new_v4().to_string().replace('-', "");
        format!("fsmt-{}", &hex[..17])
    }

    fn generate_access_point_id() -> String {
        let hex = Uuid::new_v4().to_string().replace('-', "");
        format!("fsap-{}", &hex[..17])
    }

    fn now() -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }

    // --- File System operations ---

    pub async fn create_file_system(
        &self,
        req: CreateFileSystemRequest,
    ) -> Result<FileSystemDescription, EfsError> {
        let mut state = self.inner.lock().await;

        // CreationToken idempotency: if already exists, return existing
        if let Some(fs_id) = state.creation_tokens.get(&req.creation_token) {
            if let Some(fs) = state.file_systems.get(fs_id) {
                return Ok(fs.clone());
            }
        }

        let fs_id = Self::generate_fs_id();
        let fs_arn = format!(
            "arn:aws:elasticfilesystem:{}:{}:file-system/{}",
            state.region, state.account_id, fs_id
        );

        let mut name = None;
        let tags = req.tags.unwrap_or_default();
        for tag in &tags {
            if tag.key == "Name" {
                name = Some(tag.value.clone());
            }
        }

        let fs = FileSystemDescription {
            owner_id: state.account_id.clone(),
            creation_token: req.creation_token.clone(),
            file_system_id: fs_id.clone(),
            file_system_arn: fs_arn,
            creation_time: Self::now(),
            life_cycle_state: "available".to_string(),
            name,
            number_of_mount_targets: 0,
            size_in_bytes: SizeInBytes {
                value: 0,
                timestamp: Some(Self::now()),
                value_in_ia: Some(0),
                value_in_standard: Some(0),
            },
            performance_mode: req.performance_mode.unwrap_or_else(|| "generalPurpose".to_string()),
            encrypted: req.encrypted.unwrap_or(false),
            kms_key_id: req.kms_key_id,
            throughput_mode: req.throughput_mode.unwrap_or_else(|| "bursting".to_string()),
            provisioned_throughput_in_mibps: req.provisioned_throughput_in_mibps,
            tags,
        };

        state.creation_tokens.insert(req.creation_token.clone(), fs_id.clone());
        state.file_systems.insert(fs_id, fs.clone());
        Ok(fs)
    }

    pub async fn describe_file_systems(
        &self,
        file_system_id: Option<String>,
        creation_token: Option<String>,
    ) -> Result<DescribeFileSystemsResponse, EfsError> {
        let state = self.inner.lock().await;

        let mut systems: Vec<FileSystemDescription> = if let Some(ref id) = file_system_id {
            match state.file_systems.get(id) {
                Some(fs) => vec![fs.clone()],
                None => {
                    return Err(EfsError::FileSystemNotFound(format!(
                        "File system '{}' does not exist.",
                        id
                    )));
                }
            }
        } else if let Some(ref token) = creation_token {
            state
                .file_systems
                .values()
                .filter(|fs| fs.creation_token == *token)
                .cloned()
                .collect()
        } else {
            state.file_systems.values().cloned().collect()
        };

        systems.sort_by(|a, b| a.creation_time.partial_cmp(&b.creation_time).unwrap());

        Ok(DescribeFileSystemsResponse {
            file_systems: systems,
            next_marker: None,
        })
    }

    pub async fn update_file_system(
        &self,
        file_system_id: String,
        req: UpdateFileSystemRequest,
    ) -> Result<FileSystemDescription, EfsError> {
        let mut state = self.inner.lock().await;
        let fs = state.file_systems.get_mut(&file_system_id).ok_or_else(|| {
            EfsError::FileSystemNotFound(format!(
                "File system '{}' does not exist.",
                file_system_id
            ))
        })?;

        if let Some(mode) = req.throughput_mode {
            fs.throughput_mode = mode;
        }
        if req.provisioned_throughput_in_mibps.is_some() {
            fs.provisioned_throughput_in_mibps = req.provisioned_throughput_in_mibps;
        }

        Ok(fs.clone())
    }

    pub async fn delete_file_system(&self, file_system_id: String) -> Result<(), EfsError> {
        let mut state = self.inner.lock().await;

        if !state.file_systems.contains_key(&file_system_id) {
            return Err(EfsError::FileSystemNotFound(format!(
                "File system '{}' does not exist.",
                file_system_id
            )));
        }

        // Check for existing mount targets
        let has_mount_targets = state
            .mount_targets
            .values()
            .any(|mt| mt.file_system_id == file_system_id);
        if has_mount_targets {
            return Err(EfsError::FileSystemInUse(format!(
                "File system '{}' has mount targets and cannot be deleted.",
                file_system_id
            )));
        }

        // Remove associated access points
        let ap_ids: Vec<String> = state
            .access_points
            .iter()
            .filter(|(_, ap)| ap.file_system_id == file_system_id)
            .map(|(id, _)| id.clone())
            .collect();
        for id in &ap_ids {
            if let Some(ap) = state.access_points.remove(id) {
                state.client_tokens.remove(&ap.client_token);
            }
        }

        // Remove lifecycle config
        state.lifecycle_configs.remove(&file_system_id);

        let fs = state.file_systems.remove(&file_system_id).unwrap();
        state.creation_tokens.remove(&fs.creation_token);
        Ok(())
    }

    // --- Mount Target operations ---

    pub async fn create_mount_target(
        &self,
        req: CreateMountTargetRequest,
    ) -> Result<MountTargetDescription, EfsError> {
        let mut state = self.inner.lock().await;

        if !state.file_systems.contains_key(&req.file_system_id) {
            return Err(EfsError::FileSystemNotFound(format!(
                "File system '{}' does not exist.",
                req.file_system_id
            )));
        }

        // Check for duplicate mount target in same subnet
        let dup = state.mount_targets.values().any(|mt| {
            mt.file_system_id == req.file_system_id && mt.subnet_id == req.subnet_id
        });
        if dup {
            return Err(EfsError::MountTargetConflict(format!(
                "Mount target already exists in subnet '{}'.",
                req.subnet_id
            )));
        }

        let mt_id = Self::generate_mount_target_id();
        let ip = req
            .ip_address
            .unwrap_or_else(|| "10.0.0.1".to_string());

        let mt = MountTargetDescription {
            owner_id: state.account_id.clone(),
            mount_target_id: mt_id.clone(),
            file_system_id: req.file_system_id.clone(),
            subnet_id: req.subnet_id,
            life_cycle_state: "available".to_string(),
            ip_address: Some(ip),
            network_interface_id: Some(format!("eni-{}", &Uuid::new_v4().to_string().replace('-', "")[..17])),
            availability_zone_id: Some("use1-az1".to_string()),
            availability_zone_name: Some(format!("{}a", state.region)),
            vpc_id: Some("vpc-00000000".to_string()),
        };

        // Increment mount target count on file system
        if let Some(fs) = state.file_systems.get_mut(&req.file_system_id) {
            fs.number_of_mount_targets += 1;
        }

        state.mount_targets.insert(mt_id, mt.clone());
        Ok(mt)
    }

    pub async fn describe_mount_targets(
        &self,
        mount_target_id: Option<String>,
        file_system_id: Option<String>,
    ) -> Result<DescribeMountTargetsResponse, EfsError> {
        let state = self.inner.lock().await;

        let targets: Vec<MountTargetDescription> = if let Some(ref id) = mount_target_id {
            match state.mount_targets.get(id) {
                Some(mt) => vec![mt.clone()],
                None => {
                    return Err(EfsError::MountTargetNotFound(format!(
                        "Mount target '{}' does not exist.",
                        id
                    )));
                }
            }
        } else if let Some(ref fs_id) = file_system_id {
            if !state.file_systems.contains_key(fs_id) {
                return Err(EfsError::FileSystemNotFound(format!(
                    "File system '{}' does not exist.",
                    fs_id
                )));
            }
            state
                .mount_targets
                .values()
                .filter(|mt| mt.file_system_id == *fs_id)
                .cloned()
                .collect()
        } else {
            state.mount_targets.values().cloned().collect()
        };

        Ok(DescribeMountTargetsResponse {
            mount_targets: targets,
            next_marker: None,
        })
    }

    pub async fn delete_mount_target(&self, mount_target_id: String) -> Result<(), EfsError> {
        let mut state = self.inner.lock().await;
        let mt = state.mount_targets.remove(&mount_target_id).ok_or_else(|| {
            EfsError::MountTargetNotFound(format!(
                "Mount target '{}' does not exist.",
                mount_target_id
            ))
        })?;

        // Decrement mount target count on file system
        if let Some(fs) = state.file_systems.get_mut(&mt.file_system_id) {
            fs.number_of_mount_targets -= 1;
        }

        Ok(())
    }

    // --- Access Point operations ---

    pub async fn create_access_point(
        &self,
        req: CreateAccessPointRequest,
    ) -> Result<AccessPointDescription, EfsError> {
        let mut state = self.inner.lock().await;

        if !state.file_systems.contains_key(&req.file_system_id) {
            return Err(EfsError::FileSystemNotFound(format!(
                "File system '{}' does not exist.",
                req.file_system_id
            )));
        }

        // ClientToken idempotency
        if let Some(ap_id) = state.client_tokens.get(&req.client_token) {
            if let Some(ap) = state.access_points.get(ap_id) {
                return Ok(ap.clone());
            }
        }

        let ap_id = Self::generate_access_point_id();
        let ap_arn = format!(
            "arn:aws:elasticfilesystem:{}:{}:access-point/{}",
            state.region, state.account_id, ap_id
        );

        let mut name = None;
        let tags = req.tags.unwrap_or_default();
        for tag in &tags {
            if tag.key == "Name" {
                name = Some(tag.value.clone());
            }
        }

        let ap = AccessPointDescription {
            client_token: req.client_token.clone(),
            name,
            tags,
            access_point_id: ap_id.clone(),
            access_point_arn: ap_arn,
            file_system_id: req.file_system_id,
            posix_user: req.posix_user,
            root_directory: req.root_directory,
            owner_id: state.account_id.clone(),
            life_cycle_state: "available".to_string(),
        };

        state.client_tokens.insert(req.client_token, ap_id.clone());
        state.access_points.insert(ap_id, ap.clone());
        Ok(ap)
    }

    pub async fn describe_access_points(
        &self,
        access_point_id: Option<String>,
        file_system_id: Option<String>,
    ) -> Result<DescribeAccessPointsResponse, EfsError> {
        let state = self.inner.lock().await;

        let points: Vec<AccessPointDescription> = if let Some(ref id) = access_point_id {
            match state.access_points.get(id) {
                Some(ap) => vec![ap.clone()],
                None => {
                    return Err(EfsError::AccessPointNotFound(format!(
                        "Access point '{}' does not exist.",
                        id
                    )));
                }
            }
        } else if let Some(ref fs_id) = file_system_id {
            if !state.file_systems.contains_key(fs_id) {
                return Err(EfsError::FileSystemNotFound(format!(
                    "File system '{}' does not exist.",
                    fs_id
                )));
            }
            state
                .access_points
                .values()
                .filter(|ap| ap.file_system_id == *fs_id)
                .cloned()
                .collect()
        } else {
            state.access_points.values().cloned().collect()
        };

        Ok(DescribeAccessPointsResponse {
            access_points: points,
            next_token: None,
        })
    }

    pub async fn delete_access_point(&self, access_point_id: String) -> Result<(), EfsError> {
        let mut state = self.inner.lock().await;
        let ap = state
            .access_points
            .remove(&access_point_id)
            .ok_or_else(|| {
                EfsError::AccessPointNotFound(format!(
                    "Access point '{}' does not exist.",
                    access_point_id
                ))
            })?;
        state.client_tokens.remove(&ap.client_token);
        Ok(())
    }

    // --- Tag operations ---

    pub async fn tag_resource(
        &self,
        resource_id: String,
        req: TagResourceRequest,
    ) -> Result<(), EfsError> {
        let mut state = self.inner.lock().await;

        // Find the resource and update its tags
        if let Some(fs) = state.file_systems.get_mut(&resource_id) {
            for new_tag in &req.tags {
                if let Some(existing) = fs.tags.iter_mut().find(|t| t.key == new_tag.key) {
                    existing.value = new_tag.value.clone();
                } else {
                    fs.tags.push(new_tag.clone());
                }
            }
            // Update name if Name tag changed
            fs.name = fs.tags.iter().find(|t| t.key == "Name").map(|t| t.value.clone());
            return Ok(());
        }

        if state.access_points.contains_key(&resource_id) {
            let ap = state.access_points.get_mut(&resource_id).unwrap();
            for new_tag in &req.tags {
                if let Some(existing) = ap.tags.iter_mut().find(|t| t.key == new_tag.key) {
                    existing.value = new_tag.value.clone();
                } else {
                    ap.tags.push(new_tag.clone());
                }
            }
            ap.name = ap.tags.iter().find(|t| t.key == "Name").map(|t| t.value.clone());
            return Ok(());
        }

        Err(EfsError::FileSystemNotFound(format!(
            "Resource '{}' does not exist.",
            resource_id
        )))
    }

    pub async fn untag_resource(
        &self,
        resource_id: String,
        req: UntagResourceRequest,
    ) -> Result<(), EfsError> {
        let mut state = self.inner.lock().await;

        if let Some(fs) = state.file_systems.get_mut(&resource_id) {
            fs.tags.retain(|t| !req.tag_keys.contains(&t.key));
            fs.name = fs.tags.iter().find(|t| t.key == "Name").map(|t| t.value.clone());
            return Ok(());
        }

        if state.access_points.contains_key(&resource_id) {
            let ap = state.access_points.get_mut(&resource_id).unwrap();
            ap.tags.retain(|t| !req.tag_keys.contains(&t.key));
            ap.name = ap.tags.iter().find(|t| t.key == "Name").map(|t| t.value.clone());
            return Ok(());
        }

        Err(EfsError::FileSystemNotFound(format!(
            "Resource '{}' does not exist.",
            resource_id
        )))
    }

    pub async fn list_tags_for_resource(
        &self,
        resource_id: String,
    ) -> Result<ListTagsForResourceResponse, EfsError> {
        let state = self.inner.lock().await;

        if let Some(fs) = state.file_systems.get(&resource_id) {
            return Ok(ListTagsForResourceResponse {
                tags: fs.tags.clone(),
                next_token: None,
            });
        }

        if let Some(ap) = state.access_points.get(&resource_id) {
            return Ok(ListTagsForResourceResponse {
                tags: ap.tags.clone(),
                next_token: None,
            });
        }

        Err(EfsError::FileSystemNotFound(format!(
            "Resource '{}' does not exist.",
            resource_id
        )))
    }

    // --- Lifecycle Configuration operations ---

    pub async fn put_lifecycle_configuration(
        &self,
        file_system_id: String,
        req: PutLifecycleConfigurationRequest,
    ) -> Result<LifecycleConfigurationDescription, EfsError> {
        let mut state = self.inner.lock().await;

        if !state.file_systems.contains_key(&file_system_id) {
            return Err(EfsError::FileSystemNotFound(format!(
                "File system '{}' does not exist.",
                file_system_id
            )));
        }

        state
            .lifecycle_configs
            .insert(file_system_id, req.lifecycle_policies.clone());

        Ok(LifecycleConfigurationDescription {
            lifecycle_policies: req.lifecycle_policies,
        })
    }

    pub async fn describe_lifecycle_configuration(
        &self,
        file_system_id: String,
    ) -> Result<LifecycleConfigurationDescription, EfsError> {
        let state = self.inner.lock().await;

        if !state.file_systems.contains_key(&file_system_id) {
            return Err(EfsError::FileSystemNotFound(format!(
                "File system '{}' does not exist.",
                file_system_id
            )));
        }

        let policies = state
            .lifecycle_configs
            .get(&file_system_id)
            .cloned()
            .unwrap_or_default();

        Ok(LifecycleConfigurationDescription {
            lifecycle_policies: policies,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state() -> EfsState {
        EfsState::new("123456789012".to_string(), "us-east-1".to_string())
    }

    async fn create_fs(state: &EfsState) -> FileSystemDescription {
        let req = CreateFileSystemRequest {
            creation_token: format!("token-{}", uuid::Uuid::new_v4()),
            ..Default::default()
        };
        state.create_file_system(req).await.unwrap()
    }

    #[tokio::test]
    async fn test_new_state() {
        let _state = make_state();
    }

    #[tokio::test]
    async fn test_create_file_system() {
        let state = make_state();
        let fs = create_fs(&state).await;
        assert!(fs.file_system_id.starts_with("fs-"));
        assert_eq!(fs.life_cycle_state, "available");
    }

    #[tokio::test]
    async fn test_create_file_system_idempotent() {
        let state = make_state();
        let req = CreateFileSystemRequest {
            creation_token: "my-token".to_string(),
            ..Default::default()
        };
        let fs1 = state.create_file_system(req.clone()).await.unwrap();
        let fs2 = state.create_file_system(req).await.unwrap();
        assert_eq!(fs1.file_system_id, fs2.file_system_id);
    }

    #[tokio::test]
    async fn test_describe_file_systems() {
        let state = make_state();
        create_fs(&state).await;
        let result = state.describe_file_systems(None, None).await.unwrap();
        assert_eq!(result.file_systems.len(), 1);
    }

    #[tokio::test]
    async fn test_describe_file_systems_by_id() {
        let state = make_state();
        let fs = create_fs(&state).await;
        let result = state.describe_file_systems(Some(fs.file_system_id.clone()), None).await.unwrap();
        assert_eq!(result.file_systems.len(), 1);
    }

    #[tokio::test]
    async fn test_delete_file_system() {
        let state = make_state();
        let fs = create_fs(&state).await;
        assert!(state.delete_file_system(fs.file_system_id.clone()).await.is_ok());
        let result = state.describe_file_systems(None, None).await.unwrap();
        assert!(result.file_systems.is_empty());
    }

    #[tokio::test]
    async fn test_delete_file_system_not_found() {
        let state = make_state();
        assert!(state.delete_file_system("fs-nonexistent".to_string()).await.is_err());
    }

    #[tokio::test]
    async fn test_update_file_system() {
        let state = make_state();
        let fs = create_fs(&state).await;
        let req = UpdateFileSystemRequest {
            throughput_mode: Some("provisioned".to_string()),
            provisioned_throughput_in_mibps: Some(100.0),
        };
        let result = state.update_file_system(fs.file_system_id.clone(), req).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_mount_target() {
        let state = make_state();
        let fs = create_fs(&state).await;
        let req = CreateMountTargetRequest {
            file_system_id: fs.file_system_id.clone(),
            subnet_id: "subnet-12345".to_string(),
            ..Default::default()
        };
        let result = state.create_mount_target(req).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_mount_targets() {
        let state = make_state();
        let fs = create_fs(&state).await;
        state.create_mount_target(CreateMountTargetRequest {
            file_system_id: fs.file_system_id.clone(),
            subnet_id: "subnet-12345".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.describe_mount_targets(None, Some(fs.file_system_id.clone())).await.unwrap();
        assert_eq!(result.mount_targets.len(), 1);
    }

    #[tokio::test]
    async fn test_delete_mount_target() {
        let state = make_state();
        let fs = create_fs(&state).await;
        let mt = state.create_mount_target(CreateMountTargetRequest {
            file_system_id: fs.file_system_id.clone(),
            subnet_id: "subnet-12345".to_string(),
            ..Default::default()
        }).await.unwrap();
        assert!(state.delete_mount_target(mt.mount_target_id.clone()).await.is_ok());
    }

    #[tokio::test]
    async fn test_create_access_point() {
        let state = make_state();
        let fs = create_fs(&state).await;
        let req = CreateAccessPointRequest {
            client_token: "ap-token".to_string(),
            file_system_id: fs.file_system_id.clone(),
            ..Default::default()
        };
        let result = state.create_access_point(req).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_access_points() {
        let state = make_state();
        let fs = create_fs(&state).await;
        state.create_access_point(CreateAccessPointRequest {
            client_token: "ap-token".to_string(),
            file_system_id: fs.file_system_id.clone(),
            ..Default::default()
        }).await.unwrap();
        let result = state.describe_access_points(None, None).await.unwrap();
        assert_eq!(result.access_points.len(), 1);
    }

    #[tokio::test]
    async fn test_tag_and_list_tags() {
        let state = make_state();
        let fs = create_fs(&state).await;
        let tag_req = TagResourceRequest {
            tags: vec![Tag { key: "env".to_string(), value: "test".to_string() }],
        };
        state.tag_resource(fs.file_system_id.clone(), tag_req).await.unwrap();
        let result = state.list_tags_for_resource(fs.file_system_id.clone()).await.unwrap();
        // Tags include the ones from creation plus the new one
        assert!(result.tags.iter().any(|t| t.key == "env"));
    }

    #[tokio::test]
    async fn test_put_lifecycle_configuration() {
        let state = make_state();
        let fs = create_fs(&state).await;
        let req = PutLifecycleConfigurationRequest {
            lifecycle_policies: vec![LifecyclePolicy {
                transition_to_ia: Some("AFTER_30_DAYS".to_string()),
                transition_to_primary_storage_class: None,
                transition_to_archive: None,
            }],
        };
        assert!(state.put_lifecycle_configuration(fs.file_system_id.clone(), req).await.is_ok());
        let result = state.describe_lifecycle_configuration(fs.file_system_id.clone()).await.unwrap();
        assert_eq!(result.lifecycle_policies.len(), 1);
    }
}
