mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateEnvironmentTemplateRequest {
    #[serde(rename = "EnvironmentTemplateName")]
    pub environment_template_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateEnvironmentTemplateResponse {
    #[serde(rename = "EnvironmentTemplateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_template_arn: Option<String>,
    #[serde(rename = "EnvironmentTemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_template_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeEnvironmentTemplateRequest {
    #[serde(rename = "EnvironmentTemplateName")]
    pub environment_template_name: Option<String>,
    #[serde(rename = "EnvironmentTemplateArn")]
    pub environment_template_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct EnvironmentTemplateDetail {
    #[serde(rename = "EnvironmentTemplateName")]
    pub environment_template_name: String,
    #[serde(rename = "EnvironmentTemplateArn")]
    pub environment_template_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeEnvironmentTemplateResponse {
    #[serde(rename = "EnvironmentTemplate")]
    pub environment_template: EnvironmentTemplateDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListEnvironmentTemplatesRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListEnvironmentTemplatesResponse {
    #[serde(rename = "EnvironmentTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_templates: Option<Vec<EnvironmentTemplateDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteEnvironmentTemplateRequest {
    #[serde(rename = "EnvironmentTemplateName")]
    pub environment_template_name: Option<String>,
    #[serde(rename = "EnvironmentTemplateArn")]
    pub environment_template_arn: Option<String>,
}

}
pub use _types::*;
