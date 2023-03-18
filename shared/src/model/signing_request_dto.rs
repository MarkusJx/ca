use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SigningRequestDto {
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub hash: String,
    #[serde(rename = "issuedAt")]
    pub issued_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[serde(rename = "subjectName")]
    pub subject_name: String,
}
