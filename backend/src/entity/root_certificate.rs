use async_trait::async_trait;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, IntoActiveModel};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "root_certificate")]
pub struct Model {
    #[sea_orm(primary_key, unique, generated)]
    pub id: i32,
    pub public: Vec<u8>,
    pub active: bool,
    pub created_by: Uuid,
    pub valid_until: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_one = "super::user::Entity",
        belongs_to = "super::user::Entity",
        from = "Column::CreatedBy",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.created_at = ActiveValue::Set(Utc::now().into());
            self.active = ActiveValue::Set(true);

            for cert in Entity::find().all(db).await?.into_iter() {
                let mut cert = cert.into_active_model();
                cert.active = ActiveValue::Set(false);
                cert.save(db).await?;
            }
        }

        self.updated_at = ActiveValue::Set(Utc::now().into());
        Ok(self)
    }
}
