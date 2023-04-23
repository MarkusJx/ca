use crate::config::app_state::AppState;
use crate::entity::{client, token, user};
use crate::error::http_response_error::{HttpResponseError, MapHttpResponseError};
use crate::middleware::jwt_middleware::JwtMiddleware;
use crate::middleware::keycloak_roles::KeycloakRoles;
use crate::util::types::WebResult;
use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest};
use actix_web_middleware_keycloak_auth::StandardKeycloakClaims;
use futures_util::future::LocalBoxFuture;
use openssl::sha::Sha256;
use sea_orm::{ActiveValue, TryIntoModel};
use shared::util::traits::u8_vec_to_string::U8VecToString;
use uuid::Uuid;

pub struct KeycloakUserClaims<R: KeycloakRoles> {
    pub user: user::Model,
    _roles: std::marker::PhantomData<R>,
}

impl<R: KeycloakRoles> KeycloakUserClaims<R> {
    pub fn get_user_token(&self) -> WebResult<token::Model> {
        let hash = {
            let mut hash = Sha256::new();
            hash.update(self.user.id.as_bytes());
            hash.finish().to_vec().to_hex_string("")
        };

        token::ActiveModel {
            id: ActiveValue::Set(Uuid::nil()),
            client_id: ActiveValue::Set(self.user.id.clone()),
            active: ActiveValue::Set(true),
            token_hash: ActiveValue::Set(hash),
        }
        .try_into_model()
        .map_internal_error("Failed to create token model")
    }
}

impl<R> FromRequest for KeycloakUserClaims<R>
where
    R: KeycloakRoles,
{
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let data: &web::Data<AppState> = req
                .app_data()
                .ok_or(HttpResponseError::internal_error("App data not found"))?;
            let claims: StandardKeycloakClaims =
                StandardKeycloakClaims::from_request(&req, &mut Payload::None)
                    .await
                    .map_unauthorized("Failed to extract keycloak claims")?;
            let user_id = claims.sub.to_string();

            if !R::roles_match(
                &claims
                    .realm_access
                    .as_ref()
                    .ok_or(HttpResponseError::internal_error(
                        "Failed to get user roles",
                    ))?
                    .roles,
            ) {
                return Err(HttpResponseError::unauthorized("User not authorized").into());
            }

            Ok(KeycloakUserClaims {
                user: data
                    .user_service
                    .find_by_external_id(&user_id, false)
                    .await
                    .map_internal_error("Failed to find user")?
                    .ok_or(HttpResponseError::unauthorized("User not found"))?,
                _roles: std::marker::PhantomData,
            })
        })
    }
}

pub struct JwtClientClaims {
    pub client: client::Model,
}

impl FromRequest for JwtClientClaims {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let data: &web::Data<AppState> = req
                .app_data()
                .ok_or(HttpResponseError::internal_error("App data not found"))?;
            let jwt = JwtMiddleware::from_request(&req, &mut Payload::None).await?;

            let client_id = data
                .token_service
                .find_by_id(&jwt.id, false)
                .await
                .map_internal_error("Failed to find token by id")?
                .ok_or(HttpResponseError::unauthorized("Token not found"))?
                .client_id;

            Ok(JwtClientClaims {
                client: data
                    .client_service
                    .find_by_id(&client_id, false)
                    .await
                    .map_internal_error("Failed to find client")?
                    .ok_or(HttpResponseError::unauthorized("Client not found"))?,
            })
        })
    }
}
