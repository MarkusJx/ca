use crate::entity::root_certificate;
use crate::util::types::DbResult;
use sea_orm::{ActiveModelTrait, ConnectionTrait};

pub struct RootCertificateRepository;

impl RootCertificateRepository {
    pub async fn insert<C: ConnectionTrait>(
        db: &C,
        model: root_certificate::ActiveModel,
    ) -> DbResult<root_certificate::Model> {
        model.insert(db).await.map_err(|e| e.into())
    }
}
