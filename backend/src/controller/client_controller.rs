use crate::config::app_state::AppState;
use crate::entity::{client, token};
use crate::error::http_response_error::{HttpResponseError, MapHttpResponseError};
use crate::middleware::extractors::KeycloakUserClaims;
use crate::middleware::keycloak_middleware;
use crate::model::client_dto::ClientDto;
use crate::model::create_client_dto::CreateClientDto;
use crate::model::token_claims::TokenClaims;
use crate::register_module;
use crate::util::types::WebResult;
use actix_web::web::{Data, Json, Path, Query};
use actix_web::{delete, get, post, put, HttpResponse, Responder};
use jsonwebtoken::EncodingKey;
use log::debug;
use openssl::sha::Sha256;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveValue, IntoActiveModel};
use serde::Deserialize;
use shared::util::traits::u8_vec_to_string::U8VecToString;
use utoipa::IntoParams;
use uuid::Uuid;

#[derive(Deserialize, Debug, IntoParams)]
pub struct ClientQuery {
    /// Whether to include clients users in the result.
    /// Defaults to false.
    #[serde(rename = "includeInactive")]
    pub include_inactive: Option<bool>,
}

#[derive(Deserialize, Debug, IntoParams)]
struct DeleteQuery {
    /// Whether to delete the client rather than just deactivating it.
    /// Defaults to false.
    #[serde(rename = "deleteInDatabase")]
    pub delete_in_database: Option<bool>,
}

#[utoipa::path(
    post,
    tag = "Clients",
    context_path = "/api/v1",
    request_body = CreateClientDto,
    responses(
        (status = 200, description = "Ok", body = ClientDto),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 424, description = "Failed dependency", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
    security(
        ("oauth2" = [])
    )
)]
#[post("/client", wrap = "keycloak_middleware::Keycloak")]
async fn create(
    client: Json<CreateClientDto>,
    data: Data<AppState>,
    claims: KeycloakUserClaims,
) -> WebResult<Json<ClientDto>> {
    debug!("Creating client for user {}", claims.user.id);

    let expiry_date = DateTimeWithTimeZone::parse_from_rfc3339(&client.valid_until)
        .map_bad_request(Some("Invalid date supplied"))?;
    if expiry_date < chrono::Utc::now() {
        return Err(HttpResponseError::bad_request(Some(
            "Expiry date must be in the future",
        )));
    }

    let token_id = data.token_service.generate_id().await?;
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &TokenClaims {
            sub: token_id.to_string(),
            iat: chrono::Utc::now().timestamp() as usize,
            exp: expiry_date.timestamp() as usize,
        },
        &EncodingKey::from_secret(data.config.jwt_secret.as_bytes()),
    )
    .map_internal_error(Some("Failed to encode jwt"))?;

    let token_hash = {
        let mut hash = Sha256::new();
        hash.update(token.as_bytes());
        hash.finish().to_vec().to_hex_string("")
    };

    let client = data
        .client_service
        .insert(client::ActiveModel {
            id: ActiveValue::Set(data.client_service.generate_id().await?),
            name: ActiveValue::Set(client.name.clone().ok_or(HttpResponseError::bad_request(
                Some("Client name must be supplied"),
            ))?),
            user_id: ActiveValue::Set(claims.user.id),
            valid_until: ActiveValue::Set(expiry_date),
            ..Default::default()
        })
        .await?;

    let token_entity = data
        .token_service
        .insert(token::ActiveModel {
            id: ActiveValue::Set(token_id),
            token_hash: ActiveValue::Set(token_hash),
            client_id: ActiveValue::Set(client.id),
            ..Default::default()
        })
        .await?;

    let mut dto = ClientDto::from_model(client, token_entity);
    dto.token = Some(token);
    Ok(Json(dto))
}

#[utoipa::path(
    get,
    tag = "Clients",
    context_path = "/api/v1",
    params(
        ("id", description = "Client id"),
        ClientQuery
    ),
    responses(
        (status = 200, description = "Ok", body = ClientDto),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 424, description = "Failed dependency", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
    security(
        ("oauth2" = [])
    )
)]
#[get("/client/{id}", wrap = "keycloak_middleware::Keycloak")]
async fn by_id(
    data: Data<AppState>,
    path: Path<String>,
    query: Query<ClientQuery>,
    claims: KeycloakUserClaims,
) -> WebResult<Json<ClientDto>> {
    let include_inactive = query.include_inactive.unwrap_or(false);
    let client_id = Uuid::parse_str(&path).map_bad_request(Some("Invalid client id supplied"))?;
    let client = data
        .client_service
        .find_by_id(&client_id, include_inactive)
        .await?
        .ok_or(HttpResponseError::not_found(Some("Client not found")))?;

    if client.user_id != claims.user.id {
        return Err(HttpResponseError::unauthorized(Some(
            "You are not authorized to access this client",
        )));
    }

    let token_entity = data
        .token_service
        .find_by_client_id(&client_id, include_inactive)
        .await?
        .ok_or(HttpResponseError::not_found(Some("Token not found")))?;

    Ok(Json(ClientDto::from_model(client, token_entity)))
}

#[utoipa::path(
    get,
    tag = "Clients",
    context_path = "/api/v1",
    params(ClientQuery),
    responses(
        (status = 200, description = "Ok", body = Vec<ClientDto>),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 424, description = "Failed dependency", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
    security(
        ("oauth2" = [])
    )
)]
#[get("/client/list", wrap = "keycloak_middleware::Keycloak")]
async fn list(
    data: Data<AppState>,
    query: Query<ClientQuery>,
    claims: KeycloakUserClaims,
) -> WebResult<Json<Vec<ClientDto>>> {
    let include_inactive = query.include_inactive.unwrap_or(false);
    let clients = data
        .client_service
        .find_all_by_user(&claims.user.id, include_inactive)
        .await?;

    let mut res = Vec::with_capacity(clients.len());
    for client in clients {
        let token_entity = data
            .token_service
            .find_by_client_id(&client.id, include_inactive)
            .await?
            .ok_or(HttpResponseError::not_found(Some("Token not found")))?;

        res.push(ClientDto::from_model(client, token_entity))
    }

    Ok(Json(res))
}

#[utoipa::path(
    put,
    tag = "Clients",
    context_path = "/api/v1",
    request_body = CreateClientDto,
    params(
        ("id", description = "Id of the client to update")
    ),
    responses(
        (status = 200, description = "Ok", body = ClientDto),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 424, description = "Failed dependency", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
    security(
        ("oauth2" = [])
    )
)]
#[put("/client/regenerate/{id}", wrap = "keycloak_middleware::Keycloak")]
async fn regenerate_token(
    data: Data<AppState>,
    id: Path<String>,
    client: Json<CreateClientDto>,
    claims: KeycloakUserClaims,
) -> WebResult<Json<ClientDto>> {
    let client_entity = data
        .client_service
        .find_by_id_string_unwrap(id.as_ref(), false)
        .await?;

    let expiry_date = DateTimeWithTimeZone::parse_from_rfc3339(&client.valid_until)
        .map_bad_request(Some("Invalid date supplied"))?;
    if expiry_date < chrono::Utc::now() {
        return Err(HttpResponseError::bad_request(Some(
            "Expiry date must be in the future",
        )));
    }

    if client_entity.user_id != claims.user.id {
        return Err(HttpResponseError::bad_request(Some("Client not found")));
    }

    data.token_service
        .deactivate_all_by_client_id(&client_entity.id)
        .await?;
    let token_id = data.token_service.generate_id().await?;
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &TokenClaims {
            sub: token_id.to_string(),
            iat: chrono::Utc::now().timestamp() as usize,
            exp: expiry_date.timestamp() as usize,
        },
        &EncodingKey::from_secret(data.config.jwt_secret.as_bytes()),
    )
    .map_internal_error(Some("Failed to encode jwt"))?;

    let token_hash = {
        let mut hash = Sha256::new();
        hash.update(token.as_bytes());
        hash.finish().to_vec().to_hex_string("")
    };

    let token_entity = data
        .token_service
        .insert(token::ActiveModel {
            id: ActiveValue::Set(token_id),
            token_hash: ActiveValue::Set(token_hash),
            client_id: ActiveValue::Set(client_entity.id),
            ..Default::default()
        })
        .await?;

    let mut client_entity = client_entity.into_active_model();
    client_entity.valid_until = ActiveValue::Set(expiry_date);
    let client_entity = data.client_service.update(client_entity).await?;

    let mut dto = ClientDto::from_model(client_entity, token_entity);
    dto.token = Some(token);
    Ok(Json(dto))
}

#[utoipa::path(
    delete,
    tag = "Clients",
    context_path = "/api/v1",
    params(
        ("id", description = "Id of the client to delete"),
        DeleteQuery
    ),
    responses(
        (status = 204, description = "Client deleted"),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 424, description = "Failed dependency", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
    security(
        ("oauth2" = [])
    )
)]
#[delete("/client/{id}", wrap = "keycloak_middleware::Keycloak")]
async fn delete(
    data: Data<AppState>,
    path: Path<String>,
    query: Query<DeleteQuery>,
    claims: KeycloakUserClaims,
) -> WebResult<impl Responder> {
    let client = data
        .client_service
        .find_by_id_string_unwrap(path.as_ref(), true)
        .await?;

    if client.user_id != claims.user.id {
        return Err(HttpResponseError::bad_request(Some("Client not found")));
    }

    if query.delete_in_database.unwrap_or(false) {
        data.client_service
            .delete(client.into_active_model())
            .await?
            .rows_affected
            .ge(&1)
            .then(|| ())
            .ok_or(HttpResponseError::bad_request(Some(
                "Failed to delete client",
            )))?;
    } else {
        data.client_service
            .disable(client.into_active_model())
            .await?;
    }

    Ok(HttpResponse::NoContent().finish())
}

register_module!(create, regenerate_token, list, by_id, delete);
