use async_trait::async_trait;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, IntoActiveModel};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "certificate")]
pub struct Model {
    #[sea_orm(primary_key, unique, generated)]
    pub id: i32,
    pub public: Vec<u8>,
    pub private: Option<Vec<u8>>,
    pub active: bool,
    pub valid_until: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            if self.private.is_not_set() || self.private.as_ref().is_none() {
                return Err(DbErr::Custom("Private key is required".to_string()));
            }

            self.created_at = ActiveValue::Set(Utc::now().into());
            self.active = ActiveValue::Set(true);

            for cert in Entity::find().all(db).await?.into_iter() {
                let mut cert = cert.into_active_model();
                cert.active = ActiveValue::Set(false);
                cert.save(db).await?;
            }
        }

        if !self.active.is_unchanged() && !*self.active.as_ref() && self.private.as_ref().is_some()
        {
            self.private = ActiveValue::Set(None);
        } else if *self.active.as_ref()
            && (self.private.is_not_set() || self.private.as_ref().is_none())
        {
            return Err(DbErr::Custom("Private key is required".to_string()));
        }

        self.updated_at = ActiveValue::Set(Utc::now().into());
        Ok(self)
    }
}
