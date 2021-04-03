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
pub struct Node {
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "HttpAddr", skip_serializing_if = "Option::is_none")]
    pub http_addr: Option<String>,
    #[serde(rename = "TlsEnabled", skip_serializing_if = "Option::is_none")]
    pub tls_enabled: Option<bool>,
    #[serde(rename = "Attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Box<crate::models::Resources>>,
    #[serde(rename = "Reserved", skip_serializing_if = "Option::is_none")]
    pub reserved: Option<Box<crate::models::Resources>>,
    #[serde(rename = "NodeResources", skip_serializing_if = "Option::is_none")]
    pub node_resources: Option<Box<crate::models::NodeResources>>,
    #[serde(rename = "ReservedResources", skip_serializing_if = "Option::is_none")]
    pub reserved_resources: Option<Box<crate::models::NodeReservedResources>>,
    #[serde(rename = "Links", skip_serializing_if = "Option::is_none")]
    pub links: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "NodeClass", skip_serializing_if = "Option::is_none")]
    pub node_class: Option<String>,
    #[serde(rename = "Drain", skip_serializing_if = "Option::is_none")]
    pub drain: Option<bool>,
    #[serde(rename = "DrainStrategy", skip_serializing_if = "Option::is_none")]
    pub drain_strategy: Option<Box<crate::models::DrainStrategy>>,
    #[serde(rename = "SchedulingEligibility", skip_serializing_if = "Option::is_none")]
    pub scheduling_eligibility: Option<String>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDescription", skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    #[serde(rename = "StatusUpdatedAt", skip_serializing_if = "Option::is_none")]
    pub status_updated_at: Option<i64>,
    #[serde(rename = "Events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<crate::models::NodeEvent>>,
    #[serde(rename = "Drivers", skip_serializing_if = "Option::is_none")]
    pub drivers: Option<::std::collections::HashMap<String, crate::models::DriverInfo>>,
    #[serde(rename = "HostVolumes", skip_serializing_if = "Option::is_none")]
    pub host_volumes: Option<::std::collections::HashMap<String, crate::models::HostVolumeInfo>>,
    #[serde(rename = "CsiControllerPlugins", skip_serializing_if = "Option::is_none")]
    pub csi_controller_plugins: Option<::std::collections::HashMap<String, crate::models::CsiInfo>>,
    #[serde(rename = "CsiNodePlugins", skip_serializing_if = "Option::is_none")]
    pub csi_node_plugins: Option<::std::collections::HashMap<String, crate::models::CsiInfo>>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            ID: None,
            datacenter: None,
            name: None,
            http_addr: None,
            tls_enabled: None,
            attributes: None,
            resources: None,
            reserved: None,
            node_resources: None,
            reserved_resources: None,
            links: None,
            meta: None,
            node_class: None,
            drain: None,
            drain_strategy: None,
            scheduling_eligibility: None,
            status: None,
            status_description: None,
            status_updated_at: None,
            events: None,
            drivers: None,
            host_volumes: None,
            csi_controller_plugins: None,
            csi_node_plugins: None,
            create_index: None,
            modify_index: None,
        }
    }
}


