use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ACMError;
use super::types::*;

#[allow(dead_code)]
struct ACMStateInner {
    certificates: HashMap<String, StoredCertificate>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredCertificate {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct ACMState {
    inner: Arc<Mutex<ACMStateInner>>,
}

impl ACMState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ACMState {
            inner: Arc::new(Mutex::new(ACMStateInner {
                certificates: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_certificate(&self, req: CreateCertificateRequest) -> Result<CreateCertificateResponse, ACMError> {
        let mut state = self.inner.lock().await;
        let name = req.certificate_name.clone();
        if state.certificates.contains_key(&name) {
            return Err(ACMError::ResourceAlreadyExistsException(format!("Certificate {} already exists", name)));
        }
        let arn = format!("arn:aws:acm:{}:{}:certificates/{}", state.region, state.account_id, name);
        state.certificates.insert(name.clone(), StoredCertificate {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateCertificateResponse {
            certificate_arn: Some(arn),
            certificate_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_certificate(&self, req: DescribeCertificateRequest) -> Result<DescribeCertificateResponse, ACMError> {
        let state = self.inner.lock().await;
        let name = req.certificate_name.or_else(|| req.certificate_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ACMError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.certificates.get(&name)
            .ok_or_else(|| ACMError::ResourceNotFoundException(format!("Certificate {} not found", name)))?;
        Ok(DescribeCertificateResponse {
            certificate: CertificateDetail {
                certificate_name: stored.name.clone(),
                certificate_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_certificates(&self, _req: ListCertificatesRequest) -> Result<ListCertificatesResponse, ACMError> {
        let state = self.inner.lock().await;
        let items: Vec<CertificateDetail> = state.certificates.values().map(|s| CertificateDetail {
            certificate_name: s.name.clone(),
            certificate_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListCertificatesResponse {
            certificates: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_certificate(&self, req: DeleteCertificateRequest) -> Result<(), ACMError> {
        let mut state = self.inner.lock().await;
        let name = req.certificate_name.or_else(|| req.certificate_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ACMError::ValidationException("Name or ARN required".to_string()))?;
        state.certificates.remove(&name)
            .ok_or_else(|| ACMError::ResourceNotFoundException(format!("Certificate {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ACMState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_certificate() {
        let state = ACMState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateCertificateRequest::default();
        let result = state.create_certificate(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_certificate_not_found() {
        let state = ACMState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeCertificateRequest::default();
        let result = state.describe_certificate(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_certificates_empty() {
        let state = ACMState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListCertificatesRequest::default();
        let result = state.list_certificates(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_certificate_not_found() {
        let state = ACMState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteCertificateRequest::default();
        let result = state.delete_certificate(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_certificate_create_and_list() {
        let state = ACMState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateCertificateRequest::default();
        let _created = state.create_certificate(create_req).await.unwrap();
        let list_req = ListCertificatesRequest::default();
        let listed = state.list_certificates(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_certificate_full_crud() {
        let state = ACMState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateCertificateRequest::default();
        create_req.certificate_name = "test-crud-resource".to_string();
        let create_result = state.create_certificate(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeCertificateRequest::default();
        get_req.certificate_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_certificate(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteCertificateRequest::default();
        del_req.certificate_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_certificate(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
