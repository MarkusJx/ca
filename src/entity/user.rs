use crate::repository::client_repository::ClientRepository;
use crate::repository::user_repository::UserRepository;
use crate::util::types::DbResult;
use async_trait::async_trait;
use chrono::Utc;
use futures_util::future::join_all;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, unique, generated, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique, indexed)]
    pub name: String,
    pub external_id: Option<String>,
    #[sea_orm(default = "true")]
    pub active: bool,
    #[sea_orm(auto_timestamp = "created_at")]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(auto_timestamp = "created_at")]
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::client::Entity")]
    Client,
}

impl Related<super::client::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Client.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, db: &C, insert: bool) -> DbResult<Self>
    where
        C: ConnectionTrait,
    {
        if insert {
            let mut id = Uuid::new_v4();
            while let Some(_) = UserRepository::find_by_id(db, &id, true).await? {
                id = Uuid::new_v4();
            }

            self.active = ActiveValue::Set(true);
            self.id = ActiveValue::Set(id.into());
            self.created_at = ActiveValue::Set(Utc::now().into());
        }

        if !self.active.as_ref() {
            join_all(
                ClientRepository::find_all_by_user(db, self.id.as_ref())
                    .await?
                    .into_iter()
                    .map(|c| ClientRepository::disable(db, c.into())),
            )
            .await
            .into_iter()
            .collect::<DbResult<Vec<_>>>()?;
        }

        self.updated_at = ActiveValue::Set(Utc::now().into());
        Ok(self)
    }

    async fn before_delete<C>(self, db: &C) -> DbResult<Self>
    where
        C: ConnectionTrait,
    {
        join_all(
            ClientRepository::find_all_by_user(db, self.id.as_ref())
                .await?
                .into_iter()
                .map(|c| c.delete(db)),
        )
        .await
        .into_iter()
        .collect::<DbResult<Vec<_>>>()?;

        Ok(self)
    }
}
