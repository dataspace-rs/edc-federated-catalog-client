use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "http://www.w3.org/ns/dcat#endpointDescription")]
    pub endpoint_description: String,
    #[serde(rename = "http://www.w3.org/ns/dcat#endpointURL")]
    pub endpoint_url: String,
}
