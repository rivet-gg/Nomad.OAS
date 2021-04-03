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
pub struct StatObject {
    #[serde(rename = "Nested", skip_serializing_if = "Option::is_none")]
    pub nested: Option<::std::collections::HashMap<String, crate::models::StatObject>>,
    #[serde(rename = "Attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, crate::models::StatValue>>,
}

impl StatObject {
    pub fn new() -> StatObject {
        StatObject {
            nested: None,
            attributes: None,
        }
    }
}

