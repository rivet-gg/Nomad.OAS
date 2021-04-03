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
pub struct Vault {
    #[serde(rename = "Policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
    #[serde(rename = "Env", skip_serializing_if = "Option::is_none")]
    pub env: Option<bool>,
    #[serde(rename = "ChangeMode", skip_serializing_if = "Option::is_none")]
    pub change_mode: Option<String>,
    #[serde(rename = "ChangeSignal", skip_serializing_if = "Option::is_none")]
    pub change_signal: Option<String>,
}

impl Vault {
    pub fn new() -> Vault {
        Vault {
            policies: None,
            env: None,
            change_mode: None,
            change_signal: None,
        }
    }
}

