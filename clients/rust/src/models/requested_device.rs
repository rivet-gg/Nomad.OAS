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
pub struct RequestedDevice {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "Constraints", skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<crate::models::Constraint>>,
    #[serde(rename = "Affinities", skip_serializing_if = "Option::is_none")]
    pub affinities: Option<Vec<crate::models::Affinity>>,
}

impl RequestedDevice {
    pub fn new() -> RequestedDevice {
        RequestedDevice {
            name: None,
            count: None,
            constraints: None,
            affinities: None,
        }
    }
}


