use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A certificate signing request
#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct NewSigningRequestDto {
    /// The client certificate
    pub request: String,
    /// Alternative names for the certificate
    #[serde(rename = "alternativeNames", skip_serializing_if = "Option::is_none")]
    pub alternative_names: Option<Vec<String>>,
}
