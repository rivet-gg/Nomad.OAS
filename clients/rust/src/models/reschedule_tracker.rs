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
pub struct RescheduleTracker {
    #[serde(rename = "Events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<crate::models::RescheduleEvent>>,
}

impl RescheduleTracker {
    pub fn new() -> RescheduleTracker {
        RescheduleTracker {
            events: None,
        }
    }
}

