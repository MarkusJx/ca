use crate::entity::user;
use crate::util::types::DbResult;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DbErr, DeleteResult, EntityTrait,
    ModelTrait, QueryFilter,
};
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_name<C: ConnectionTrait>(
        db: &C,
        name: &str,
    ) -> DbResult<Option<user::Model>> {
        user::Entity::find()
            .filter(user::Column::Name.eq(name))
            .filter(user::Column::Active.eq(true))
            .one(db)
            .await
    }

    pub async fn find_by_id<C: ConnectionTrait>(
        db: &C,
        id: &Uuid,
    ) -> DbResult<Option<user::Model>> {
        user::Entity::find_by_id(id.clone())
            .filter(user::Column::Active.eq(true))
            .one(db)
            .await
    }

    pub async fn find_all<C: ConnectionTrait>(db: &C) -> DbResult<Vec<user::Model>> {
        user::Entity::find()
            .filter(user::Column::Active.eq(true))
            .all(db)
            .await
    }

    pub async fn delete_by_id<C: ConnectionTrait>(db: &C, id: &Uuid) -> DbResult<DeleteResult> {
        Self::find_by_id(db, id)
            .await?
            .ok_or(DbErr::RecordNotFound(id.to_string()))?
            .delete(db)
            .await
    }

    pub async fn disable<C: ConnectionTrait>(
        db: &C,
        mut model: user::ActiveModel,
    ) -> DbResult<user::ActiveModel> {
        model.active = ActiveValue::Set(false);
        model.name = ActiveValue::Set(format!("{}-{}", model.name.as_ref(), Uuid::new_v4()));
        model.external_id = ActiveValue::Set(None);

        model.save(db).await
    }
}
