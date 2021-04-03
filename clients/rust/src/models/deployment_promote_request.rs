/*
 * Nomad
 *
 * Nomad OpenApi specification
 *
 * The version of the OpenAPI document: 0.11.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentPromoteRequest {
    #[serde(rename = "DeploymentID", skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "All", skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[serde(rename = "Groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
}

impl DeploymentPromoteRequest {
    pub fn new() -> DeploymentPromoteRequest {
        DeploymentPromoteRequest {
            deployment_id: None,
            all: None,
            groups: None,
        }
    }
}

