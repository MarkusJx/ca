use crate::entity::signing_request;
use crate::error::http_response_error::MapHttpResponseError;
use crate::util::types::WebResult;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

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
}
