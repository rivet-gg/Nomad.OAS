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
pub struct JobRegisterResponse {
    #[serde(rename = "EvalID", skip_serializing_if = "Option::is_none")]
    pub eval_id: Option<String>,
    #[serde(rename = "EvalCreateIndex", skip_serializing_if = "Option::is_none")]
    pub eval_create_index: Option<i32>,
    #[serde(rename = "JobModifyIndex", skip_serializing_if = "Option::is_none")]
    pub job_modify_index: Option<i32>,
    #[serde(rename = "Warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<String>,
}

impl JobRegisterResponse {
    pub fn new() -> JobRegisterResponse {
        JobRegisterResponse {
            eval_id: None,
            eval_create_index: None,
            job_modify_index: None,
            warnings: None,
        }
    }
}


