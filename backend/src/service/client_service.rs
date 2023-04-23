use crate::entity::client;
use crate::error::http_response_error::{HttpResponseError, MapHttpResponseError};
use crate::repository::client_repository::ClientRepository;
use crate::util::types::WebResult;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult};
use std::sync::Arc;
use uuid::Uuid;

pub struct ClientService(Arc<DatabaseConnection>);

impl ClientService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self(db)
    }

    pub async fn insert(&self, model: client::ActiveModel) -> WebResult<client::Model> {
        ClientRepository::insert(self.0.as_ref(), model)
            .await
            .map_internal_error("Failed to create client")
    }

    pub async fn update(&self, model: client::ActiveModel) -> WebResult<client::Model> {
        model
            .update(self.0.as_ref())
            .await
            .map_internal_error("Failed to update client")
    }

    pub async fn find_by_id(
        &self,
        id: &Uuid,
        include_inactive: bool,
    ) -> WebResult<Option<client::Model>> {
        ClientRepository::find_by_id(self.0.as_ref(), id, include_inactive)
            .await
            .map_internal_error("Failed to find client by id")
    }

    pub async fn find_user_client(&self, user_id: &Uuid) -> WebResult<Option<client::Model>> {
        ClientRepository::find_user_client(self.0.as_ref(), user_id)
            .await
            .map_internal_error("Failed to find user client")
    }

    pub async fn generate_id(&self) -> WebResult<Uuid> {
        let mut id = Uuid::new_v4();
        while let Some(_) = self.find_by_id(&id, true).await? {
            id = Uuid::new_v4();
        }

        Ok(id)
    }

    pub async fn find_by_name(&self, name: &str) -> WebResult<Option<client::Model>> {
        ClientRepository::find_by_name(self.0.as_ref(), name)
            .await
            .map_internal_error("Failed to find client by name")
    }

    pub async fn find_by_id_string_unwrap(
        &self,
        id: &String,
        include_inactive: bool,
    ) -> WebResult<client::Model> {
        ClientRepository::find_by_id(
            self.0.as_ref(),
            &Uuid::parse_str(id.as_str()).map_bad_request("Invalid id supplied")?,
            include_inactive,
        )
        .await
        .map_internal_error("Failed to find client by id")?
        .ok_or_else(|| HttpResponseError::not_found("Client not found"))
    }

    pub async fn find_all_by_user(
        &self,
        user_id: &Uuid,
        include_inactive: bool,
    ) -> WebResult<Vec<client::Model>> {
        ClientRepository::find_all_by_user(self.0.as_ref(), user_id, include_inactive)
            .await
            .map_internal_error("Failed to find all clients by user")
    }

    pub async fn disable(&self, model: client::ActiveModel) -> WebResult<client::ActiveModel> {
        ClientRepository::disable(self.0.as_ref(), model)
            .await
            .map_internal_error("Failed to disable client")
    }

    pub async fn delete(&self, model: client::ActiveModel) -> WebResult<DeleteResult> {
        ClientRepository::delete(self.0.as_ref(), model)
            .await
            .map_internal_error("Failed to delete client")
    }
}
