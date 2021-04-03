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
pub struct AllocatedTaskResources {
    #[serde(rename = "Cpu", skip_serializing_if = "Option::is_none")]
    pub cpu: Option<Box<crate::models::AllocatedCpuResources>>,
    #[serde(rename = "Memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<Box<crate::models::AllocatedMemoryResources>>,
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<crate::models::NetworkResource>>,
}

impl AllocatedTaskResources {
    pub fn new() -> AllocatedTaskResources {
        AllocatedTaskResources {
            cpu: None,
            memory: None,
            networks: None,
        }
    }
}

