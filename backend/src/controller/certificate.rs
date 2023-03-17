use crate::config::app_state::AppState;
use crate::entity::signing_request;
use crate::error::http_response_error::{HttpResponseError, MapHttpResponseError};
use crate::middleware::extractors::JwtClientClaims;
use crate::mk_certs::mk_ca_signed_cert;
use crate::util::traits::from_model::FromModel;
use crate::util::types::WebResult;
use crate::{mk_certs, register_module};
use actix_web::web::Json;
use actix_web::{get, post, web};
use log::info;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::x509::{X509Req, X509};
use sea_orm::{ActiveValue, TryIntoModel};
use shared::model::new_signing_request_dto::NewSigningRequestDto;
use shared::model::signing_request_dto::SigningRequestDto;
use shared::util::traits::u8_vec_to_string::U8VecToString;
use std::io;
use std::sync::Mutex;

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

/// Get the server's CA certificate
#[utoipa::path(
    get,
    context_path = "/api/v1/certificate",
    tag = "Certificates",
    responses(
        (status = 200, description = "Ok", body = String),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
)]
#[get("/ca")]
async fn ca_certificate() -> Result<String, HttpResponseError> {
    let ca_cert = get_ca_cert().map_internal_error(None)?;
    Ok(ca_cert.0.to_pem().map_internal_error(None)?.to_string())
}

/// Sign a certificate signing request
/// using the server's CA certificate
#[utoipa::path(
    post,
    context_path = "/api/v1/certificate",
    tag = "Certificates",
    request_body = NewSigningRequestDto,
    responses(
        (status = 200, description = "Ok", body = String),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
    security(
        ("jwt" = [])
    )
)]
#[post("/sign")]
async fn sign(
    request: Json<NewSigningRequestDto>,
    data: web::Data<AppState>,
    claims: JwtClientClaims,
) -> WebResult<Json<SigningRequestDto>> {
    let req = X509Req::from_pem(request.request.as_bytes()).map_internal_error(None)?;
    let ca_cert = get_ca_cert().map_internal_error(None)?;

    let signed = mk_ca_signed_cert(&req, &ca_cert.0, &ca_cert.1, &request.alternative_names)
        .map_internal_error(None)?;

    let req = data
        .signing_request_service
        .save(signing_request::ActiveModel {
            id: ActiveValue::NotSet,
            client_id: ActiveValue::Set(claims.client.id),
            hash: ActiveValue::Set(
                signed
                    .digest(MessageDigest::sha256())
                    .map_internal_error(None)?
                    .to_vec()
                    .to_hex_string(":"),
            ),
            subject_name: ActiveValue::Set(
                signed
                    .subject_name()
                    .entries_by_nid(openssl::nid::Nid::COMMONNAME)
                    .next()
                    .ok_or(HttpResponseError::bad_request(Some(
                        "No common name in subject name",
                    )))?
                    .data()
                    .as_utf8()
                    .map_internal_error(None)?
                    .to_string(),
            ),
            serial_number: ActiveValue::Set(
                signed
                    .serial_number()
                    .to_bn()
                    .map_internal_error(None)?
                    .to_hex_str()
                    .map_internal_error(None)?
                    .to_string(),
            ),
            issued_at: ActiveValue::Set(chrono::Utc::now().into()),
        })
        .await?;

    let mut dto = SigningRequestDto::from_model(
        req.try_into_model()
            .map_internal_error(Some("Failed to map model"))?,
    );
    dto.certificate = Some(
        signed
            .to_pem()
            .map_internal_error(Some("Failed to stringify certificate"))?
            .to_string(),
    );
    Ok(Json(dto))
}

register_module!("/certificate", ca_certificate, sign);
