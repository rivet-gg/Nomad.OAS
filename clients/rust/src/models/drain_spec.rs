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
pub struct DrainSpec {
    #[serde(rename = "Deadline")]
    pub deadline: i64,
    #[serde(rename = "IgnoreSystemJobs", skip_serializing_if = "Option::is_none")]
    pub ignore_system_jobs: Option<bool>,
}

impl DrainSpec {
    pub fn new(deadline: i64) -> DrainSpec {
        DrainSpec {
            deadline,
            ignore_system_jobs: None,
        }
    }
}

