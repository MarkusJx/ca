use crate::entity::certificate;
use crate::util::types::DbResult;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter};

pub struct CertificateRepository;

impl CertificateRepository {
    pub async fn find_active<C>(db: &C) -> DbResult<Option<certificate::Model>>
    where
        C: ConnectionTrait,
    {
        let mut res = certificate::Entity::find()
            .filter(certificate::Column::Active.eq(true))
            .all(db)
            .await?;

        if res.len() > 1 {
            return Err(DbErr::Custom(
                "More than one active certificate found".into(),
            ));
        }

        Ok(res.pop())
    }

    pub async fn insert<C>(db: &C, model: certificate::ActiveModel) -> DbResult<certificate::Model>
    where
        C: ConnectionTrait,
    {
        model.insert(db).await
    }
}
