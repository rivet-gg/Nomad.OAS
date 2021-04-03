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
pub struct SearchRequest {
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Context")]
    pub context: String,
}

impl SearchRequest {
    pub fn new(prefix: String, context: String) -> SearchRequest {
        SearchRequest {
            prefix,
            context,
        }
    }
}

