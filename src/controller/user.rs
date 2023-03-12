use crate::config::app_state::AppState;
use crate::entity::user;
use crate::error::http_response_error::HttpResponseError;
use crate::model::create_user_dto::CreateUserDto;
use crate::model::user_dto::UserDto;
use crate::register_module;
use crate::util::types::WebResult;
use actix_web::web::Json;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use futures_util::future::join_all;
use keycloak::types::{CredentialRepresentation, UserRepresentation};
use log::debug;
use sea_orm::{ActiveValue, IntoActiveModel};

#[utoipa::path(
    post,
    tag = "Users",
    context_path = "/api/v1",
    request_body = CreateUserDto,
    responses(
        (status = 200, description = "Ok", body = UserDto),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 424, description = "Failed dependency", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
)]
#[post("/user")]
async fn create(
    user: Json<CreateUserDto>,
    data: web::Data<AppState>,
) -> Result<impl Responder, HttpResponseError> {
    if user.name.len() < 3 || user.password.len() < 3 {
        return Err(HttpResponseError::bad_request(Some(
            "The username and password must be at least 3 characters long",
        )));
    }

    data.user_service
        .find_by_name(user.name.as_str())
        .await?
        .map(|_| Err(HttpResponseError::bad_request(Some("User already exists"))))
        .unwrap_or(Ok(()))?;

    let kc_users = data
        .keycloak_service
        .get_users(user.name.clone(), true)
        .await?;

    let kc_user = kc_users.first();
    let kc_user = if let Some(kc_user) = kc_user {
        if !kc_user.username.is_some() || kc_user.username.as_ref().unwrap() != user.name.as_str() {
            return Err(HttpResponseError::bad_request(Some(
                "The username and name of the user do not match",
            )));
        }

        kc_user.clone()
    } else {
        debug!("Creating user in keycloak");
        data.keycloak_service
            .create_user(UserRepresentation {
                username: Some(user.name.clone()),
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                enabled: Some(true),
                email: user.email.clone(),
                email_verified: Some(
                    user.email.is_some() && data.config.keycloak_default_email_verified,
                ),
                credentials: Some(vec![CredentialRepresentation {
                    value: Some(user.password.clone()),
                    temporary: Some(data.config.keycloak_passwords_temporary),
                    type_: Some("password".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            })
            .await?;

        data.keycloak_service
            .get_users(user.name.clone(), true)
            .await?
            .first()
            .map(|kc_user| kc_user.clone())
            .ok_or_else(|| {
                HttpResponseError::failed_dependency(Some(
                    "Failed to get the created user from keycloak",
                ))
            })?
    };

    let model = data
        .user_service
        .insert(user::ActiveModel {
            name: ActiveValue::set(user.name.clone()),
            external_id: ActiveValue::set(Some(kc_user.id.clone().ok_or(
                HttpResponseError::internal_error(Some(
                    "Failed to get the external id of the user",
                )),
            )?)),
            ..Default::default()
        })
        .await?;

    Ok(Json(UserDto::from_model(model, kc_user)))
}

#[utoipa::path(
    get,
    tag = "Users",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Ok", body = Vec<UserDto>),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 424, description = "Failed dependency", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
)]
#[get("/user/list")]
async fn list(data: web::Data<AppState>) -> Result<impl Responder, HttpResponseError> {
    async fn map_user(
        model: user::Model,
        data: &web::Data<AppState>,
    ) -> Result<(user::Model, UserRepresentation), HttpResponseError> {
        let kc_user = data
            .keycloak_service
            .get_user_by_id(model.external_id.as_ref().unwrap())
            .await?;

        Ok((model, kc_user))
    }

    Ok(Json(
        join_all(
            data.user_service
                .find_all()
                .await?
                .into_iter()
                .map(|model| map_user(model, &data)),
        )
        .await
        .into_iter()
        .collect::<WebResult<Vec<_>>>()?
        .into_iter()
        .map(|(model, kc_user)| UserDto::from_model(model, kc_user))
        .collect::<Vec<_>>(),
    ))
}

#[utoipa::path(
    get,
    tag = "Users",
    context_path = "/api/v1",
    params(
        ("id", description = "Id of the user to find")
    ),
    responses(
        (status = 200, description = "Ok", body = UserDto),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 404, description = "Not found", body = ErrorDto),
        (status = 424, description = "Failed dependency", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
)]
#[get("/user/{id}")]
async fn get(
    id: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<impl Responder, HttpResponseError> {
    let model = data
        .user_service
        .find_by_id_string_unwrap(&id.into_inner())
        .await?;

    let kc_user = data
        .keycloak_service
        .get_user_by_id(model.external_id.as_ref().unwrap())
        .await?;

    Ok(Json(UserDto::from_model(model, kc_user)))
}

#[utoipa::path(
    delete,
    tag = "Users",
    context_path = "/api/v1",
    params(
        ("id", description = "Id of the user to delete")
    ),
    responses(
        (status = 204, description = "User deleted"),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 404, description = "Not found", body = ErrorDto),
        (status = 424, description = "Failed dependency", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
)]
#[delete("/user/{id}")]
async fn delete(
    id: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<impl Responder, HttpResponseError> {
    let user = data
        .user_service
        .find_by_id_string_unwrap(&id.into_inner())
        .await?;

    data.keycloak_service
        .delete_user(user.external_id.as_ref().unwrap())
        .await?;
    data.user_service.disable(user.into()).await?;

    Ok(HttpResponse::NoContent().finish())
}

register_module!(create, list, get, delete);
