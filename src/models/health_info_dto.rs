use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthInfoDto {
    /// The current version of the API
    #[schema(example = "1.0.0")]
    pub version: String,
    /// The current status of the API
    #[schema(example = "OK")]
    pub status: String,
}
