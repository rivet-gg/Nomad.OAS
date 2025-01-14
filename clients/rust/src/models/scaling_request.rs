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
pub struct ScalingRequest {
    #[serde(rename = "Count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "Target")]
    pub target: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "Meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "PolicyOverride", skip_serializing_if = "Option::is_none")]
    pub policy_override: Option<bool>,
}

impl ScalingRequest {
    pub fn new(target: Option<::std::collections::HashMap<String, String>>) -> ScalingRequest {
        ScalingRequest {
            count: None,
            target: None,
            reason: None,
            error: None,
            meta: None,
            policy_override: None,
        }
    }
}


