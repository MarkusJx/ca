use crate::entity::client;
use crate::error::http_response_error::MapHttpResponseError;
use crate::repository::client_repository::ClientRepository;
use crate::util::types::WebResult;
use sea_orm::DatabaseConnection;
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

    pub async fn find_by_name(&self, name: &str) -> WebResult<Option<client::Model>> {
        ClientRepository::find_by_name(&self.0, name)
            .await
            .map_internal_error(Some("Failed to find client by name"))
    }

    pub async fn find_all_by_user(&self, user_id: &Uuid) -> WebResult<Vec<client::Model>> {
        ClientRepository::find_all_by_user(&self.0, user_id)
            .await
            .map_internal_error(Some("Failed to find all clients by user"))
    }
}
