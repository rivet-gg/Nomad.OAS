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
pub struct SchedulerConfigurationResponse {
    #[serde(rename = "SchedulerConfig", skip_serializing_if = "Option::is_none")]
    pub scheduler_config: Option<Box<crate::models::SchedulerConfiguration>>,
}

impl SchedulerConfigurationResponse {
    pub fn new() -> SchedulerConfigurationResponse {
        SchedulerConfigurationResponse {
            scheduler_config: None,
        }
    }
}

