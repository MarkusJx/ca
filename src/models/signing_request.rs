use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A certificate signing request
#[derive(Deserialize, Serialize, ToSchema)]
pub struct SigningRequest {
    /// The client certificate
    pub cert: String,
    /// The client name
    #[serde(rename = "clientName")]
    pub client_name: String,
}
