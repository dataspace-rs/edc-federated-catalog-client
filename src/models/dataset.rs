use super::Policy;
use serde::{Deserialize, Serialize};
use serde_with::{OneOrMany, formats::PreferMany, serde_as};

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dataset {
  #[serde(rename = "@id")]
  pub id: String,
  #[serde(rename = "@type")]
  pub r#type: String,
  #[serde(rename = "http://www.w3.org/ns/odrl/2/hasPolicy")]
  #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
  pub has_policy: Vec<Policy>,
  #[serde(rename = "name", alias = "edc:name")]
  pub name: String,
  #[serde(rename = "contenttype", alias = "edc:contenttype")]
  pub content_type: String,
}
