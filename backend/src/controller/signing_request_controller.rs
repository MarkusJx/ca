use crate::config::app_state::AppState;
use crate::error::http_response_error::{HttpResponseError, MapHttpResponseError};
use crate::middleware::extractors::KeycloakUserClaims;
use crate::middleware::keycloak_middleware;
use crate::middleware::keycloak_roles::NoRoles;
use crate::register_module;
use crate::util::traits::from_model::FromModel;
use crate::util::types::WebResult;
use actix_web::get;
use actix_web::web;
use actix_web::web::Json;
use shared::model::signing_request_dto::SigningRequestDto;
use std::str::FromStr;
use uuid::Uuid;

#[utoipa::path(
    get,
    tag = "Signing requests",
    context_path = "/api/v1",
    operation_id = "getSigningRequestsByClientId",
    params(
        ("id", description = "The id of the client")
    ),
    responses(
        (status = 200, description = "Ok", body = Vec<SigningRequestDto>),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 404, description = "Not found", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
    security(
        ("oauth2" = [])
    )
)]
#[get("/signing-request/{id}", wrap = "keycloak_middleware::Keycloak")]
async fn by_client_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
    claims: KeycloakUserClaims<NoRoles>,
) -> WebResult<Json<Vec<SigningRequestDto>>> {
    let client_id = Uuid::from_str(&id).map_bad_request(Some("Invalid client id supplied"))?;
    let client = data
        .client_service
        .find_by_id(&client_id, false)
        .await?
        .ok_or(HttpResponseError::not_found(Some("Client not found")))?;

    if client.user_id != claims.user.id {
        return Err(HttpResponseError::unauthorized(Some(
            "User is not allowed to access this resource",
        )));
    }

    Ok(Json(
        data.signing_request_service
            .find_all_by_client_id(&client_id)
            .await
            .map_internal_error(Some("Failed to get signing requests"))?
            .into_iter()
            .map(|r| SigningRequestDto::from_model(r))
            .collect(),
    ))
}

#[utoipa::path(
    get,
    tag = "Signing requests",
    context_path = "/api/v1",
    operation_id = "getSigningRequests",
    responses(
        (status = 200, description = "Ok", body = Vec<SigningRequestDto>),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
    security(
        ("oauth2" = [])
    )
)]
#[get("/signing-request", wrap = "keycloak_middleware::Keycloak")]
async fn get_all(
    data: web::Data<AppState>,
    claims: KeycloakUserClaims<NoRoles>,
) -> WebResult<Json<Vec<SigningRequestDto>>> {
    Ok(Json(
        data.signing_request_service
            .find_all_by_user_id(&claims.user.id)
            .await
            .map_internal_error(Some("Failed to get signing requests"))?
            .into_iter()
            .map(|r| SigningRequestDto::from_model(r))
            .collect(),
    ))
}

register_module!(by_client_id, get_all);
