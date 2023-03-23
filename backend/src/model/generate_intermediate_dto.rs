use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GenerateIntermediateDto {
    #[serde(rename = "rootCertificate")]
    pub root_certificate: String,
}
