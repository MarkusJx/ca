use crate::repository::token_repository::TokenRepository;
use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "token")]
pub struct Model {
    #[sea_orm(primary_key, unique, generated, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    pub client_id: Uuid,
    pub token_hash: String,
    pub active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_one = "super::client::Entity",
        belongs_to = "super::client::Entity",
        from = "Column::ClientId",
        to = "super::client::Column::Id"
    )]
    Client,
}

impl Related<super::client::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Client.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.active = ActiveValue::Set(true);
            TokenRepository::deactivate_all_by_client(db, self.client_id.as_ref()).await?;
        } else if !self.active.is_unchanged() && *self.active.as_ref() {
            return Err(DbErr::Custom("Cannot activate token".to_string()));
        }

        Ok(self)
    }
}
