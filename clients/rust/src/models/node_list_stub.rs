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
pub struct NodeListStub {
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NodeClass", skip_serializing_if = "Option::is_none")]
    pub node_class: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "Drain", skip_serializing_if = "Option::is_none")]
    pub drain: Option<bool>,
    #[serde(rename = "SchedulingEligibility", skip_serializing_if = "Option::is_none")]
    pub scheduling_eligibility: Option<String>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDescription", skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    #[serde(rename = "Drivers", skip_serializing_if = "Option::is_none")]
    pub drivers: Option<::std::collections::HashMap<String, crate::models::DriverInfo>>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
}

impl NodeListStub {
    pub fn new() -> NodeListStub {
        NodeListStub {
            address: None,
            ID: None,
            datacenter: None,
            name: None,
            node_class: None,
            version: None,
            drain: None,
            scheduling_eligibility: None,
            status: None,
            status_description: None,
            drivers: None,
            create_index: None,
            modify_index: None,
        }
    }
}


