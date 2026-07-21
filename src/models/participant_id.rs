use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParticipantId {
  #[serde(rename = "@id")]
  pub id: String,
}
