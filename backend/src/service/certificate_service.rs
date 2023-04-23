use crate::entity::certificate;
use crate::error::http_response_error::MapHttpResponseError;
use crate::repository::certificate_repository::CertificateRepository;
use crate::util::types::WebResult;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct CertificateService(Arc<DatabaseConnection>);

impl CertificateService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self(db)
    }

    pub async fn insert(&self, model: certificate::ActiveModel) -> WebResult<certificate::Model> {
        CertificateRepository::insert(self.0.as_ref(), model)
            .await
            .map_internal_error("Failed to create certificate")
    }

    pub async fn find_active(&self) -> WebResult<Option<certificate::Model>> {
        CertificateRepository::find_active(self.0.as_ref())
            .await
            .map_internal_error("Failed to find active certificate")
    }

    /*pub async fn get_certificate(&self, config: &Config) -> WebResult<certificate::Model> {
        if let Some(cert) = self.find_active().await? {
            return Ok(cert);
        } else {
            info!("No active ca certificate found, generating new one");
            let cert = CACertificate::generate(config)
                .map_internal_error(Some("Failed to generate CA cert"))?;

            self.insert(certificate::ActiveModel {
                public: ActiveValue::set(
                    cert.cert_as_pem()
                        .map_internal_error(Some("Failed to convert public key to pem"))?,
                ),
                private: ActiveValue::set(Some(
                    cert.key_pair_as_pem()
                        .map_internal_error(Some("Failed to convert private key to pem"))?,
                )),
                valid_until: ActiveValue::set(
                    cert.valid_until().map_internal_error(Some(
                        "Failed to get valid until date from certificate",
                    ))?,
                ),
                ..Default::default()
            })
            .await
        }
    }*/
}
