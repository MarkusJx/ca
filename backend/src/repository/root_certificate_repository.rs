use crate::entity::root_certificate;
use crate::util::types::DbResult;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

pub struct RootCertificateRepository;

impl RootCertificateRepository {
    pub async fn insert<C: ConnectionTrait>(
        db: &C,
        model: root_certificate::ActiveModel,
    ) -> DbResult<root_certificate::Model> {
        model.insert(db).await.map_err(|e| e.into())
    }

    pub async fn find_active<C: ConnectionTrait>(
        db: &C,
    ) -> DbResult<Option<root_certificate::Model>> {
        root_certificate::Entity::find()
            .filter(root_certificate::Column::Active.eq(true))
            .one(db)
            .await
    }
}
