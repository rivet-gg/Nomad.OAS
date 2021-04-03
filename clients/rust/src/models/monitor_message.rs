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
pub struct MonitorMessage {
    #[serde(rename = "Level", skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(rename = "Message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl MonitorMessage {
    pub fn new() -> MonitorMessage {
        MonitorMessage {
            level: None,
            message: None,
        }
    }
}

