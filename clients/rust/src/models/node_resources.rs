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
pub struct NodeResources {
    #[serde(rename = "CPU", skip_serializing_if = "Option::is_none")]
    pub CPU: Option<Box<crate::models::NodeCpuResources>>,
    #[serde(rename = "Memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<Box<crate::models::NodeMemoryResources>>,
    #[serde(rename = "Disk", skip_serializing_if = "Option::is_none")]
    pub disk: Option<Box<crate::models::NodeDiskResources>>,
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<crate::models::NetworkResource>>,
    #[serde(rename = "Devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<crate::models::NodeDeviceResource>>,
}

impl NodeResources {
    pub fn new() -> NodeResources {
        NodeResources {
            CPU: None,
            memory: None,
            disk: None,
            networks: None,
            devices: None,
        }
    }
}


