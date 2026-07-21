use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FederatedCatalogParticipantCreateForm {
  pub id: String,
  pub name: String,
  pub target_url: String,
}
