use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::LicensemanagerError;
use super::types::*;

#[allow(dead_code)]
struct LicensemanagerStateInner {
    licenses: HashMap<String, StoredLicense>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredLicense {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct LicensemanagerState {
    inner: Arc<Mutex<LicensemanagerStateInner>>,
}

impl LicensemanagerState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        LicensemanagerState {
            inner: Arc::new(Mutex::new(LicensemanagerStateInner {
                licenses: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_license(&self, req: CreateLicenseRequest) -> Result<CreateLicenseResponse, LicensemanagerError> {
        let mut state = self.inner.lock().await;
        let name = req.license_name.clone();
        if state.licenses.contains_key(&name) {
            return Err(LicensemanagerError::ResourceAlreadyExistsException(format!("License {} already exists", name)));
        }
        let arn = format!("arn:aws:license-manager:{}:{}:licenses/{}", state.region, state.account_id, name);
        state.licenses.insert(name.clone(), StoredLicense {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateLicenseResponse {
            license_arn: Some(arn),
            license_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_license(&self, req: DescribeLicenseRequest) -> Result<DescribeLicenseResponse, LicensemanagerError> {
        let state = self.inner.lock().await;
        let name = req.license_name.or_else(|| req.license_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| LicensemanagerError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.licenses.get(&name)
            .ok_or_else(|| LicensemanagerError::ResourceNotFoundException(format!("License {} not found", name)))?;
        Ok(DescribeLicenseResponse {
            license: LicenseDetail {
                license_name: stored.name.clone(),
                license_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_licenses(&self, _req: ListLicensesRequest) -> Result<ListLicensesResponse, LicensemanagerError> {
        let state = self.inner.lock().await;
        let items: Vec<LicenseDetail> = state.licenses.values().map(|s| LicenseDetail {
            license_name: s.name.clone(),
            license_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListLicensesResponse {
            licenses: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_license(&self, req: DeleteLicenseRequest) -> Result<(), LicensemanagerError> {
        let mut state = self.inner.lock().await;
        let name = req.license_name.or_else(|| req.license_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| LicensemanagerError::ValidationException("Name or ARN required".to_string()))?;
        state.licenses.remove(&name)
            .ok_or_else(|| LicensemanagerError::ResourceNotFoundException(format!("License {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = LicensemanagerState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_license() {
        let state = LicensemanagerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateLicenseRequest::default();
        let result = state.create_license(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_license_not_found() {
        let state = LicensemanagerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeLicenseRequest::default();
        let result = state.describe_license(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_licenses_empty() {
        let state = LicensemanagerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListLicensesRequest::default();
        let result = state.list_licenses(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_license_not_found() {
        let state = LicensemanagerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteLicenseRequest::default();
        let result = state.delete_license(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_license_create_and_list() {
        let state = LicensemanagerState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateLicenseRequest::default();
        let _created = state.create_license(create_req).await.unwrap();
        let list_req = ListLicensesRequest::default();
        let listed = state.list_licenses(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_license_full_crud() {
        let state = LicensemanagerState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateLicenseRequest::default();
        create_req.license_name = "test-crud-resource".to_string();
        let create_result = state.create_license(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeLicenseRequest::default();
        get_req.license_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_license(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteLicenseRequest::default();
        del_req.license_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_license(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
