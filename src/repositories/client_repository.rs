use crate::entities::client;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::error::Error;

pub async fn find_by_name(
    db: &DatabaseConnection,
    name: &str,
) -> Result<Option<client::Model>, Box<dyn Error>> {
    client::Entity::find()
        .filter(client::Column::Name.eq(name))
        .one(db)
        .await
        .map_err(|e| e.into())
}
