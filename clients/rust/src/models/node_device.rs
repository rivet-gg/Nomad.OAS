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
pub struct NodeDevice {
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Healthy", skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
    #[serde(rename = "HealthDescription", skip_serializing_if = "Option::is_none")]
    pub health_description: Option<String>,
    #[serde(rename = "Locality", skip_serializing_if = "Option::is_none")]
    pub locality: Option<Box<crate::models::NodeDeviceLocality>>,
}

impl NodeDevice {
    pub fn new() -> NodeDevice {
        NodeDevice {
            ID: None,
            healthy: None,
            health_description: None,
            locality: None,
        }
    }
}


