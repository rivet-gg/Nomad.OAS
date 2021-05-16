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
pub struct NodeReservedResources {
    #[serde(rename = "CPU", skip_serializing_if = "Option::is_none")]
    pub CPU: Option<Box<crate::models::NodeReservedCpuResources>>,
    #[serde(rename = "Memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<Box<crate::models::NodeReservedMemoryResources>>,
    #[serde(rename = "Disk", skip_serializing_if = "Option::is_none")]
    pub disk: Option<Box<crate::models::NodeReservedDiskResources>>,
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Box<crate::models::NodeReservedNetworkResources>>,
}

impl NodeReservedResources {
    pub fn new() -> NodeReservedResources {
        NodeReservedResources {
            CPU: None,
            memory: None,
            disk: None,
            networks: None,
        }
    }
}


