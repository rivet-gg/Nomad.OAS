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
pub struct Spread {
    #[serde(rename = "Attribute", skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    #[serde(rename = "Weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(rename = "SpreadTarget", skip_serializing_if = "Option::is_none")]
    pub spread_target: Option<Vec<crate::models::SpreadTarget>>,
}

impl Spread {
    pub fn new() -> Spread {
        Spread {
            attribute: None,
            weight: None,
            spread_target: None,
        }
    }
}

