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
pub struct AllocDeploymentStatus {
    #[serde(rename = "Healthy", skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
    #[serde(rename = "Timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "Canary", skip_serializing_if = "Option::is_none")]
    pub canary: Option<bool>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
}

impl AllocDeploymentStatus {
    pub fn new() -> AllocDeploymentStatus {
        AllocDeploymentStatus {
            healthy: None,
            timestamp: None,
            canary: None,
            modify_index: None,
        }
    }
}


