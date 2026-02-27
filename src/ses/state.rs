use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use super::error::SesError;
use super::types::*;

struct EmailIdentity {
    identity_name: String,
    identity_type: String, // EMAIL_ADDRESS or DOMAIN
    tags: HashMap<String, String>,
    verified: bool,
}

struct SesStateInner {
    identities: HashMap<String, EmailIdentity>,
    sent_emails: Vec<StoredEmail>,
}

pub struct SesState {
    inner: Arc<Mutex<SesStateInner>>,
}

impl SesState {
    pub fn new(_account_id: String, _region: String) -> Self {
        SesState {
            inner: Arc::new(Mutex::new(SesStateInner {
                identities: HashMap::new(),
                sent_emails: Vec::new(),
            })),
        }
    }

    fn identity_type(name: &str) -> &'static str {
        if name.contains('@') { "EMAIL_ADDRESS" } else { "DOMAIN" }
    }

    pub async fn send_email(&self, _req: SendEmailRequest) -> Result<SendEmailResponse, SesError> {
        let mut state = self.inner.lock().await;
        let message_id = format!("010{}@email.amazonses.com", Uuid::new_v4().to_string().replace('-', ""));
        state.sent_emails.push(StoredEmail {});
        Ok(SendEmailResponse { message_id })
    }

    pub async fn create_email_identity(
        &self,
        name: String,
        req: CreateEmailIdentityRequest,
    ) -> Result<CreateEmailIdentityResponse, SesError> {
        let mut state = self.inner.lock().await;
        if state.identities.contains_key(&name) {
            return Err(SesError::AlreadyExistsException(format!(
                "Identity {} already exists", name
            )));
        }
        let identity_type = Self::identity_type(&name);
        let mut tags = HashMap::new();
        if let Some(t) = req.tags {
            for tag in t { tags.insert(tag.key, tag.value); }
        }
        state.identities.insert(name.clone(), EmailIdentity {
            identity_name: name,
            identity_type: identity_type.to_string(),
            tags,
            verified: true, // auto-verify in local mode
        });
        Ok(CreateEmailIdentityResponse {
            identity_type: identity_type.to_string(),
            verified_for_sending_status: true,
            dkim_attributes: DkimAttributes {
                signing_enabled: true,
                status: "SUCCESS".to_string(),
                tokens: vec!["token1".to_string(), "token2".to_string(), "token3".to_string()],
            },
        })
    }

    pub async fn delete_email_identity(&self, name: String) -> Result<(), SesError> {
        let mut state = self.inner.lock().await;
        if state.identities.remove(&name).is_none() {
            return Err(SesError::NotFoundException(format!("Identity {} not found", name)));
        }
        Ok(())
    }

    pub async fn get_email_identity(&self, name: String) -> Result<GetEmailIdentityResponse, SesError> {
        let state = self.inner.lock().await;
        let identity = state.identities.get(&name)
            .ok_or_else(|| SesError::NotFoundException(format!("Identity {} not found", name)))?;
        let tags: Vec<Tag> = identity.tags.iter().map(|(k, v)| Tag {
            key: k.clone(),
            value: v.clone(),
        }).collect();
        Ok(GetEmailIdentityResponse {
            identity_type: identity.identity_type.clone(),
            feedback_forwarding_status: true,
            verified_for_sending_status: identity.verified,
            dkim_attributes: DkimAttributes {
                signing_enabled: true,
                status: "SUCCESS".to_string(),
                tokens: vec!["token1".to_string(), "token2".to_string(), "token3".to_string()],
            },
            tags,
        })
    }

    pub async fn list_email_identities(
        &self,
        page_size: Option<usize>,
    ) -> Result<ListEmailIdentitiesResponse, SesError> {
        let state = self.inner.lock().await;
        let mut identities: Vec<IdentityInfo> = state.identities.values().map(|i| IdentityInfo {
            identity_type: i.identity_type.clone(),
            identity_name: i.identity_name.clone(),
            sending_enabled: i.verified,
        }).collect();
        identities.sort_by(|a, b| a.identity_name.cmp(&b.identity_name));
        let limit = page_size.unwrap_or(1000);
        let has_more = identities.len() > limit;
        identities.truncate(limit);
        Ok(ListEmailIdentitiesResponse {
            email_identities: identities,
            next_token: if has_more { Some("next".to_string()) } else { None },
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = SesState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_send_email() {
        let state = SesState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = SendEmailRequest::default();
        let _ = state.send_email(req).await;
    }
    #[tokio::test]
    async fn test_delete_email_identity_not_found() {
        let state = SesState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_email_identity("nonexistent".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_get_email_identity_not_found() {
        let state = SesState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_email_identity("nonexistent".to_string()).await;
        assert!(result.is_err());
    }

    fn make_state() -> SesState {
        SesState::new("123456789012".to_string(), "us-east-1".to_string())
    }

    // --- Extended coverage: create_email_identity ---

    #[tokio::test]
    async fn test_create_email_identity_email() {
        let state = make_state();
        let result = state.create_email_identity(
            "user@example.com".to_string(),
            CreateEmailIdentityRequest { email_identity: "user@example.com".to_string(), tags: None },
        ).await.unwrap();
        assert_eq!(result.identity_type, "EMAIL_ADDRESS");
        assert!(result.verified_for_sending_status);
        assert_eq!(result.dkim_attributes.status, "SUCCESS");
    }

    #[tokio::test]
    async fn test_create_email_identity_domain() {
        let state = make_state();
        let result = state.create_email_identity(
            "example.com".to_string(),
            CreateEmailIdentityRequest { email_identity: "example.com".to_string(), tags: None },
        ).await.unwrap();
        assert_eq!(result.identity_type, "DOMAIN");
    }

    #[tokio::test]
    async fn test_create_email_identity_with_tags() {
        let state = make_state();
        state.create_email_identity(
            "user@example.com".to_string(),
            CreateEmailIdentityRequest {
                email_identity: "user@example.com".to_string(),
                tags: Some(vec![Tag { key: "env".to_string(), value: "test".to_string() }]),
            },
        ).await.unwrap();
        let identity = state.get_email_identity("user@example.com".to_string()).await.unwrap();
        assert_eq!(identity.tags.len(), 1);
        assert_eq!(identity.tags[0].key, "env");
    }

    #[tokio::test]
    async fn test_create_email_identity_duplicate() {
        let state = make_state();
        state.create_email_identity(
            "user@example.com".to_string(),
            CreateEmailIdentityRequest { email_identity: "user@example.com".to_string(), tags: None },
        ).await.unwrap();
        let result = state.create_email_identity(
            "user@example.com".to_string(),
            CreateEmailIdentityRequest { email_identity: "user@example.com".to_string(), tags: None },
        ).await;
        assert!(result.is_err());
    }

    // --- Extended coverage: delete_email_identity ---

    #[tokio::test]
    async fn test_delete_email_identity_success() {
        let state = make_state();
        state.create_email_identity(
            "user@example.com".to_string(),
            CreateEmailIdentityRequest { email_identity: "user@example.com".to_string(), tags: None },
        ).await.unwrap();
        assert!(state.delete_email_identity("user@example.com".to_string()).await.is_ok());
        assert!(state.get_email_identity("user@example.com".to_string()).await.is_err());
    }

    // --- Extended coverage: get_email_identity ---

    #[tokio::test]
    async fn test_get_email_identity_success() {
        let state = make_state();
        state.create_email_identity(
            "user@example.com".to_string(),
            CreateEmailIdentityRequest { email_identity: "user@example.com".to_string(), tags: None },
        ).await.unwrap();
        let result = state.get_email_identity("user@example.com".to_string()).await.unwrap();
        assert_eq!(result.identity_type, "EMAIL_ADDRESS");
        assert!(result.verified_for_sending_status);
        assert!(result.feedback_forwarding_status);
    }

    // --- Extended coverage: list_email_identities ---

    #[tokio::test]
    async fn test_list_email_identities_empty() {
        let state = make_state();
        let result = state.list_email_identities(None).await.unwrap();
        assert!(result.email_identities.is_empty());
        assert!(result.next_token.is_none());
    }

    #[tokio::test]
    async fn test_list_email_identities_multiple() {
        let state = make_state();
        state.create_email_identity(
            "a@example.com".to_string(),
            CreateEmailIdentityRequest { email_identity: "a@example.com".to_string(), tags: None },
        ).await.unwrap();
        state.create_email_identity(
            "b@example.com".to_string(),
            CreateEmailIdentityRequest { email_identity: "b@example.com".to_string(), tags: None },
        ).await.unwrap();
        state.create_email_identity(
            "example.com".to_string(),
            CreateEmailIdentityRequest { email_identity: "example.com".to_string(), tags: None },
        ).await.unwrap();
        let result = state.list_email_identities(None).await.unwrap();
        assert_eq!(result.email_identities.len(), 3);
        // Sorted by name
        assert_eq!(result.email_identities[0].identity_name, "a@example.com");
    }

    #[tokio::test]
    async fn test_list_email_identities_with_page_size() {
        let state = make_state();
        state.create_email_identity(
            "a@example.com".to_string(),
            CreateEmailIdentityRequest { email_identity: "a@example.com".to_string(), tags: None },
        ).await.unwrap();
        state.create_email_identity(
            "b@example.com".to_string(),
            CreateEmailIdentityRequest { email_identity: "b@example.com".to_string(), tags: None },
        ).await.unwrap();
        state.create_email_identity(
            "c@example.com".to_string(),
            CreateEmailIdentityRequest { email_identity: "c@example.com".to_string(), tags: None },
        ).await.unwrap();
        let result = state.list_email_identities(Some(2)).await.unwrap();
        assert_eq!(result.email_identities.len(), 2);
        assert!(result.next_token.is_some());
    }

    // --- Extended coverage: send_email ---

    #[tokio::test]
    async fn test_send_email_returns_message_id() {
        let state = make_state();
        let result = state.send_email(SendEmailRequest::default()).await.unwrap();
        assert!(!result.message_id.is_empty());
        assert!(result.message_id.contains("@email.amazonses.com"));
    }
}
