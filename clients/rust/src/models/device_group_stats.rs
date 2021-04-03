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
pub struct DeviceGroupStats {
    #[serde(rename = "Vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "InstanceStats", skip_serializing_if = "Option::is_none")]
    pub instance_stats: Option<::std::collections::HashMap<String, crate::models::DeviceStats>>,
}

impl DeviceGroupStats {
    pub fn new() -> DeviceGroupStats {
        DeviceGroupStats {
            vendor: None,
            _type: None,
            name: None,
            instance_stats: None,
        }
    }
}

