use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Policy {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub r#type: String,
    // #[serde(rename = "http://www.w3.org/ns/odrl/2/obligation")]
    // obligations: Vec<String>,
    // #[serde(rename = "http://www.w3.org/ns/odrl/2/permission")]
    // permission: Vec<String>,
    // #[serde(rename = "http://www.w3.org/ns/odrl/2/prohibition")]
    // prohibition: Vec<String>,
}
