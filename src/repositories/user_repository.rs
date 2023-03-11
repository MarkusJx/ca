use crate::entities::user;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::error::Error;

pub async fn find_by_name(
    db: &DatabaseConnection,
    name: &str,
) -> Result<Option<user::Model>, Box<dyn Error>> {
    user::Entity::find()
        .filter(user::Column::Name.eq(name))
        .one(db)
        .await
        .map_err(|e| e.into())
}
