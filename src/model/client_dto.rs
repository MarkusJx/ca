use crate::entity::client;
use crate::util::traits::from_model::FromModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ClientDto {
    /// The client id
    pub id: String,
    /// The client name
    pub name: String,
    /// The id of the user that owns the client
    #[serde(rename = "userId")]
    pub user_id: String,
    /// The client token. Only returned when creating a new client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// The client token hash
    #[serde(rename = "tokenHash")]
    pub token_hash: String,
    /// Whether the client is active
    pub active: bool,
    /// The time the client is valid until
    #[serde(rename = "validUntil")]
    pub valid_until: String,
    /// The time the client was created
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// The time the client was last updated
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl FromModel<client::Model> for ClientDto {
    fn from_model(model: client::Model) -> Self {
        Self {
            id: model.id.to_string(),
            name: model.name,
            user_id: model.user_id.to_string(),
            token: None,
            token_hash: model.token_hash,
            active: model.active,
            valid_until: model.valid_until.to_rfc3339(),
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}
