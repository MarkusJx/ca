use crate::entity::token;
use crate::error::http_response_error::MapHttpResponseError;
use crate::repository::token_repository::TokenRepository;
use crate::util::types::WebResult;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use uuid::Uuid;

pub struct TokenService(DatabaseConnection);

impl TokenService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }

    pub async fn generate_id(&self) -> WebResult<Uuid> {
        let mut id = Uuid::new_v4();
        while let Some(_) = self.find_by_id(&id, true).await? {
            id = Uuid::new_v4();
        }

        Ok(id)
    }

    pub async fn find_by_id(
        &self,
        id: &Uuid,
        include_inactive: bool,
    ) -> WebResult<Option<token::Model>> {
        TokenRepository::find_by_id(&self.0, id, include_inactive)
            .await
            .map_internal_error(Some("Failed to find token by id"))
    }

    pub async fn find_by_client_id(
        &self,
        client_id: &Uuid,
        include_inactive: bool,
    ) -> WebResult<Option<token::Model>> {
        Ok(
            TokenRepository::find_all_by_client(&self.0, client_id, include_inactive)
                .await
                .map_internal_error(Some("Failed to find token by client id"))?
                .into_iter()
                .next(),
        )
    }

    pub async fn insert(&self, model: token::ActiveModel) -> WebResult<token::Model> {
        model
            .insert(&self.0)
            .await
            .map_internal_error(Some("Failed to create token"))
    }

    pub async fn deactivate_all_by_client_id(
        &self,
        client_id: &Uuid,
    ) -> WebResult<Vec<token::Model>> {
        TokenRepository::deactivate_all_by_client(&self.0, client_id)
            .await
            .map_internal_error(Some("Failed to deactivate tokens by client id"))
    }
}
