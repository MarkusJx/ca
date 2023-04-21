use crate::config::app_state::AppState;
use crate::entity::{certificate, root_certificate, signing_request};
use crate::error::http_response_error::{HttpResponseError, MapHttpResponseError};
use crate::middleware::extractors::{JwtClientClaims, KeycloakUserClaims};
use crate::middleware::keycloak_middleware;
use crate::middleware::keycloak_roles::{AdminRole, NoRoles};
use crate::model::ca_certificate_dto::CACertificateDto;
use crate::model::generate_intermediate_dto::GenerateIntermediateDto;
use crate::register_module;
use crate::util::ca_certificate::CACertificate;
use crate::util::traits::from_model::FromModel;
use crate::util::types::WebResult;
use actix_web::web::{Data, Json};
use actix_web::{get, post};
use openssl::hash::MessageDigest;
use openssl::x509::X509Req;
use sea_orm::{ActiveValue, TryIntoModel};
use shared::model::new_signing_request_dto::NewSigningRequestDto;
use shared::model::signing_request_dto::SigningRequestDto;
use shared::util::traits::u8_vec_to_string::U8VecToString;
use uuid::Uuid;

/// Get the CA's intermediate certificate
/// This is the certificate that is used to sign the client certificates
#[utoipa::path(
    get,
    context_path = "/api/v1/certificate",
    tag = "Certificates",
    operation_id = "getCaCertificate",
    responses(
        (status = 200, description = "Ok", body = CACertificateDto),
        (status = 404, description = "Intermediate certificate does not exist"),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
)]
#[get("/intermediate")]
async fn get_intermediate(data: Data<AppState>) -> WebResult<Option<Json<CACertificateDto>>> {
    Ok(data
        .certificate_service
        .find_active()
        .await?
        .map(|cert| Json(CACertificateDto::from_model(cert))))
}

#[utoipa::path(
    post,
    context_path = "/api/v1/certificate",
    tag = "Certificates",
    operation_id = "generateIntermediate",
    request_body = GenerateIntermediateDto,
    responses(
        (status = 200, description = "Ok", body = CACertificateDto),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
    security(
        ("oauth2" = [])
    )
)]
#[post("/intermediate/generate", wrap = "keycloak_middleware::Keycloak")]
async fn generate_intermediate(
    data: Data<AppState>,
    body: Json<GenerateIntermediateDto>,
    _claims: KeycloakUserClaims<AdminRole>,
) -> WebResult<Json<CACertificateDto>> {
    let root = data.root_certificate_service.find_active().await?.ok_or(
        HttpResponseError::bad_request(Some("Root certificate does not exist")),
    )?;
    let root = CACertificate::root_from_pem(&root.public, body.root_certificate.as_bytes())
        .map_internal_error(Some("Failed to parse root certificate"))?;

    let intermediate = CACertificate::generate_intermediate(&data.config, &root)
        .map_internal_error(Some("Failed to generate intermediate certificate"))?;

    let model = data
        .certificate_service
        .insert(certificate::ActiveModel {
            public: ActiveValue::set(
                intermediate
                    .cert_as_pem()
                    .map_internal_error(Some("Failed to get intermediate certificate"))?,
            ),
            private: ActiveValue::set(Some(
                intermediate
                    .key_pair_as_pem()
                    .map_internal_error(Some("Failed to get intermediate key pair"))?,
            )),
            valid_until: ActiveValue::set(
                intermediate
                    .valid_until()
                    .map_internal_error(Some("Failed to get intermediate certificate validity"))?,
            ),
            ..Default::default()
        })
        .await?;

    Ok(Json(CACertificateDto::from_model(model)))
}

async fn sign_certificate(
    request: Json<NewSigningRequestDto>,
    data: Data<AppState>,
    client_id: Uuid,
) -> WebResult<Json<SigningRequestDto>> {
    let req = X509Req::from_pem(request.request.as_bytes()).map_internal_error(None)?;
    let ca_cert: CACertificate = data
        .certificate_service
        .find_active()
        .await?
        .ok_or(HttpResponseError::bad_request(Some(
            "No active CA certificate found",
        )))?
        .try_into()
        .map_internal_error(Some("Failed to map model"))?;

    let signed = ca_cert
        .sign_request(&req, &request.alternative_names, &data.config, false)
        .map_internal_error(None)?;

    let req = data
        .signing_request_service
        .save(signing_request::ActiveModel {
            id: ActiveValue::NotSet,
            client_id: ActiveValue::Set(client_id),
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

/// Sign a certificate signing request
/// using the server's CA certificate
#[utoipa::path(
    post,
    context_path = "/api/v1/certificate",
    tag = "Certificates",
    operation_id = "signCertificate",
    request_body = NewSigningRequestDto,
    responses(
        (status = 200, description = "Ok", body = SigningRequestDto),
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
    data: Data<AppState>,
    claims: JwtClientClaims,
) -> WebResult<Json<SigningRequestDto>> {
    sign_certificate(request, data, claims.client.id).await
}

#[utoipa::path(
    post,
    tag = "Users",
    context_path = "/api/v1",
    request_body = NewSigningRequestDto,
    operation_id = "userSignCertificate",
    responses(
        (status = 200, description = "Ok", body = SigningRequestDto),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 424, description = "Failed dependency", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
    security(
        ("oauth2" = [])
    )
)]
#[post("/user/sign", wrap = "keycloak_middleware::Keycloak")]
async fn user_sign(
    data: Data<AppState>,
    user: Json<NewSigningRequestDto>,
    claims: KeycloakUserClaims<NoRoles>,
) -> WebResult<Json<SigningRequestDto>> {
    let client = data
        .client_service
        .find_user_client(&claims.user.id)
        .await?
        .ok_or(HttpResponseError::bad_request(Some(
            "No client found for user",
        )))?;
    sign_certificate(user, data, client.id).await
}

/// Get the root CA certificate
/// Only returns the public key as the private key isn't stored
/// on the server
#[utoipa::path(
    get,
    context_path = "/api/v1/certificate",
    tag = "Certificates",
    operation_id = "getRootCertificate",
    responses(
        (status = 200, description = "Ok", body = CACertificateDto),
        (status = 404, description = "Root certificate does not exist"),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
)]
#[get("/root")]
async fn get_root_certificate(data: Data<AppState>) -> WebResult<Option<Json<CACertificateDto>>> {
    Ok(data
        .root_certificate_service
        .find_active()
        .await?
        .map(|cert| Json(CACertificateDto::from_root_model(cert, None))))
}

/// Generate a new root certificate
/// This will invalidate the old root certificate
/// and all certificates signed by it (not yet implemented)
#[utoipa::path(
    post,
    context_path = "/api/v1/certificate",
    tag = "Certificates",
    operation_id = "generateRootCertificate",
    responses(
        (status = 200, description = "Ok", body = CACertificateDto),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 401, description = "Unauthorized", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto),
    ),
    security(
        ("oauth2" = [])
    )
)]
#[post("/root/generate", wrap = "keycloak_middleware::Keycloak")]
async fn generate_root_certificate(
    data: Data<AppState>,
    claims: KeycloakUserClaims<AdminRole>,
) -> WebResult<Json<CACertificateDto>> {
    let root = CACertificate::generate_root(&data.config)
        .map_internal_error(Some("Failed to generate root certificate"))?;
    let valid_until = root
        .valid_until()
        .map_internal_error(Some("Failed to get valid until"))?;

    let model = data
        .root_certificate_service
        .insert(root_certificate::ActiveModel {
            valid_until: ActiveValue::Set(valid_until.clone()),
            public: ActiveValue::Set(
                root.cert_as_pem()
                    .map_internal_error(Some("Failed to get public key"))?,
            ),
            created_by: ActiveValue::Set(claims.user.id),
            ..Default::default()
        })
        .await?;

    Ok(Json(CACertificateDto::from_root_model(
        model,
        Some(
            root.key_pair_as_pem()
                .map_internal_error(Some("Failed to get key pair"))?,
        ),
    )))
}

register_module!(
    "/certificate",
    get_intermediate,
    generate_intermediate,
    sign,
    user_sign,
    generate_root_certificate,
    get_root_certificate
);
