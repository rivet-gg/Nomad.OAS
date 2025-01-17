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
pub struct TaskArtifact {
    #[serde(rename = "GetterSource", skip_serializing_if = "Option::is_none")]
    pub getter_source: Option<String>,
    #[serde(rename = "GetterOptions", skip_serializing_if = "Option::is_none")]
    pub getter_options: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "GetterMode", skip_serializing_if = "Option::is_none")]
    pub getter_mode: Option<String>,
    #[serde(rename = "RelativeDest", skip_serializing_if = "Option::is_none")]
    pub relative_dest: Option<String>,
}

impl TaskArtifact {
    pub fn new() -> TaskArtifact {
        TaskArtifact {
            getter_source: None,
            getter_options: None,
            getter_mode: None,
            relative_dest: None,
        }
    }
}


