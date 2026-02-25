use serde::{Deserialize, Serialize};

// SES v2 types

#[derive(Debug, Deserialize)]
pub struct SendEmailRequest {
}

#[derive(Debug, Serialize)]
pub struct SendEmailResponse {
    #[serde(rename = "MessageId")]
    pub message_id: String,
}

// Email Identity
#[derive(Debug, Deserialize)]
pub struct CreateEmailIdentityRequest {
    #[serde(rename = "EmailIdentity")]
    pub email_identity: String,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Serialize)]
pub struct CreateEmailIdentityResponse {
    #[serde(rename = "IdentityType")]
    pub identity_type: String,
    #[serde(rename = "VerifiedForSendingStatus")]
    pub verified_for_sending_status: bool,
    #[serde(rename = "DkimAttributes")]
    pub dkim_attributes: DkimAttributes,
}

#[derive(Debug, Serialize)]
pub struct DkimAttributes {
    #[serde(rename = "SigningEnabled")]
    pub signing_enabled: bool,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tokens")]
    pub tokens: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ListEmailIdentitiesResponse {
    #[serde(rename = "EmailIdentities")]
    pub email_identities: Vec<IdentityInfo>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct IdentityInfo {
    #[serde(rename = "IdentityType")]
    pub identity_type: String,
    #[serde(rename = "IdentityName")]
    pub identity_name: String,
    #[serde(rename = "SendingEnabled")]
    pub sending_enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct GetEmailIdentityResponse {
    #[serde(rename = "IdentityType")]
    pub identity_type: String,
    #[serde(rename = "FeedbackForwardingStatus")]
    pub feedback_forwarding_status: bool,
    #[serde(rename = "VerifiedForSendingStatus")]
    pub verified_for_sending_status: bool,
    #[serde(rename = "DkimAttributes")]
    pub dkim_attributes: DkimAttributes,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

// Stored email record
#[derive(Debug, Clone)]
pub struct StoredEmail {
}
