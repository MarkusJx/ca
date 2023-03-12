use crate::entity::signing_request;
use crate::util::types::DbResult;
use sea_orm::{ColumnTrait, ConnectionTrait, DeleteResult, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

pub struct SigningRequestRepository;

impl SigningRequestRepository {
    pub async fn find_all_by_client<C: ConnectionTrait>(
        db: &C,
        client_id: &Uuid,
    ) -> DbResult<Vec<signing_request::Model>> {
        signing_request::Entity::find()
            .filter(signing_request::Column::ClientId.eq(client_id.clone()))
            .all(db)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete<C: ConnectionTrait>(
        db: &C,
        model: signing_request::Model,
    ) -> DbResult<DeleteResult> {
        model.delete(db).await.map_err(|e| e.into())
    }
}
