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
pub struct DrainStrategy {
    #[serde(rename = "DrainSpec", skip_serializing_if = "Option::is_none")]
    pub drain_spec: Option<Box<crate::models::DrainSpec>>,
    #[serde(rename = "ForceDeadline", skip_serializing_if = "Option::is_none")]
    pub force_deadline: Option<String>,
    #[serde(rename = "StartedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
}

impl DrainStrategy {
    pub fn new() -> DrainStrategy {
        DrainStrategy {
            drain_spec: None,
            force_deadline: None,
            started_at: None,
        }
    }
}

