use crate::config::app_state::AppState;
use crate::entities::signing_request;
use crate::errors::http_response_error::{HttpResponseError, MapHttpResponseError};
use crate::mk_certs::mk_ca_signed_cert;
use crate::models::signing_request::SigningRequest;
use crate::util::traits::u8_vec_to_string::U8VecToString;
use crate::{mk_certs};
use actix_web::{web, Error, HttpMessage, HttpRequest};
use log::info;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::x509::{X509Req, X509};
use paperclip::actix::web::Json;
use paperclip::actix::{api_v2_operation, get, post};
use sea_orm::{ActiveModelTrait, ActiveValue};
use std::io;
use std::sync::Mutex;
use crate::middlewares::jwt_middleware;

static CA_CERT: Mutex<Option<(X509, PKey<Private>)>> = Mutex::new(None);

fn get_ca_cert() -> io::Result<(X509, PKey<Private>)> {
    let mut ca_cert = CA_CERT.lock().unwrap();
    if ca_cert.is_none() {
        info!("Generating CA cert");
        *ca_cert = Some(mk_certs::mk_ca_cert()?);
    } else {
        info!("Using cached CA cert");
    }

    Ok(ca_cert.as_ref().unwrap().clone())
}

#[api_v2_operation(
    tags(Certificates),
    summary = "Get the CA certificate",
    consumes = "text/plain",
    produces = "text/plain"
)]
#[get("/ca-certificate")]
async fn ca_certificate() -> Result<String, HttpResponseError> {
    let ca_cert = get_ca_cert().map_internal_error(None)?;
    Ok(ca_cert.0.to_pem().map_internal_error(None)?.to_string())
}

#[api_v2_operation(
    tags(Certificates),
    summary = "Sign a certificate request",
    consumes = "application/json",
    produces = "text/plain"
)]
#[post("/sign")]
async fn sign(
    request: Json<SigningRequest>,
    http_request: HttpRequest,
    data: web::Data<AppState>,
    //middleware: JwtMiddleware,
) -> Result<String, HttpResponseError> {
    let req = X509Req::from_pem(request.cert.as_bytes()).map_internal_error(None)?;
    let ca_cert = get_ca_cert().map_internal_error(None)?;

    let signed = mk_ca_signed_cert(&req, &ca_cert.0, &ca_cert.1).map_internal_error(None)?;
    let client_id = *http_request.extensions().get().ok_or_else(|| {
        HttpResponseError::internal_error(Some("Failed to retrieve client id".into()))
    })?;

    signing_request::ActiveModel {
        id: ActiveValue::NotSet,
        client_id: ActiveValue::Set(client_id),
        hash: ActiveValue::Set(
            signed
                .digest(MessageDigest::sha256())
                .map_internal_error(None)?
                .to_vec()
                .to_hex_string(":"),
        ),
        issued_at: ActiveValue::Set(chrono::Utc::now().into()),
    }
    .save(&data.db)
    .await
    .map_internal_error(None)?;

    Ok(signed.to_pem().map_internal_error(None)?.to_string())
}

pub fn module<T>(app: paperclip::actix::App<T>) -> paperclip::actix::App<T>
where
    T: actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Error = Error,
        InitError = (),
    >,
{
    let scope = paperclip::actix::web::scope("/api/v1/certificates")
        .wrap(jwt_middleware::Jwt)
        .service(ca_certificate)
        .service(sign);

    app.service(scope)
}

//register_module!("/api/v1/certificates", ca_certificate, sign);
