use crate::entities::user;
use crate::util::traits::from_model::FromModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserDto {
    pub id: Option<String>,
    pub external_id: Option<String>,
    pub name: String,
    //pub email: String,
    pub password: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl FromModel<user::Model> for UserDto {
    fn from_model(model: user::Model) -> Self {
        Self {
            id: Some(model.id.to_string()),
            external_id: model.external_id.map(|id| id.to_string()),
            name: model.name,
            //email: model.email,
            password: Some(model.password),
            created_at: None,
            updated_at: None,
        }
    }
}
