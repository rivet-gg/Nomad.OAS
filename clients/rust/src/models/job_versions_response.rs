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
pub struct JobVersionsResponse {
    #[serde(rename = "Versions", skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<crate::models::Job>>,
    #[serde(rename = "Diffs", skip_serializing_if = "Option::is_none")]
    pub diffs: Option<Vec<crate::models::JobDiff>>,
}

impl JobVersionsResponse {
    pub fn new() -> JobVersionsResponse {
        JobVersionsResponse {
            versions: None,
            diffs: None,
        }
    }
}


