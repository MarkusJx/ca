use crate::entity::signing_request;
use crate::util::traits::from_model::FromModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SigningRequestDto {
    #[serde(rename = "clientId")]
    client_id: String,
    hash: String,
    #[serde(rename = "issuedAt")]
    issued_at: String,
}

impl FromModel<signing_request::Model> for SigningRequestDto {
    fn from_model(model: signing_request::Model) -> Self {
        Self {
            client_id: model.client_id.to_string(),
            hash: model.hash,
            issued_at: model.issued_at.to_rfc3339(),
        }
    }
}
