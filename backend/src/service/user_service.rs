use crate::entity::user;
use crate::error::http_response_error::{HttpResponseError, MapHttpResponseError};
use crate::repository::user_repository::UserRepository;
use crate::util::types::WebResult;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService(Arc<DatabaseConnection>);

impl UserService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self(db)
    }

    pub async fn insert(&self, user: user::ActiveModel) -> WebResult<user::Model> {
        user.insert(self.0.as_ref())
            .await
            .map_internal_error("Failed to insert user")
    }

    pub async fn find_by_name(
        &self,
        name: &str,
        include_inactive: bool,
    ) -> WebResult<Option<user::Model>> {
        UserRepository::find_by_name(self.0.as_ref(), name, include_inactive)
            .await
            .map_internal_error("Failed to find user by name")
    }

    pub async fn find_by_id(
        &self,
        id: &Uuid,
        include_inactive: bool,
    ) -> WebResult<Option<user::Model>> {
        UserRepository::find_by_id(self.0.as_ref(), id, include_inactive)
            .await
            .map_internal_error("Failed to find user by id")
    }

    pub async fn find_by_id_string_unwrap(
        &self,
        id: &String,
        include_inactive: bool,
    ) -> WebResult<user::Model> {
        self.find_by_id(
            &Uuid::parse_str(id.as_str()).map_bad_request("Invalid id supplied")?,
            include_inactive,
        )
        .await
        .map_internal_error("Failed to find user by id")?
        .ok_or_else(|| HttpResponseError::not_found("User not found"))
    }

    pub async fn find_by_external_id(
        &self,
        external_id: &str,
        include_inactive: bool,
    ) -> WebResult<Option<user::Model>> {
        UserRepository::find_by_external_id(self.0.as_ref(), external_id, include_inactive)
            .await
            .map_internal_error("Failed to find user by external id")
    }

    pub async fn find_all(&self, include_inactive: bool) -> WebResult<Vec<user::Model>> {
        UserRepository::find_all(self.0.as_ref(), include_inactive)
            .await
            .map_internal_error("Failed to find all users")
    }

    pub async fn delete(&self, model: user::Model) -> WebResult<DeleteResult> {
        UserRepository::delete(self.0.as_ref(), model)
            .await
            .map_internal_error("Failed to delete user by id")
    }

    pub async fn disable(&self, model: user::ActiveModel) -> WebResult<user::ActiveModel> {
        UserRepository::disable(self.0.as_ref(), model)
            .await
            .map_internal_error("Failed to disable user")
    }
}
