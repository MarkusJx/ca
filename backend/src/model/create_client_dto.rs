use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateClientDto {
    pub name: String,
    #[serde(rename = "validUntil")]
    #[schema(example = "2025-01-01T00:00:00Z")]
    pub valid_until: String,
}
