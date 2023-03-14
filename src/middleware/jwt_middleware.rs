use crate::config::app_state::AppState;
use crate::error::http_response_error::{HttpResponseError, MapHttpResponseError};
use crate::model::token_claims::TokenClaims;
use crate::util::types::WebResult;
use actix_web::dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{http, web, Error, FromRequest, HttpMessage, HttpRequest};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::future::{ready, Ready};
use log::info;

pub struct Jwt;

impl<S, B> Transform<S, ServiceRequest> for Jwt
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = Middleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(Middleware { service }))
    }
}

pub struct JwtMiddleware {
    pub id: uuid::Uuid,
}

impl JwtMiddleware {
    fn new(req: &HttpRequest) -> Result<Self, Error> {
        let data = req.app_data::<web::Data<AppState>>().unwrap();

        let token: Option<WebResult<String>> = req
            .cookie("token")
            .map(|c| Ok(c.value().to_string()))
            .or_else(|| {
                req.headers().get(http::header::AUTHORIZATION).map(|h| {
                    let s = h.to_str().map_bad_request(Some("Invalid token provided"))?;
                    if !s.starts_with("Bearer ") {
                        return Err(HttpResponseError::bad_request(Some(
                            "Invalid token provided",
                        )));
                    }

                    Ok(s.get(7..)
                        .ok_or(HttpResponseError::bad_request(Some(
                            "Invalid token provided",
                        )))?
                        .to_string())
                })
            });

        if token.is_none() {
            return Err(HttpResponseError::unauthorized(Some("No token provided")).into());
        }

        let claims = match decode::<TokenClaims>(
            &token.unwrap()?,
            &DecodingKey::from_secret(data.config.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(e) => {
                return Err(e).map_unauthorized(Some("Invalid token"))?;
            }
        };

        let id = uuid::Uuid::parse_str(claims.sub.as_str()).unwrap();
        req.extensions_mut().insert::<uuid::Uuid>(id.to_owned());

        Ok(JwtMiddleware { id })
    }
}

impl FromRequest for JwtMiddleware {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(Self::new(req))
    }
}

pub struct Middleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for Middleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match JwtMiddleware::new(req.request()) {
            Ok(_) => {
                let fut = self.service.call(req);
                Box::pin(async move { fut.await })
            }
            Err(err) => Box::pin(async move { Err(err) }),
        }
    }
}
