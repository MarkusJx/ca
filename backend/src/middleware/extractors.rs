use crate::config::app_state::AppState;
use crate::entity::{client, user};
use crate::error::http_response_error::{HttpResponseError, MapHttpResponseError};
use crate::middleware::jwt_middleware::JwtMiddleware;
use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest};
use actix_web_middleware_keycloak_auth::StandardKeycloakClaims;
use futures_util::future::LocalBoxFuture;

pub struct KeycloakUserClaims {
    pub user: user::Model,
}

impl FromRequest for KeycloakUserClaims {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let data: &web::Data<AppState> =
                req.app_data()
                    .ok_or(HttpResponseError::internal_error(Some(
                        "App data not found",
                    )))?;
            let claims: StandardKeycloakClaims =
                StandardKeycloakClaims::from_request(&req, &mut Payload::None).await?;
            let user_id = claims.sub.to_string();

            Ok(KeycloakUserClaims {
                user: data
                    .user_service
                    .find_by_external_id(&user_id, false)
                    .await
                    .map_internal_error(Some("Failed to find user"))?
                    .ok_or(HttpResponseError::unauthorized(Some("User not found")))?,
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
            let data: &web::Data<AppState> =
                req.app_data()
                    .ok_or(HttpResponseError::internal_error(Some(
                        "App data not found",
                    )))?;
            let jwt = JwtMiddleware::from_request(&req, &mut Payload::None).await?;

            Ok(JwtClientClaims {
                client: data
                    .client_service
                    .find_by_id(&jwt.id, false)
                    .await
                    .map_internal_error(Some("Failed to find client"))?
                    .ok_or(HttpResponseError::unauthorized(Some("Client not found")))?,
            })
        })
    }
}
