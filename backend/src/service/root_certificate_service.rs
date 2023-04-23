use crate::entity::root_certificate;
use crate::error::http_response_error::MapHttpResponseError;
use crate::repository::root_certificate_repository::RootCertificateRepository;
use crate::util::types::WebResult;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct RootCertificateService(Arc<DatabaseConnection>);

impl RootCertificateService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self(db)
    }

    pub async fn insert(
        &self,
        model: root_certificate::ActiveModel,
    ) -> WebResult<root_certificate::Model> {
        RootCertificateRepository::insert(self.0.as_ref(), model)
            .await
            .map_internal_error("Failed to insert root certificate")
    }

    pub async fn find_active(&self) -> WebResult<Option<root_certificate::Model>> {
        RootCertificateRepository::find_active(self.0.as_ref())
            .await
            .map_internal_error("Failed to find active root certificate")
    }
}
