use crate::entity::user;
use keycloak::types::UserRepresentation;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserDto {
    pub id: String,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub active: bool,
    pub email: Option<String>,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub roles: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl UserDto {
    pub fn from_model(model: user::Model, kc_user: Option<UserRepresentation>) -> Self {
        Self {
            id: model.id.to_string(),
            name: model.name.clone(),
            display_name: model.original_name,
            active: model.active,
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
            email: kc_user.as_ref().and_then(|u| u.email.clone()),
            roles: kc_user
                .as_ref()
                .and_then(|u| u.realm_roles.clone())
                .unwrap_or_default(),
            first_name: kc_user.as_ref().and_then(|u| u.first_name.clone()),
            last_name: kc_user.and_then(|u| u.last_name),
        }
    }
}
