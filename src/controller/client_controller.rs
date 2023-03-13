use crate::config::app_state::AppState;
use crate::error::http_response_error::{HttpResponseError, MapHttpResponseError};
use crate::model::client_dto::ClientDto;
use crate::model::create_client_dto::CreateClientDto;
use crate::register_module;
use crate::util::traits::from_model::FromModel;
use crate::util::types::WebResult;
use actix_web::web::{Data, Json};
use actix_web::{post, Responder};
use actix_web_middleware_keycloak_auth::{StandardKeycloakClaims};
use jsonwebtoken::EncodingKey;
use openssl::sha::Sha256;
use sea_orm::ActiveValue;
use uuid::Uuid;
use crate::entity::client;
use crate::model::token_claims::TokenClaims;
use crate::util::traits::u8_vec_to_string::U8VecToString;

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
)]
#[post("/client")]
pub async fn create(
    client: Json<CreateClientDto>,
    data: Data<AppState>,
    claims: StandardKeycloakClaims,
) -> WebResult<impl Responder> {
    let user_id = Uuid::from_bytes(claims.sub.as_bytes().clone());
    data.user_service
        .find_by_external_id(&user_id.to_string(), false)
        .await?
        .ok_or(HttpResponseError::bad_request(Some("User not found")))?;

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &TokenClaims {
            sub: user_id.to_string(),
            iat: chrono::Utc::now().timestamp() as usize,
            exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
        },
        &EncodingKey::from_secret(data.config.jwt_secret.as_bytes()),
    ).map_internal_error(Some("Failed to encode jwt"))?;

    let token_hash = {
        let mut hash = Sha256::new();
        hash.update(token.as_bytes());
        hash.finish().to_vec().to_hex_string(":")
    };

    let client = data.client_service.insert(client::ActiveModel {
        name: ActiveValue::Set(client.name.clone()),
        user_id: ActiveValue::Set(user_id),
        token_hash: ActiveValue::Set(token_hash),
        ..Default::default()
    }).await?;
    
    let mut dto = ClientDto::from_model(client);
    dto.token = Some(token);
    Ok(Json(dto))
}

register_module!(create);
