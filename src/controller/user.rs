use crate::config::app_state::AppState;
use crate::entities::user;
use crate::errors::http_response_error::{HttpResponseError, MapHttpResponseError};
use crate::models::user_dto::UserDto;
use crate::register_module;
use crate::repositories::user_repository;
use crate::util::traits::from_model::FromModel;
use actix_web::web::Json;
use actix_web::{post, web, Responder};
use argon2::Argon2;
use password_hash::{PasswordHasher, SaltString};
use sea_orm::{ActiveModelTrait, ActiveValue};
use uuid::Uuid;

#[utoipa::path(
    post,
    tag = "Users",
    context_path = "/api/v1/user",
    request_body = UserDto,
    responses(
        (status = 200, description = "Ok", body = UserDto),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
)]
#[post("/create")]
async fn create(
    user: Json<UserDto>,
    data: web::Data<AppState>,
) -> Result<impl Responder, HttpResponseError> {
    user_repository::find_by_name(&data.db, user.name.as_str())
        .await
        .map_internal_error(Some("Failed to check if the user already exists"))?
        .ok_or(HttpResponseError::bad_request(Some("User already exists")))?;

    let salt = SaltString::generate(&mut rand::thread_rng());
    let hash = Argon2::default()
        .hash_password(
            user.password
                .as_ref()
                .ok_or(HttpResponseError::bad_request(Some(
                    "A password must be supplied",
                )))?
                .as_str()
                .as_bytes(),
            salt.as_salt(),
        )
        .map_internal_error(Some("Failed to hash the password"))?;

    let model = user::ActiveModel {
        name: ActiveValue::set(user.name.clone()),
        password: ActiveValue::set(hash.to_string()),
        salt: ActiveValue::set(salt.to_string()),
        external_id: ActiveValue::set(if let Some(id) = user.external_id.as_ref() {
            Some(
                Uuid::try_parse(id.as_str())
                    .map_internal_error(Some("Failed to parse the external id"))?,
            )
        } else {
            None
        }),
        ..Default::default()
    }
    .insert(&data.db)
    .await
    .map_internal_error(None)?;

    Ok(Json(UserDto::from_model(model)))
}

register_module!("/api/v1/user", create);
