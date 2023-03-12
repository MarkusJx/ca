use crate::repository::client_repository::ClientRepository;
use crate::repository::signing_request_repository::SigningRequestRepository;
use crate::util::types::DbResult;
use async_trait::async_trait;
use chrono::Utc;
use futures_util::future::join_all;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "client")]
pub struct Model {
    #[sea_orm(primary_key, unique, generated, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    pub user_id: Uuid,
    #[sea_orm(unique)]
    pub name: String,
    pub active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_one = "super::user::Entity",
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "super::signing_request::Entity")]
    SigningRequest,
}

impl Related<super::signing_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SigningRequest.def()
    }
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
            let mut id = Uuid::new_v4();
            while let Some(_) = ClientRepository::find_by_id(db, &id).await? {
                id = Uuid::new_v4();
            }

            self.active = ActiveValue::Set(true);
            self.id = ActiveValue::Set(id);
            self.created_at = ActiveValue::Set(Utc::now().into());
        }

        if !self.active.as_ref() {
            join_all(
                SigningRequestRepository::find_all_by_client(db, self.id.as_ref())
                    .await?
                    .into_iter()
                    .map(|sr| sr.delete(db)),
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
            SigningRequestRepository::find_all_by_client(db, self.id.as_ref())
                .await?
                .into_iter()
                .map(|sr| sr.delete(db)),
        )
        .await
        .into_iter()
        .collect::<DbResult<Vec<_>>>()?;

        Ok(self)
    }
}
