use crate::entity::signing_request;
use crate::error::http_response_error::MapHttpResponseError;
use crate::repository::signing_request_repository::SigningRequestRepository;
use crate::util::types::WebResult;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use std::sync::Arc;
use uuid::Uuid;

pub struct SigningRequestService(Arc<DatabaseConnection>);

impl SigningRequestService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self(db)
    }

    pub async fn save(
        &self,
        model: signing_request::ActiveModel,
    ) -> WebResult<signing_request::ActiveModel> {
        model
            .save(self.0.as_ref())
            .await
            .map_internal_error("Failed to save signing request")
    }

    pub async fn find_all_by_client_id(
        &self,
        client_id: &Uuid,
    ) -> WebResult<Vec<signing_request::Model>> {
        SigningRequestRepository::find_all_by_client(self.0.as_ref(), client_id)
            .await
            .map_internal_error("Failed to find signing requests")
    }

    pub async fn find_all_by_user_id(
        &self,
        user_id: &Uuid,
    ) -> WebResult<Vec<signing_request::Model>> {
        SigningRequestRepository::find_all_by_user(self.0.as_ref(), user_id)
            .await
            .map_internal_error("Failed to find signing requests")
    }
}
