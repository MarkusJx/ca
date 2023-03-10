use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "client")]
pub struct Model {
    #[sea_orm(primary_key, unique, generated, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique, indexed)]
    pub name: String,
    pub password: String,
    pub salt: String,
    pub external_id: Option<Uuid>,
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

impl ActiveModelBehavior for ActiveModel {}
