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
pub struct AutopilotConfiguration {
    #[serde(rename = "CleanupDeadServers", skip_serializing_if = "Option::is_none")]
    pub cleanup_dead_servers: Option<bool>,
    #[serde(rename = "LastContactThreshold", skip_serializing_if = "Option::is_none")]
    pub last_contact_threshold: Option<i64>,
    #[serde(rename = "MaxTrailingLogs", skip_serializing_if = "Option::is_none")]
    pub max_trailing_logs: Option<i32>,
    #[serde(rename = "MinQuorum", skip_serializing_if = "Option::is_none")]
    pub min_quorum: Option<i64>,
    #[serde(rename = "ServerStabilizationTime", skip_serializing_if = "Option::is_none")]
    pub server_stabilization_time: Option<i64>,
    #[serde(rename = "EnableRedundancyZones", skip_serializing_if = "Option::is_none")]
    pub enable_redundancy_zones: Option<bool>,
    #[serde(rename = "DisableUpgradeMigration", skip_serializing_if = "Option::is_none")]
    pub disable_upgrade_migration: Option<bool>,
    #[serde(rename = "EnableCustomUpgrades", skip_serializing_if = "Option::is_none")]
    pub enable_custom_upgrades: Option<bool>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
}

impl AutopilotConfiguration {
    pub fn new() -> AutopilotConfiguration {
        AutopilotConfiguration {
            cleanup_dead_servers: None,
            last_contact_threshold: None,
            max_trailing_logs: None,
            min_quorum: None,
            server_stabilization_time: None,
            enable_redundancy_zones: None,
            disable_upgrade_migration: None,
            enable_custom_upgrades: None,
            create_index: None,
            modify_index: None,
        }
    }
}

