use crate::entity::certificate;
use crate::util::traits::from_model::FromModel;
use serde::{Deserialize, Serialize};
use shared::util::traits::u8_vec_to_string::U8VecToString;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CACertificateDto {
    /// The certificate pem string
    pub certificate: String,
    /// The time the certificate is valid until
    #[serde(rename = "validUntil")]
    pub valid_until: String,
    /// The time the certificate was created
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

impl FromModel<certificate::Model> for CACertificateDto {
    fn from_model(model: certificate::Model) -> Self {
        Self {
            certificate: model.public.to_string(),
            valid_until: model.valid_until.to_rfc3339(),
            created_at: model.created_at.to_rfc3339(),
        }
    }
}
