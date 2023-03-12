use crate::entity::client;
use crate::repository::client_repository::ClientRepository;
use crate::util::types::DbResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct ClientService(DatabaseConnection);

impl ClientService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self(db)
    }

    pub async fn find_by_name(&self, name: &str) -> DbResult<Option<client::Model>> {
        ClientRepository::find_by_name(&self.0, name).await
    }

    pub async fn find_all_by_user(&self, user_id: &Uuid) -> DbResult<Vec<client::Model>> {
        ClientRepository::find_all_by_user(&self.0, user_id).await
    }
}
