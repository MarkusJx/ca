use actix_web::body::EitherBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use actix_web_middleware_keycloak_auth::{
    AlwaysReturnPolicy, KeycloakAuth, KeycloakAuthMiddleware,
};
use futures_util::future::Ready;
use jsonwebtoken::DecodingKey;
use shared::util::types::BasicResult;
use std::sync::Mutex;

static mut KEYCLOAK_AUTH: Mutex<Option<KeycloakAuth<AlwaysReturnPolicy>>> = Mutex::new(None);

#[allow(dead_code)]
pub enum KeyType {
    RSA,
    ECDSA,
}

pub fn set_keycloak_public_key(cert: String, key_type: KeyType) -> BasicResult<()> {
    let key = match key_type {
        KeyType::RSA => DecodingKey::from_rsa_pem(cert.as_bytes())?,
        KeyType::ECDSA => DecodingKey::from_ec_pem(cert.as_bytes())?,
    };

    unsafe { KEYCLOAK_AUTH.lock() }
        .unwrap()
        .replace(KeycloakAuth::default_with_pk(key));
    Ok(())
}

pub struct Keycloak;

impl<S, B> Transform<S, ServiceRequest> for Keycloak
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = KeycloakAuthMiddleware<AlwaysReturnPolicy, S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        unsafe { KEYCLOAK_AUTH.lock() }
            .unwrap()
            .clone()
            .unwrap()
            .new_transform(service)
    }
}
