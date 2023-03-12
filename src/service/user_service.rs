use crate::entity::user;
use crate::error::http_response_error::{HttpResponseError, MapHttpResponseError};
use crate::repository::user_repository::UserRepository;
use crate::util::types::WebResult;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult};
use uuid::Uuid;

pub struct UserService(DatabaseConnection);

impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }

    pub async fn insert(&self, user: user::ActiveModel) -> WebResult<user::Model> {
        user.insert(&self.0)
            .await
            .map_internal_error(Some("Failed to insert user"))
    }

    pub async fn find_by_name(&self, name: &str) -> WebResult<Option<user::Model>> {
        UserRepository::find_by_name(&self.0, name)
            .await
            .map_internal_error(Some("Failed to find user by name"))
    }

    pub async fn find_by_id(&self, id: &Uuid) -> WebResult<Option<user::Model>> {
        UserRepository::find_by_id(&self.0, id)
            .await
            .map_internal_error(Some("Failed to find user by id"))
    }

    pub async fn find_by_id_string_unwrap(&self, id: &String) -> WebResult<user::Model> {
        self.find_by_id(&Uuid::parse_str(id.as_str()).map_bad_request(Some("Invalid id supplied"))?)
            .await
            .map_internal_error(Some("Failed to find user by id"))?
            .ok_or_else(|| HttpResponseError::not_found(Some("User not found")))
    }

    pub async fn find_all(&self) -> WebResult<Vec<user::Model>> {
        UserRepository::find_all(&self.0)
            .await
            .map_internal_error(Some("Failed to find all users"))
    }

    pub async fn delete_by_id(&self, id: &Uuid) -> WebResult<DeleteResult> {
        UserRepository::delete_by_id(&self.0, id)
            .await
            .map_internal_error(Some("Failed to delete user by id"))
    }

    pub async fn disable(&self, model: user::ActiveModel) -> WebResult<user::ActiveModel> {
        UserRepository::disable(&self.0, model)
            .await
            .map_internal_error(Some("Failed to disable user"))
    }
}
