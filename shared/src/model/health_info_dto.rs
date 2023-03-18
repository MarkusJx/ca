use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthInfoDto {
    /// The current version of the API
    #[schema(example = "1.0.0")]
    pub version: String,
    /// The current version of keycloak
    #[schema(example = "1.0.0")]
    #[serde(rename = "keycloakVersion", skip_serializing_if = "Option::is_none")]
    pub keycloak_version: Option<String>,
    /// The current status of the API
    #[schema(example = "OK")]
    pub status: String,
    /// Whether the API is up and running
    #[schema(example = "true")]
    pub ok: bool,
}
