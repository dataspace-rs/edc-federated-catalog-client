pub mod models;

use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub enum FederatedCatalogClientVersion {
    V4,
}

impl Display for FederatedCatalogClientVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FederatedCatalogClientVersion::V4 => write!(f, "v4"),
        }
    }
}

pub struct FederatedCatalogClient {
    client: reqwest::Client,
    endpoint: String,
    bearer_token: Option<String>,
    version: FederatedCatalogClientVersion,
}

impl FederatedCatalogClient {
    pub fn new(
        client: reqwest::Client,
        endpoint: String,
        bearer_token: Option<String>,
        version: FederatedCatalogClientVersion,
    ) -> Self {
        Self {
            client,
            endpoint,
            bearer_token,
            version,
        }
    }

    pub async fn list_offers(&self) -> reqwest::Result<Vec<models::FederatedCatalogOffer>> {
        let list_offer_body = ListOfferBody::default();

        let request_builder = self.client.post(format!(
            "{}/api/management/{}/catalogs/request",
            self.endpoint, self.version,
        ));

        let request_builder = if let Some(bearer_token) = &self.bearer_token {
            request_builder.header("Authorization", format!("Bearer {bearer_token}"))
        } else {
            request_builder
        };

        let response = request_builder
          .json(&list_offer_body)
          .send()
          .await?;

        let offers = response
            .json::<Vec<models::FederatedCatalogOffer>>()
            .await
            .unwrap_or_default();

        Ok(offers)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListOfferBody {
    #[serde(rename = "@context")]
    context: ListOfferContext,
    #[serde(rename = "@type")]
    r#type: String,
}

impl Default for ListOfferBody {
    fn default() -> Self {
        Self {
            context: ListOfferContext {
                vocab: "https://w3id.org/edc/v0.0.1/ns/".to_string(),
            },
            r#type: "QuerySpec".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListOfferContext {
    #[serde(rename = "@vocab")]
    vocab: String,
}
