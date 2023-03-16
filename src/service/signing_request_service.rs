use crate::entity::signing_request;
use crate::error::http_response_error::MapHttpResponseError;
use crate::repository::signing_request_repository::SigningRequestRepository;
use crate::util::types::WebResult;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use uuid::Uuid;

pub struct SigningRequestService(DatabaseConnection);

impl SigningRequestService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }

    pub async fn save(
        &self,
        model: signing_request::ActiveModel,
    ) -> WebResult<signing_request::ActiveModel> {
        model
            .save(&self.0)
            .await
            .map_internal_error(Some("Failed to save signing request"))
    }

    pub async fn find_all_by_client_id(
        &self,
        client_id: &Uuid,
    ) -> WebResult<Vec<signing_request::Model>> {
        SigningRequestRepository::find_all_by_client(&self.0, client_id)
            .await
            .map_internal_error(Some("Failed to find signing requests"))
    }

    pub async fn find_all_by_user_id(
        &self,
        user_id: &Uuid,
    ) -> WebResult<Vec<signing_request::Model>> {
        SigningRequestRepository::find_all_by_user(&self.0, user_id)
            .await
            .map_internal_error(Some("Failed to find signing requests"))
    }
}
