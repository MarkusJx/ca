use crate::entity::client;
use crate::util::types::DbResult;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
};
use uuid::Uuid;

pub struct ClientRepository;

impl ClientRepository {
    pub async fn find_by_name<C: ConnectionTrait>(
        db: &C,
        name: &str,
    ) -> DbResult<Option<client::Model>> {
        client::Entity::find()
            .filter(client::Column::Name.eq(name))
            .one(db)
            .await
            .map_err(|e| e.into())
    }

    pub async fn find_by_id<C: ConnectionTrait>(
        db: &C,
        id: &Uuid,
    ) -> DbResult<Option<client::Model>> {
        client::Entity::find_by_id(id.clone())
            .one(db)
            .await
            .map_err(|e| e.into())
    }

    pub async fn find_all_by_user<C: ConnectionTrait>(
        db: &C,
        user_id: &Uuid,
    ) -> DbResult<Vec<client::Model>> {
        client::Entity::find()
            .filter(client::Column::UserId.eq(user_id.clone()))
            .all(db)
            .await
            .map_err(|e| e.into())
    }

    pub async fn disable<C: ConnectionTrait>(
        db: &C,
        mut model: client::ActiveModel,
    ) -> DbResult<client::ActiveModel> {
        model.active = ActiveValue::Set(false);
        model.save(db).await.map_err(|e| e.into())
    }
}
