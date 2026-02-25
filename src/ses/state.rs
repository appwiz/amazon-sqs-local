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
