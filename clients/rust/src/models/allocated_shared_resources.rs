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
pub struct AllocatedSharedResources {
    #[serde(rename = "DiskMB", skip_serializing_if = "Option::is_none")]
    pub disk_mb: Option<i64>,
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<crate::models::NetworkResource>>,
}

impl AllocatedSharedResources {
    pub fn new() -> AllocatedSharedResources {
        AllocatedSharedResources {
            disk_mb: None,
            networks: None,
        }
    }
}


