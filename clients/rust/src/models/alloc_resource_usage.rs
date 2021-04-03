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
pub struct AllocResourceUsage {
    #[serde(rename = "ResourceUsage", skip_serializing_if = "Option::is_none")]
    pub resource_usage: Option<Box<crate::models::ResourceUsage>>,
    #[serde(rename = "Tasks", skip_serializing_if = "Option::is_none")]
    pub tasks: Option<::std::collections::HashMap<String, crate::models::TaskResourceUsage>>,
    #[serde(rename = "Timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

impl AllocResourceUsage {
    pub fn new() -> AllocResourceUsage {
        AllocResourceUsage {
            resource_usage: None,
            tasks: None,
            timestamp: None,
        }
    }
}


