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
pub struct NodeEvalResponse {
    #[serde(rename = "EvalID", skip_serializing_if = "Option::is_none")]
    pub eval_id: Option<String>,
}

impl NodeEvalResponse {
    pub fn new() -> NodeEvalResponse {
        NodeEvalResponse {
            eval_id: None,
        }
    }
}


