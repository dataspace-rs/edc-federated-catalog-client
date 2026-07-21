use super::{Dataset, ParticipantId, Service};
use serde::{Deserialize, Serialize};
use serde_with::{OneOrMany, formats::PreferMany, serde_as};

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FederatedCatalogOffer {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "http://www.w3.org/ns/dcat#dataset")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub dataset: Vec<Dataset>,
    #[serde(rename = "http://www.w3.org/ns/dcat#service")]
    pub service: Service,
    #[serde(
        rename = "participantId",
        alias = "dspace:participantId",
        alias = "https://w3id.org/dspace/2025/1/participantId"
    )]
    pub participant_id: ParticipantId,
    #[serde(rename = "originator", alias = "edc:originator")]
    pub originator: String,
}
