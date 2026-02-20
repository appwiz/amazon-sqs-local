use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// SES v2 types

#[derive(Debug, Deserialize)]
pub struct SendEmailRequest {
    #[serde(rename = "FromEmailAddress")]
    pub from_email_address: Option<String>,
    #[serde(rename = "Destination")]
    pub destination: Option<Destination>,
    #[serde(rename = "Content")]
    pub content: EmailContent,
    #[serde(rename = "ReplyToAddresses")]
    pub reply_to_addresses: Option<Vec<String>>,
    #[serde(rename = "FeedbackForwardingEmailAddress")]
    pub feedback_forwarding_email_address: Option<String>,
    #[serde(rename = "EmailTags")]
    pub email_tags: Option<Vec<MessageTag>>,
    #[serde(rename = "ConfigurationSetName")]
    pub configuration_set_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Destination {
    #[serde(rename = "ToAddresses")]
    pub to_addresses: Option<Vec<String>>,
    #[serde(rename = "CcAddresses")]
    pub cc_addresses: Option<Vec<String>>,
    #[serde(rename = "BccAddresses")]
    pub bcc_addresses: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct EmailContent {
    #[serde(rename = "Simple")]
    pub simple: Option<Message>,
    #[serde(rename = "Raw")]
    pub raw: Option<RawMessage>,
    #[serde(rename = "Template")]
    pub template: Option<Template>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    #[serde(rename = "Subject")]
    pub subject: Content,
    #[serde(rename = "Body")]
    pub body: Body,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "Charset")]
    pub charset: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Body {
    #[serde(rename = "Text")]
    pub text: Option<Content>,
    #[serde(rename = "Html")]
    pub html: Option<Content>,
}

#[derive(Debug, Deserialize)]
pub struct RawMessage {
    #[serde(rename = "Data")]
    pub data: String, // base64
}

#[derive(Debug, Deserialize)]
pub struct Template {
    #[serde(rename = "TemplateName")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateArn")]
    pub template_arn: Option<String>,
    #[serde(rename = "TemplateData")]
    pub template_data: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MessageTag {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,
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

#[derive(Debug, Deserialize)]
pub struct DeleteEmailIdentityRequest {
    // Email identity is in path param
}

#[derive(Debug, Deserialize, Default)]
pub struct ListEmailIdentitiesRequest {
    #[serde(rename = "PageSize")]
    pub page_size: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
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
    pub message_id: String,
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub body: String,
    pub timestamp: f64,
}

pub type TagMap = HashMap<String, String>;
