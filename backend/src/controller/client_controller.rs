use crate::config::app_state::AppState;
use crate::entity::{client, token};
use crate::error::http_response_error::{HttpResponseError, MapHttpResponseError};
use crate::middleware::extractors::KeycloakUserClaims;
use crate::middleware::keycloak_middleware;
use crate::middleware::keycloak_roles::NoRoles;
use crate::model::client_dto::ClientDto;
use crate::model::create_client_dto::CreateClientDto;
use crate::model::token_claims::TokenClaims;
use crate::register_module;
use crate::util::types::WebResult;
use actix_web::web::{Data, Json, Path, Query};
use actix_web::{delete, get, post, put, HttpResponse, Responder};
use chrono::{DateTime, FixedOffset};
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
    /// Whether to include inactive clients in the result.
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

async fn create_token(
    client: &Json<CreateClientDto>,
    data: &Data<AppState>,
) -> WebResult<(DateTime<FixedOffset>, Uuid, String, String)> {
    let expiry_date = DateTimeWithTimeZone::parse_from_rfc3339(&client.valid_until)
        .map_bad_request("Invalid date supplied")?;
    if expiry_date < chrono::Utc::now() {
        return Err(HttpResponseError::bad_request(
            "The expiry date must be in the future",
        ));
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
    .map_internal_error("Failed to encode jwt")?;

    let token_hash = {
        let mut hash = Sha256::new();
        hash.update(token.as_bytes());
        hash.finish().to_vec().to_hex_string("")
    };

    Ok((expiry_date, token_id, token, token_hash))
}

async fn create_token_entity(
    data: &Data<AppState>,
    token_id: Uuid,
    token_hash: String,
    client_id: Uuid,
) -> WebResult<token::Model> {
    data.token_service
        .insert(token::ActiveModel {
            id: ActiveValue::Set(token_id),
            token_hash: ActiveValue::Set(token_hash),
            client_id: ActiveValue::Set(client_id),
            ..Default::default()
        })
        .await
}

#[utoipa::path(
    post,
    tag = "Clients",
    context_path = "/api/v1",
    request_body = CreateClientDto,
    operation_id = "createClient",
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
    claims: KeycloakUserClaims<NoRoles>,
) -> WebResult<Json<ClientDto>> {
    debug!("Creating client for user {}", claims.user.id);

    let client_name = client.name.clone().ok_or(HttpResponseError::bad_request(
        "Client name must be supplied",
    ))?;

    data.client_service
        .find_by_name(&client_name)
        .await?
        .map(|_| {
            Err(HttpResponseError::bad_request(
                "A client with that name already exists",
            ))
        })
        .unwrap_or(Ok(()))?;

    let (expiry_date, token_id, token, token_hash) = create_token(&client, &data).await?;

    let client = data
        .client_service
        .insert(client::ActiveModel {
            id: ActiveValue::Set(data.client_service.generate_id().await?),
            name: ActiveValue::Set(client_name.clone()),
            user_id: ActiveValue::Set(claims.user.id),
            valid_until: ActiveValue::Set(Some(expiry_date)),
            ..Default::default()
        })
        .await?;

    let token_entity = create_token_entity(&data, token_id, token_hash, client.id).await?;
    Ok(Json(ClientDto::from_model_with_token(
        client,
        token_entity,
        token,
    )))
}

#[utoipa::path(
    put,
    tag = "Clients",
    context_path = "/api/v1",
    request_body = CreateClientDto,
    operation_id = "regenerateClientToken",
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
    claims: KeycloakUserClaims<NoRoles>,
) -> WebResult<Json<ClientDto>> {
    let client_entity = data
        .client_service
        .find_by_id_string_unwrap(id.as_ref(), false)
        .await?;

    if client_entity.user_id != claims.user.id {
        return Err(HttpResponseError::not_found(
            "User is not the owner of the client",
        ));
    }

    let (expiry_date, token_id, token, token_hash) = create_token(&client, &data).await?;
    data.token_service
        .deactivate_all_by_client_id(&client_entity.id)
        .await?;
    let token_entity = create_token_entity(&data, token_id, token_hash, client_entity.id).await?;

    let client_entity = {
        let mut entity = client_entity.into_active_model();
        entity.valid_until = ActiveValue::Set(Some(expiry_date));
        data.client_service.update(entity).await?
    };

    Ok(Json(ClientDto::from_model_with_token(
        client_entity,
        token_entity,
        token,
    )))
}

#[utoipa::path(
    get,
    tag = "Clients",
    context_path = "/api/v1",
    operation_id = "getClientById",
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
    claims: KeycloakUserClaims<NoRoles>,
) -> WebResult<Json<ClientDto>> {
    let include_inactive = query.include_inactive.unwrap_or(false);
    let client_id = Uuid::parse_str(&path).map_bad_request("Invalid client id supplied")?;
    let client = data
        .client_service
        .find_by_id(&client_id, include_inactive)
        .await?
        .ok_or(HttpResponseError::not_found("Client not found"))?;

    if client.user_id != claims.user.id {
        return Err(HttpResponseError::not_found(
            "You are not authorized to access this client",
        ));
    }

    let token_entity = if client.is_user_client {
        claims.get_user_token()?
    } else {
        data.token_service
            .find_by_client_id(&client_id, include_inactive)
            .await?
            .ok_or(HttpResponseError::not_found("Token not found"))?
    };

    Ok(Json(ClientDto::from_model(client, token_entity)))
}

#[utoipa::path(
    get,
    tag = "Clients",
    context_path = "/api/v1",
    operation_id = "listClients",
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
    claims: KeycloakUserClaims<NoRoles>,
) -> WebResult<Json<Vec<ClientDto>>> {
    let include_inactive = query.include_inactive.unwrap_or(false);
    let clients = data
        .client_service
        .find_all_by_user(&claims.user.id, include_inactive)
        .await?;

    let mut res = Vec::with_capacity(clients.len());
    for client in clients {
        let token_entity = if client.is_user_client {
            claims.get_user_token()?
        } else {
            data.token_service
                .find_by_client_id(&client.id, include_inactive)
                .await?
                .ok_or(HttpResponseError::not_found("Token not found"))?
        };

        res.push(ClientDto::from_model(client, token_entity))
    }

    Ok(Json(res))
}

#[utoipa::path(
    delete,
    tag = "Clients",
    context_path = "/api/v1",
    operation_id = "deleteClient",
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
    claims: KeycloakUserClaims<NoRoles>,
) -> WebResult<impl Responder> {
    let client = data
        .client_service
        .find_by_id_string_unwrap(path.as_ref(), true)
        .await?;

    if client.is_user_client {
        return Err(HttpResponseError::bad_request(
            "User client cannot be deleted",
        ));
    } else if client.user_id != claims.user.id {
        return Err(HttpResponseError::not_found(
            "This client does not belong to you",
        ));
    }

    if query.delete_in_database.unwrap_or(false) {
        data.client_service
            .delete(client.into_active_model())
            .await?
            .rows_affected
            .ge(&1)
            .then(|| ())
            .ok_or(HttpResponseError::bad_request("Failed to delete client"))?;
    } else if client.active {
        data.client_service
            .disable(client.into_active_model())
            .await?;
    } else {
        return Err(HttpResponseError::bad_request("Client is already inactive"));
    }

    Ok(HttpResponse::NoContent().finish())
}

register_module!(create, regenerate_token, list, by_id, delete);
