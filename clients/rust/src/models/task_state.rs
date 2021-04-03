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
pub struct TaskState {
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Failed", skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    #[serde(rename = "Restarts", skip_serializing_if = "Option::is_none")]
    pub restarts: Option<i32>,
    #[serde(rename = "LastRestart", skip_serializing_if = "Option::is_none")]
    pub last_restart: Option<String>,
    #[serde(rename = "StartedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(rename = "FinishedAt", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    #[serde(rename = "Events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<crate::models::TaskEvent>>,
}

impl TaskState {
    pub fn new() -> TaskState {
        TaskState {
            state: None,
            failed: None,
            restarts: None,
            last_restart: None,
            started_at: None,
            finished_at: None,
            events: None,
        }
    }
}

