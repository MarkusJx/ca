use crate::entity::user;
use crate::util::types::DbResult;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DeleteResult, EntityTrait,
    ModelTrait, QueryFilter,
};
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_name<C: ConnectionTrait>(
        db: &C,
        name: &str,
        include_inactive: bool,
    ) -> DbResult<Option<user::Model>> {
        let mut q = user::Entity::find().filter(user::Column::Name.eq(name));
        if !include_inactive {
            q = q.filter(user::Column::Active.eq(true));
        }

        q.one(db).await
    }

    pub async fn find_by_id<C: ConnectionTrait>(
        db: &C,
        id: &Uuid,
        include_inactive: bool,
    ) -> DbResult<Option<user::Model>> {
        let mut q = user::Entity::find_by_id(id.clone());
        if !include_inactive {
            q = q.filter(user::Column::Active.eq(true));
        }
        q.one(db).await
    }

    pub async fn find_all<C: ConnectionTrait>(
        db: &C,
        include_inactive: bool,
    ) -> DbResult<Vec<user::Model>> {
        let mut q = user::Entity::find();
        if !include_inactive {
            q = q.filter(user::Column::Active.eq(true));
        }

        q.all(db).await
    }

    pub async fn find_by_external_id<C: ConnectionTrait>(
        db: &C,
        external_id: &str,
        include_inactive: bool,
    ) -> DbResult<Option<user::Model>> {
        let mut q = user::Entity::find().filter(user::Column::ExternalId.eq(external_id));
        if !include_inactive {
            q = q.filter(user::Column::Active.eq(true));
        }

        q.one(db).await
    }

    pub async fn delete<C: ConnectionTrait>(db: &C, model: user::Model) -> DbResult<DeleteResult> {
        model.delete(db).await
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
