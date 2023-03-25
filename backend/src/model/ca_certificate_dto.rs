use crate::entity::{certificate, root_certificate};
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
    /// Whether the certificate is the root certificate
    pub root: bool,
    /// The private key of the certificate
    /// This is only returned if the certificate is the root certificate
    /// and has just been created.
    #[serde(rename = "privateKey", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

impl FromModel<certificate::Model> for CACertificateDto {
    fn from_model(model: certificate::Model) -> Self {
        Self {
            certificate: model.public.to_string(),
            valid_until: model.valid_until.to_rfc3339(),
            created_at: model.created_at.to_rfc3339(),
            root: false,
            private_key: None,
        }
    }
}

impl CACertificateDto {
    pub fn from_root_model(model: root_certificate::Model, private_key: Option<Vec<u8>>) -> Self {
        Self {
            certificate: model.public.to_string(),
            valid_until: model.valid_until.to_rfc3339(),
            created_at: model.created_at.to_rfc3339(),
            root: true,
            private_key: private_key.map(|key| key.to_string()),
        }
    }
}
