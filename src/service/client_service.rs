use crate::entity::client;
use crate::error::http_response_error::{HttpResponseError, MapHttpResponseError};
use crate::repository::client_repository::ClientRepository;
use crate::util::types::WebResult;
use sea_orm::{DatabaseConnection, DeleteResult};
use uuid::Uuid;

pub struct ClientService(DatabaseConnection);

impl ClientService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }

    pub async fn insert(&self, model: client::ActiveModel) -> WebResult<client::Model> {
        ClientRepository::insert(&self.0, model)
            .await
            .map_internal_error(Some("Failed to create client"))
    }

    pub async fn find_by_id(
        &self,
        id: &Uuid,
        include_inactive: bool,
    ) -> WebResult<Option<client::Model>> {
        ClientRepository::find_by_id(&self.0, id, include_inactive)
            .await
            .map_internal_error(Some("Failed to find client by id"))
    }

    pub async fn generate_id(&self) -> WebResult<Uuid> {
        let mut id = Uuid::new_v4();
        while let Some(_) = self.find_by_id(&id, true).await? {
            id = Uuid::new_v4();
        }

        Ok(id)
    }

    pub async fn find_by_name(&self, name: &str) -> WebResult<Option<client::Model>> {
        ClientRepository::find_by_name(&self.0, name)
            .await
            .map_internal_error(Some("Failed to find client by name"))
    }

    pub async fn find_by_id_string_unwrap(
        &self,
        id: &String,
        include_inactive: bool,
    ) -> WebResult<client::Model> {
        ClientRepository::find_by_id(
            &self.0,
            &Uuid::parse_str(id.as_str()).map_bad_request(Some("Invalid id supplied"))?,
            include_inactive,
        )
        .await
        .map_internal_error(Some("Failed to find client by id"))?
        .ok_or_else(|| HttpResponseError::not_found(Some("Client not found")))
    }

    pub async fn find_all_by_user(
        &self,
        user_id: &Uuid,
        include_inactive: bool,
    ) -> WebResult<Vec<client::Model>> {
        ClientRepository::find_all_by_user(&self.0, user_id, include_inactive)
            .await
            .map_internal_error(Some("Failed to find all clients by user"))
    }

    pub async fn disable(&self, model: client::ActiveModel) -> WebResult<client::ActiveModel> {
        ClientRepository::disable(&self.0, model)
            .await
            .map_internal_error(Some("Failed to disable client"))
    }

    pub async fn delete(&self, model: client::ActiveModel) -> WebResult<DeleteResult> {
        ClientRepository::delete(&self.0, model)
            .await
            .map_internal_error(Some("Failed to delete client"))
    }
}
