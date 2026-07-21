use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FederatedCatalogParticipant {
  pub id: String,
  pub name: String,
  pub target_url: String,
}
