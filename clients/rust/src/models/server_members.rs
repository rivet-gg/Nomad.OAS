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
pub struct ServerMembers {
    #[serde(rename = "ServerName", skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "ServerRegion", skip_serializing_if = "Option::is_none")]
    pub server_region: Option<String>,
    #[serde(rename = "ServerDc", skip_serializing_if = "Option::is_none")]
    pub server_dc: Option<String>,
    #[serde(rename = "Members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<crate::models::AgentMember>>,
}

impl ServerMembers {
    pub fn new() -> ServerMembers {
        ServerMembers {
            server_name: None,
            server_region: None,
            server_dc: None,
            members: None,
        }
    }
}

