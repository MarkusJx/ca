use crate::entity::client;
use crate::util::types::DbResult;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DeleteResult, EntityTrait,
    QueryFilter,
};
use uuid::Uuid;

pub struct ClientRepository;

impl ClientRepository {
    pub async fn insert<C: ConnectionTrait>(
        db: &C,
        model: client::ActiveModel,
    ) -> DbResult<client::Model> {
        model.insert(db).await.map_err(|e| e.into())
    }

    pub async fn find_by_name<C: ConnectionTrait>(
        db: &C,
        name: &str,
    ) -> DbResult<Option<client::Model>> {
        client::Entity::find()
            .filter(client::Column::Name.eq(name))
            .one(db)
            .await
    }

    pub async fn find_by_id<C: ConnectionTrait>(
        db: &C,
        id: &Uuid,
        include_inactive: bool,
    ) -> DbResult<Option<client::Model>> {
        let mut q = client::Entity::find_by_id(id.clone());
        if !include_inactive {
            q = q.filter(client::Column::Active.eq(true));
        }

        q.one(db).await
    }

    pub async fn find_all_by_user<C: ConnectionTrait>(
        db: &C,
        user_id: &Uuid,
        include_inactive: bool,
    ) -> DbResult<Vec<client::Model>> {
        let mut q = client::Entity::find().filter(client::Column::UserId.eq(user_id.clone()));
        if !include_inactive {
            q = q.filter(client::Column::Active.eq(true));
        }

        q.all(db).await
    }

    pub async fn delete<C: ConnectionTrait>(
        db: &C,
        model: client::ActiveModel,
    ) -> DbResult<DeleteResult> {
        model.delete(db).await
    }

    pub async fn disable<C: ConnectionTrait>(
        db: &C,
        mut model: client::ActiveModel,
    ) -> DbResult<client::ActiveModel> {
        model.active = ActiveValue::Set(false);
        model.save(db).await
    }
}
